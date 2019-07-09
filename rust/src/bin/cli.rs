extern crate simple_kv;

#[cfg(feature = "mio_net")]
use simple_kv::net::mio::SimpleKVServer;
#[cfg(feature = "std_net")]
use simple_kv::net::std::SimpleKVServer;
#[cfg(feature = "tokio_net")]
use simple_kv::net::tokio::SimpleKVServer;
use simple_kv::net::KVServer;

fn main() {
    let mut server = SimpleKVServer::new("0.0.0.0:6378".parse().expect("Failed to parse addr"))
        .expect("Failed to start server");
    server.start().expect("Error occured while running server");
}
