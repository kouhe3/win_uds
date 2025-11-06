use crate::common::*;
use crate::net::{Socket, socketaddr_un};
use std::{io, path::Path};
use windows::Win32::Networking::WinSock::{self, SOCKADDR_UN, SOCKET_ERROR};

pub struct UnixStream(pub Socket);
impl UnixStream {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = Socket::new()?;
            let addr = socketaddr_un(path.as_ref())?;
            let err = WinSock::connect(
                s.0,
                &addr as *const _ as *const _,
                size_of::<SOCKADDR_UN>() as _,
            );
            if err == SOCKET_ERROR {
                Err(wsa_error())
            } else {
                Ok(Self(s))
            }
        }
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
