use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{BufReader, Result, ErrorKind};

use super::{Value, Decoder};

pub struct Connection {
    stream: BufReader<TcpStream>,
    decoder: Decoder,
}

impl Connection {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp = try!(TcpStream::connect(addr));
        Ok(Connection {
            stream: BufReader::new(tcp),
            decoder: Decoder::new(),
        })
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        let stream = self.stream.get_mut() as &mut Write;
        stream.write_all(buf)
    }

    pub fn read(&mut self) -> Result<Value> {
        if let Some(value) = self.decoder.read() {
            return Ok(value);
        }
        loop {
            let consumed_len = {
                let buffer = match self.stream.fill_buf() {
                    Ok(buf) => buf,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
                    Err(err) => return Err(err),
                };

                if buffer.len() == 0 {
                    continue;
                }
                try!(self.decoder.feed(&buffer));
                buffer.len()
            };

            self.stream.consume(consumed_len);
            if let Some(value) = self.decoder.read() {
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
        let mut connection = Connection::new("127.0.0.1:7711").unwrap();
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
