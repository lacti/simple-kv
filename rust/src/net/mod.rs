pub trait KVServer: Sized {
    type Err;
    fn new(addr: ::std::net::SocketAddr) -> Result<Self, Self::Err>;
    fn start(&mut self) -> Result<(), Self::Err>;
}
#[cfg(feature = "mio_net")]
pub mod mio;
#[cfg(feature = "std_net")]
pub mod std;
#[cfg(feature = "tokio_net")]
pub mod tokio;
