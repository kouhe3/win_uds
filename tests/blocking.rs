use std::io::{ErrorKind, Read};
use win_uds::net::{UnixListener, UnixStream};

#[test]
fn no_blocking_test() {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_no_blocking.sock");
    let _ = std::fs::remove_file(&path);

    let l = UnixListener::bind(&path).unwrap();
    l.set_nonblocking(true).unwrap();
    let mut s = UnixStream::connect(&path).unwrap();
    s.set_nonblocking(true).unwrap();
    let mut buf = [0u8; 1024];
    assert_eq!(s.read(&mut buf).unwrap_err().kind(), ErrorKind::WouldBlock);
    let _ = std::fs::remove_file(&path);
}
