use crate::net::{SockAddr, Socket};
use socket2::{Domain, Type};
use std::ops::{Deref, DerefMut};
use std::os::windows::io::{AsRawSocket, AsSocket, IntoRawSocket};
use std::{io, path::Path};

pub struct UnixStream(pub Socket);
impl UnixStream {
    /// Connects to the socket named by `path`.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// let socket = match UnixStream::connect("/tmp/sock") {
    ///     Ok(sock) => sock,
    ///     Err(e) => {
    ///         println!("Couldn't connect: {:?}", e);
    ///         return
    ///     }
    /// };
    /// ```
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let addr = SockAddr::unix(path)?;
        Self::connect_addr(&addr)
    }
    pub fn connect_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let s = Socket::new(Domain::UNIX, Type::STREAM, None)?;
        s.connect(socket_addr)?;
        Ok(Self(s))
    }
}
impl Deref for UnixStream {
    type Target = Socket;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for UnixStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl io::Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Write::write(&mut **self, buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        io::Write::flush(&mut **self)
    }
}
impl io::Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::Read::read(&mut **self, buf)
    }
}
impl AsSocket for UnixStream {
    fn as_socket(&self) -> std::os::windows::prelude::BorrowedSocket<'_> {
        self.0.as_socket()
    }
}
impl AsRawSocket for UnixStream {
    fn as_raw_socket(&self) -> std::os::windows::prelude::RawSocket {
        self.0.as_raw_socket()
    }
}
impl IntoRawSocket for UnixStream {
    fn into_raw_socket(self) -> std::os::windows::prelude::RawSocket {
        self.0.into_raw_socket()
    }
}
