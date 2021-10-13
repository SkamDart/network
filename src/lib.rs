use anyhow::Result;

use std::io::Read;
use std::io::Write;

#[cxx::bridge]
pub mod tcp {

    extern "Rust" {
        /// TCP Stream
        ///
        /// This struct is shared between both languages but
        /// Rust can see the contents and it is opaque to C++.
        ///
        /// The contents of opaque types are hidden from the other langauge
        /// while the native language can inspect the contents.
        type TcpStream;

        fn connect(addr: String) -> Result<Box<TcpStream>>;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

        fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;

        fn read_to_string(&mut self, buf: &mut String) -> Result<usize>;

        fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

        fn write(&mut self, buf: &[u8]) -> Result<usize>;

        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    }
}

#[cxx::bridge]
pub mod udp {
    extern "Rust" {
        type UdpSocket;
    }
}

pub struct UdpSocket {
    socket: std::net::UdpSocket,
}

pub struct TcpStream {
    stream: std::net::TcpStream,
}

impl TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stream.read(buf).map_err(Into::into)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.stream.read_to_end(buf).map_err(Into::into)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        self.stream.read_to_string(buf).map_err(Into::into)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.stream.read_exact(buf).map_err(Into::into)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf).map_err(Into::into)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush().map_err(Into::into)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.stream.write_all(buf).map_err(Into::into)
    }
}

pub fn connect(addr: String) -> anyhow::Result<Box<TcpStream>> {
    let stream = std::net::TcpStream::connect(addr)?;
    Ok(Box::new(TcpStream { stream }))
}
