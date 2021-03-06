use std::io::{Result, Error, ErrorKind};
use std::net::{ToSocketAddrs};

use super::{Value, encode_slice};
use super::connection::{Connection};

pub fn create_client(hostname: &str, port: u16, password: &str) -> Result<Client> {
    let mut client = try!(Client::new((hostname, port)));
    try!(client.init(password));
    Ok(client)
}

pub struct Client {
    conn: Connection,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(addrs: A) -> Result<Self> {
        Ok(Client {
            conn: try!(Connection::new(addrs)),
        })
    }

    pub fn cmd(&mut self, slice: &[&str]) -> Result<Value> {
        let buf = encode_slice(slice);
        self.conn.write(&buf).unwrap();
        self.conn.read()
    }

    pub fn read_more(&mut self) -> Result<Value> {
        self.conn.read()
    }

    fn init(&mut self, password: &str) -> Result<()> {
        if password.len() > 0 {
            if let Value::Error(err)  = try!(self.cmd(&["auth", password])) {
                return Err(Error::new(ErrorKind::PermissionDenied, err));
            }

        }
        Ok(())
    }
}
