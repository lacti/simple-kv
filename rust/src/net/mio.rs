use super::KVServer;
use hashbrown::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Poll, PollOpt, Ready, Token};
use storage::{new_unsharable, UnSharableStorage};

use process::process;
use request::Request;

type Buffer = Vec<u8>;

pub struct SimpleKVServer {
    storage: UnSharableStorage,
    listener: TcpListener,
}

impl KVServer for SimpleKVServer {
    type Err = std::io::Error;

    fn new(addr: std::net::SocketAddr) -> Result<Self, std::io::Error> {
        Ok(Self {
            storage: new_unsharable(),
            listener: TcpListener::bind(&addr)?,
        })
    }

    fn start(&mut self) -> std::io::Result<()> {
        println!(
            "Server is listening on {}",
            self.listener
                .local_addr()
                .expect("Failed to get local addr")
                .port(),
        );

        let poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        let mut clients = HashMap::<usize, TcpStream>::new();
        let mut buffer = Buffer::with_capacity(4096);
        let mut idx = 1;

        // Register the socket with `Poll`
        poll.register(&self.listener, Token(0), Ready::readable(), PollOpt::edge())?;

        loop {
            poll.poll(&mut events, None)?;
            for event in &events {
                let readable = event.readiness().is_readable();

                match event.token().0 {
                    0 => {
                        if readable {
                            match self.listener.accept() {
                                Ok((stream, _addr)) => {
                                    if let Err(e) = stream.set_nodelay(true) {
                                        eprintln!("Failed to set nodelay {:?}", e);
                                        continue;
                                    }
                                    if let Err(e) = poll.register(
                                        &stream,
                                        Token(idx),
                                        Ready::readable(),
                                        PollOpt::level(),
                                    ) {
                                        eprintln!("Failed to register stream {:?}", e);
                                        continue;
                                    }

                                    clients.insert(idx, stream);
                                    idx += 1;
                                }
                                Err(e) => eprintln!("Failed to accept {:?}", e),
                            }
                        }
                    }
                    r => {
                        if readable
                            && clients
                                .get_mut(&r)
                                .and_then(|stream| {
                                    match Self::handle_connection(
                                        &mut buffer,
                                        &mut self.storage,
                                        stream,
                                    ) {
                                        Err(ref e)
                                            if e.kind() != std::io::ErrorKind::WouldBlock =>
                                        {
                                            eprintln!(
                                                "Error occured while handle connection {:?}",
                                                e
                                            );
                                            Some(())
                                        }
                                        Ok(true) => Some(()),
                                        _ => None,
                                    }
                                })
                                .is_some()
                        {
                            let stream = clients.remove(&r).unwrap();
                            if let Err(e) = poll.deregister(&stream) {
                                eprintln!("Failed to deregister client {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl SimpleKVServer {
    #[inline]
    fn handle_connection(
        buffer: &mut Buffer,
        storage: &mut UnSharableStorage,
        stream: &mut TcpStream,
    ) -> Result<bool, std::io::Error> {
        use std::io::{Read, Write};

        buffer.clear();

        match stream.read_to_end(buffer) {
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    return Err(e);
                }
            }
            Ok(len) => {
                if len == 0 {
                    return Ok(true);
                }
            }
        }

        let s = unsafe { std::str::from_utf8_unchecked(buffer.as_slice()) };
        for line in s.lines() {
            let request = Request::parse(line);
            if let Err(e) = request {
                eprintln!("{}", e);
                continue;
            }
            let request = request.unwrap();

            let response = process(storage, request);
            stream.write_fmt(format_args!("{}", response))?;
        }

        Ok(false)
    }
}
