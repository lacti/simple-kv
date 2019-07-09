use super::KVServer;
use storage::{new_sharable, SharableStorage};

use std::io::BufReader;

use futures::prelude::*;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::io::{lines, write_all};
use tokio_io::AsyncRead;

use process::process;
use request::Request;
use response::Response;

pub struct SimpleKVServer {
    core: Core,
    storage: SharableStorage,
    listener: Option<TcpListener>,
}

impl KVServer for SimpleKVServer {
    type Err = ::std::io::Error;

    fn new(addr: ::std::net::SocketAddr) -> Result<Self, std::io::Error> {
        let core = Core::new().unwrap();
        let handle = core.handle();
        Ok(Self {
            core: core,
            storage: new_sharable(),
            listener: Some(TcpListener::bind(&addr, &handle)?),
        })
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        println!(
            "Server is listening on {}",
            self.listener
                .as_ref()
                .unwrap()
                .local_addr()
                .expect("Failed to get local addr")
                .port(),
        );

        let listener = self.listener.take().unwrap();
        let storage = self.storage.clone();
        let handle = self.core.handle();
        let done = listener.incoming().for_each(move |(socket, _addr)| {
            let (reader, writer) = socket.split();

            let lines = lines(BufReader::new(reader));

            let storage = storage.clone();
            let responses = lines.map(move |line| {
                let request = Request::parse(&line);
                if let Err(e) = request {
                    eprintln!("{}", e);
                    return Response::None;
                }
                let request = request.unwrap();

                process(&mut storage.borrow_mut(), request)
            });

            let writes = responses.fold(writer, |writer, response| {
                let response = format!("{}", response);
                write_all(writer, response.into_bytes()).map(|(w, _)| w)
            });

            let msg = writes.then(move |_| Ok(()));
            handle.spawn(msg);
            Ok(())
        });

        self.core.run(done).unwrap();

        Ok(())
    }
}
