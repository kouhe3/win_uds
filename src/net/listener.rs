use std::{io, path::Path};

use socket2::{Domain, Type};
use crate::net::{SockAddr, Socket, UnixStream};
pub struct UnixListener(Socket);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        //startup()?;
        let addr = SockAddr::unix(path)?;
        Self::bind_addr(&addr)
    }
    pub fn bind_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let s = Socket::new(Domain::UNIX, Type::STREAM, None)?;
        s.bind(socket_addr)?;
        s.listen(5)?;
        Ok(Self(s))
    }
    pub fn accept(&self) -> io::Result<(UnixStream, SockAddr)> {
        let (s, addr) = self.0.accept()?;
        Ok((UnixStream(s), addr))
    }
}
