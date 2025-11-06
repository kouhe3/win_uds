use crate::common::*;
use crate::net::{Socket, SocketAddr, socketaddr_un};
use std::time::Duration;
use std::{io, path::Path};
use windows::Win32::Networking::WinSock::{self, SO_RCVTIMEO, SO_SNDTIMEO, SOCKET_ERROR};

pub struct UnixStream(pub Socket);
impl UnixStream {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = Socket::new()?;
            let (addr, len) = socketaddr_un(path.as_ref())?;
            match WinSock::connect(s.0, &addr as *const _ as *const _, len) {
                SOCKET_ERROR => Err(wsa_error()),
                _ => Ok(Self(s)),
            }
        }
    }
    pub fn connect_addr(socket_addr: &SocketAddr) -> io::Result<Self> {
        let s = Socket::new()?;
        match unsafe {
            WinSock::connect(
                s.0,
                &socket_addr.addr as *const _ as *const _,
                socket_addr.addrlen,
            )
        } {
            SOCKET_ERROR => Err(wsa_error()),
            _ => Ok(Self(s)),
        }
    }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0.peer_addr()
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0.take_error()
    }
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.0.set_nonblocking(nonblocking)
    }
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.0.set_timeout(timeout, SO_RCVTIMEO)
    }
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.0.set_timeout(timeout, SO_SNDTIMEO)
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.timeout(SO_RCVTIMEO)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.timeout(SO_SNDTIMEO)
    }
}

impl io::Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Write::write(&mut &*self, buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        io::Write::flush(&mut &*self)
    }
}
impl io::Write for &UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl io::Read for &UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf)
    }
}
impl io::Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::Read::read(&mut &*self, buf)
    }
}
