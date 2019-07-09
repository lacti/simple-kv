#[cfg(feature = "tokio_net")]
extern crate futures;
extern crate hashbrown;
#[cfg(feature = "mio_net")]
extern crate mio;
#[cfg(feature = "tokio_net")]
extern crate tokio_core;
#[cfg(feature = "tokio_net")]
extern crate tokio_io;

pub trait Server: Sized {
    type Err;
    fn new(addr: ::std::net::SocketAddr) -> Result<Self, Self::Err>;
    fn start(&mut self) -> Result<(), Self::Err>;
}

mod process;
mod request;
mod response;

pub mod net;
mod storage;
