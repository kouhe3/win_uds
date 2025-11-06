use std::{io, path::Path};

use windows::Win32::Networking::WinSock::{self, SOCKADDR_UN, SOCKET_ERROR, SOMAXCONN};

use crate::{
    common::{startup, wsa_error},
    net::{Socket, SocketAddr, UnixStream, socketaddr_un},
};
pub struct UnixListener(Socket);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = Socket::new()?;
            let (addr, len) = socketaddr_un(path.as_ref())?;
            if WinSock::bind(s.0, &addr as *const _ as *const _, len) == SOCKET_ERROR {
                Err(wsa_error())
            } else {
                match WinSock::listen(s.0, SOMAXCONN as _) {
                    SOCKET_ERROR => Err(wsa_error()),
                    _ => Ok(Self(s)),
                }
            }
        }
    }
    pub fn bind_addr(socket_addr: &SocketAddr) -> io::Result<Self> {
        unsafe {
            let s = Socket::new()?;
            if WinSock::bind(
                s.0,
                &socket_addr.addr as *const _ as *const _,
                socket_addr.addrlen,
            ) == SOCKET_ERROR
            {
                Err(wsa_error())
            } else {
                match WinSock::listen(s.0, SOMAXCONN as _) {
                    SOCKET_ERROR => Err(wsa_error()),
                    _ => Ok(Self(s)),
                }
            }
        }
    }
    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        let mut addr = SOCKADDR_UN::default();
        let mut addrlen = size_of::<SOCKADDR_UN>() as _;
        let s = self.0.accept(
            Some(&mut addr as *mut _ as *mut _),
            Some(&mut addrlen as *mut _),
        )?;
        Ok((UnixStream(s), SocketAddr { addr, addrlen }))
    }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }
}
