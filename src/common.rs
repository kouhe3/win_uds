use std::io;

use windows::Win32::{
    Foundation::{NO_ERROR, WIN32_ERROR},
    Networking::WinSock::{self, WSADATA, WSAGetLastError},
};

pub fn startup() -> io::Result<()> {
    let err = WIN32_ERROR(unsafe { WinSock::WSAStartup(0x202, &mut WSADATA::default()) } as _);
    if err != NO_ERROR {
        Err(io::Error::other(format!("WSAStartup failed: {:?}", err)))
    } else {
        Ok(())
    }
}
pub fn wsa_error() -> io::Error {
    use windows::Win32::Networking::WinSock::{
        WSAEACCES, WSAEADDRINUSE, WSAEADDRNOTAVAIL, WSAEAFNOSUPPORT, WSAECONNABORTED,
        WSAECONNREFUSED, WSAECONNRESET, WSAEHOSTUNREACH, WSAEINPROGRESS, WSAEINVAL,
        WSAEINVALIDPROCTABLE, WSAEINVALIDPROVIDER, WSAEISCONN, WSAEMFILE, WSAEMSGSIZE, WSAENETDOWN,
        WSAENETUNREACH, WSAENOBUFS, WSAENOTCONN, WSAEPROTONOSUPPORT, WSAEPROTOTYPE,
        WSAEPROVIDERFAILEDINIT, WSAESHUTDOWN, WSAESOCKTNOSUPPORT, WSAETIMEDOUT, WSANOTINITIALISED,
    };
    let err = unsafe { WSAGetLastError() };
    let kind = match err {
        WSANOTINITIALISED => io::ErrorKind::NotConnected,
        WSAENETDOWN => io::ErrorKind::ConnectionReset,
        WSAEAFNOSUPPORT => io::ErrorKind::Unsupported,
        WSAEINPROGRESS => io::ErrorKind::WouldBlock,
        WSAEMFILE => io::ErrorKind::ResourceBusy,
        WSAEINVAL => io::ErrorKind::InvalidInput,
        WSAEINVALIDPROVIDER | WSAEINVALIDPROCTABLE | WSAEPROVIDERFAILEDINIT => {
            io::ErrorKind::InvalidData
        }
        WSAENOBUFS => io::ErrorKind::OutOfMemory,
        WSAEPROTONOSUPPORT | WSAEPROTOTYPE | WSAESOCKTNOSUPPORT => io::ErrorKind::Unsupported,
        WSAECONNREFUSED => io::ErrorKind::ConnectionRefused,
        WSAETIMEDOUT => io::ErrorKind::TimedOut,
        WSAECONNABORTED => io::ErrorKind::ConnectionAborted,
        WSAECONNRESET => io::ErrorKind::ConnectionReset,
        WSAEADDRINUSE => io::ErrorKind::AddrInUse,
        WSAEADDRNOTAVAIL => io::ErrorKind::AddrNotAvailable,
        WSAEACCES => io::ErrorKind::PermissionDenied,
        WSAEISCONN => io::ErrorKind::AlreadyExists,
        WSAENOTCONN => io::ErrorKind::NotConnected,
        WSAESHUTDOWN => io::ErrorKind::BrokenPipe,
        WSAEMSGSIZE => io::ErrorKind::InvalidInput,
        WSAEHOSTUNREACH | WSAENETUNREACH => io::ErrorKind::HostUnreachable,

        _ => io::ErrorKind::Other,
    };
    let description = match err {
        WSANOTINITIALISED => "Successful WSAStartup call must occur before using this function",
        WSAENETDOWN => "The network subsystem has failed",
        WSAEAFNOSUPPORT => "The specified address family is not supported",
        WSAEINPROGRESS => "A blocking Windows Sockets call is in progress",
        WSAEMFILE => "No more socket descriptors are available",
        WSAEINVAL => "An invalid argument was supplied",
        WSAEINVALIDPROVIDER => "The service provider returned a version other than 2.2",
        WSAEINVALIDPROCTABLE => "The service provider returned an invalid procedure table",
        WSAENOBUFS => "No buffer space is available",
        WSAEPROTONOSUPPORT => "The specified protocol is not supported",
        WSAEPROTOTYPE => "The specified protocol is the wrong type for this socket",
        WSAEPROVIDERFAILEDINIT => "The service provider failed to initialize",
        WSAESOCKTNOSUPPORT => "The specified socket type is not supported in this address family",
        WSAECONNREFUSED => "Connection refused",
        WSAETIMEDOUT => "Connection timed out",
        WSAECONNABORTED => "Connection aborted",
        WSAECONNRESET => "Connection reset by peer",
        WSAEADDRINUSE => "Address already in use",
        WSAEADDRNOTAVAIL => "Address not available",
        WSAEACCES => "Permission denied",
        WSAEISCONN => "Socket is already connected",
        WSAENOTCONN => "Socket is not connected",
        WSAESHUTDOWN => "Socket has been shut down",
        WSAEMSGSIZE => "Message too long",
        WSAEHOSTUNREACH => "Host is unreachable",
        WSAENETUNREACH => "Network is unreachable",
        _ => "Windows Sockets error",
    };
    io::Error::new(kind, format!("{} (error code: {:?})", description, err))
}
