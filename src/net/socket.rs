use std::{
    io, mem,
    net::Shutdown,
    os::{
        raw::c_int,
        windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
    },
};

use windows::{
    Win32::Networking::WinSock::{
        self, AF_UNIX, INVALID_SOCKET, SEND_RECV_FLAGS, SO_ERROR, SOCK_STREAM, SOCKADDR, SOCKET,
        SOCKET_ERROR, SOL_SOCKET,
    },
    core::{PSTR, s},
};

use crate::{
    common::{startup, wsa_error},
    net::SocketAddr,
};
// wrap Winsock method like std
pub struct Socket(pub SOCKET);

impl Socket {
    pub fn new() -> io::Result<Self> {
        unsafe {
            startup()?;
            match WinSock::socket(AF_UNIX as _, SOCK_STREAM, 0)? {
                INVALID_SOCKET => Err(wsa_error()),
                s => Ok(Self(s)),
            }
        }
    }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::send(self.0, buf, SEND_RECV_FLAGS(0)) {
                SOCKET_ERROR => Err(wsa_error()),
                len => Ok(len as _),
            }
        }
    }
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::recv(self.0, buf, SEND_RECV_FLAGS(0)) {
                0 => Err(io::Error::other("Connection closed")),
                len if len > 0 => Ok(len as _),
                _ => Err(wsa_error()),
            }
        }
    }
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        use WinSock::{SD_BOTH, SD_RECEIVE, SD_SEND};
        let shutdown_how = match how {
            Shutdown::Read => SD_RECEIVE,
            Shutdown::Write => SD_SEND,
            Shutdown::Both => SD_BOTH,
        };
        unsafe {
            match WinSock::shutdown(self.0, shutdown_how) {
                0 => Ok(()),
                _ => Err(wsa_error()),
            }
        }
    }
    pub fn accept(
        &self,
        addr: Option<*mut SOCKADDR>,
        addrlen: Option<*mut i32>,
    ) -> io::Result<Socket> {
        unsafe {
            // or we should just use None None here because
            // seems like accept write nothing to addr and addrlen
            match WinSock::accept(self.0, addr, addrlen)? {
                INVALID_SOCKET => Err(wsa_error()),
                s => Ok(Socket(s)),
            }
        }
    }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        let mut addr = SocketAddr::default();
        match unsafe {
            WinSock::getsockname(
                self.0,
                &mut addr.addr as *mut _ as *mut _,
                &mut addr.addrlen as *mut _,
            )
        } {
            SOCKET_ERROR => Err(wsa_error()),
            _ => Ok(addr),
        }
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsafe {
            let mut val: c_int = 0;
            let mut len = size_of::<c_int>() as i32;
            match WinSock::getsockopt(
                self.0,
                SOL_SOCKET,
                SO_ERROR,
                PSTR::from_raw(&mut val as *mut _ as *mut _),
                &mut len as *mut _,
            ) {
                SOCKET_ERROR => Err(wsa_error()),
                _ => {
                    if val == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(wsa_error()))
                    }
                }
            }
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = unsafe { WinSock::closesocket(self.0) };
    }
}
impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        self.0.0 as RawSocket
    }
}

impl FromRawSocket for Socket {
    unsafe fn from_raw_socket(sock: RawSocket) -> Self {
        Socket(SOCKET(sock as _))
    }
}

impl IntoRawSocket for Socket {
    fn into_raw_socket(self) -> RawSocket {
        let ret = self.0.0 as RawSocket;
        mem::forget(self);
        ret
    }
}
