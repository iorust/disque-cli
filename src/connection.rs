use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{BufReader, Result, ErrorKind};

use super::{Value, Decoder};

pub struct Connection {
    tcp: TcpStream,
    de: Decoder,
}

impl Connection {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Self {
        Connection {
            tcp: TcpStream::connect(addr).unwrap(),
            de: Decoder::new(),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.tcp.write_all(buf)
    }

    pub fn read(&mut self) -> Result<Value> {
        if let Some(value) = self.de.read() {
            return Ok(value);
        }
        let mut reader = BufReader::new(&mut self.tcp);
        loop {
            let consumed_len = {
                let buffer = match reader.fill_buf() {
                    Ok(buf) => buf,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
                    Err(err) => return Err(err),
                };

                if buffer.len() == 0 {
                    continue;
                }
                try!(self.de.feed(&buffer));
                buffer.len()
            };

            reader.consume(consumed_len);
            if let Some(value) = self.de.read() {
                return Ok(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Value, encode_slice};

    #[test]
    fn struct_connection() {
        let mut connection = Connection::new("127.0.0.1:7711");
        connection.write(&encode_slice(&["ping"])).unwrap();
        assert_eq!(connection.read().unwrap(), Value::String("PONG".to_string()));

        connection.write(&encode_slice(&["qlen", "rust_test"])).unwrap();
        assert_eq!(connection.read().unwrap(), Value::Integer(0));

        connection.write(&encode_slice(&["ping"])).unwrap();
        connection.write(&encode_slice(&["qlen", "rust_test"])).unwrap();
        assert_eq!(connection.read().unwrap(), Value::String("PONG".to_string()));
        assert_eq!(connection.read().unwrap(), Value::Integer(0));
    }
}
