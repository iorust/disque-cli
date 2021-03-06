#![feature(test)]

extern crate test;
extern crate disque_cli;

use test::Bencher;
use disque_cli::{create_client, Client};

fn prepare_client() -> Client {
    create_client("127.0.0.1", 7711, "").expect("Failed to connect")
}

#[bench]
fn ping(b: &mut Bencher) {
    let mut client =  prepare_client();
    let command = ["ping"];
    b.iter(|| {
        for _ in 0..1000 {
            client.cmd(&command).unwrap();
        }
    });
}
