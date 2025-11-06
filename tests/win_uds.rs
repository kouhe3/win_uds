use std::io::Read;
use std::io::Write;
use win_uds::net::*;
#[test]
fn win_uds_echo() {
    let tmp = std::env::temp_dir();
    let sock_path = tmp.join("test-uds-echo.sock");
    let _ = std::fs::remove_file(&sock_path);
    let listener = UnixListener::bind(&sock_path).unwrap();
    let srv = std::thread::spawn(move || {
        let (mut stream, addr) = listener.accept().expect("accept failed");
        let mut buf = [0u8; 128];
        let n = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(e) => panic!("read error: {}", e),
        };
        stream.write_all(&buf[..n]).expect("write_all failed");
    });

    let sock_path_clone = sock_path.clone();

    let cli = std::thread::spawn(move || {
        let mut stream = UnixStream::connect(&sock_path).unwrap();
        let req = b"Hello, world!";
        stream.write_all(req).unwrap();
        let mut resp = vec![0; req.len()];
        stream.read_exact(&mut resp).unwrap();
        assert_eq!(req, &resp[..]);
    });
    cli.join().unwrap();
    srv.join().unwrap();
    let _ = std::fs::remove_file(&sock_path_clone);
}
