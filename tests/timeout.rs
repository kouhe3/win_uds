use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use win_uds::net::{UnixListener, UnixStream};

#[test]
fn read_time_out() {
    let tmp = std::env::temp_dir();
    let path = tmp.join("read_time_out.sock");
    let _ = std::fs::remove_file(&path);

    let listener = UnixListener::bind(&path).unwrap();
    let srv = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let mut buf = [0u8; 128];
        assert!(stream.read(&mut buf).is_err());
    });
    let path_clone = path.clone();
    let cli = std::thread::spawn(move || {
        let mut _stream = UnixStream::connect(&path_clone).unwrap();
        thread::sleep(Duration::from_secs(3));
    });
    cli.join().unwrap();
    srv.join().unwrap();
    let _ = std::fs::remove_file(&path);
}

#[test]
fn write_time_out() {
    let tmp = std::env::temp_dir();
    let path = tmp.join("write_time_out.sock");
    let _ = std::fs::remove_file(&path);

    let listener = UnixListener::bind(&path).unwrap();
    let srv = std::thread::spawn(move || {
        let (mut _stream, _) = listener.accept().unwrap();
        std::thread::sleep(Duration::from_secs(2));
    });

    let path_clone = path.clone();
    let cli = std::thread::spawn(move || {
        let mut stream = UnixStream::connect(&path_clone).unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let mut buf = [0u8; 1];
        assert!(stream.read(&mut buf).is_err());
    });

    cli.join().unwrap();
    srv.join().unwrap();
    let _ = std::fs::remove_file(&path);
}
