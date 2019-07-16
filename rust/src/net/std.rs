use super::KVServer;
use process::process;
use request::Request;
use std::net::{TcpListener, TcpStream};
use storage::{new_unsharable, UnSharableStorage};

pub struct SimpleKVServer {
    storage: UnSharableStorage,
    listener: TcpListener,
}

impl KVServer for SimpleKVServer {
    type Err = std::io::Error;

    fn new(addr: ::std::net::SocketAddr) -> Result<Self, std::io::Error> {
        Ok(Self {
            storage: new_unsharable(),
            listener: TcpListener::bind(addr)?,
        })
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        println!(
            "Server is listening on {}",
            self.listener
                .local_addr()
                .expect("Failed to get local addr")
                .port(),
        );

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = Self::handle_connection(&mut self.storage, stream) {
                        eprintln!("Error occured while handle connection {:?}", e);
                    }
                }
                Err(e) => eprintln!("Failed to connect {:?}", e),
            }
        }

        Ok(())
    }
}

impl SimpleKVServer {
    #[inline]
    fn handle_connection(
        storage: &mut UnSharableStorage,
        mut stream: TcpStream,
    ) -> Result<(), std::io::Error> {
        use std::io::BufReader;
        use std::io::{BufRead, Write};

        stream.set_nodelay(true)?;
        let reader = BufReader::new(stream.try_clone()?);
        for line in reader.lines() {
            let line = line?;
            let request = Request::parse(line.as_str());
            if let Err(e) = request {
                eprintln!("{}", e);
                continue;
            }
            let request = request.unwrap();

            let response = process(storage, request);
            stream.write_fmt(format_args!("{}", response))?;
        }

        Ok(())
    }
}
