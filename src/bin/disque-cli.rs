#[macro_use]
extern crate clap;
extern crate disque_cli;

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use clap::{Arg, App};

use disque_cli::{create_client};

fn main() {
    let matches = App::new("disque-cli")
        .version("0.1.0")
        .author("Qing Yan <admin@zensh.com>")
        .arg(Arg::with_name("hostname")
            .short("h")
            .long("hostname")
            .help("Server hostname (default: 127.0.0.1).")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Server port (default: 7711).")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("a")
            .long("password")
            .help("Password to use when connecting to the server.")
            .required(false)
            .takes_value(true))
        .get_matches();

    let mut port: u16 = 7711;
    let mut password = "";
    let mut hostname = "127.0.0.1";

    if let Some(_port) = matches.value_of("port") {
        port = u16::from_str(_port).expect("Failed to read port");
    }
    if let Some(_password) = matches.value_of("password") {
        password = _password;
    }
    if let Some(_hostname) = matches.value_of("hostname") {
        hostname = _hostname;
    }

    let mut client = create_client(hostname, port, password).expect("Failed to connect");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read command");

        let commands: Vec<&str> = input.split_whitespace().collect();
        if commands.len() == 0 {
            continue;
        }
        let command: &str = &commands[0].to_uppercase();
        match client.cmd(&commands) {
            Ok(value) => {
                let mut reply = value.to_beautify_string();
                match command {
                    "INFO" => {
                        // remove first and last '"'
                        reply.remove(0);
                        reply.pop();
                        writeln!(stdout, "{}", reply.trim()).unwrap();
                    }

                    "MONITOR" => {
                        writeln!(stdout, "{}", "Reading messages... (press Ctrl-C to quit)").unwrap();
                        writeln!(stdout, "{}", reply).unwrap();

                        loop {
                            let reply = client.read_more().unwrap().to_beautify_string();
                            writeln!(stdout, "{}", reply).unwrap();
                        }
                    }

                    _ => {
                        writeln!(stdout, "{}", reply).unwrap();
                    }
                }
            }

            Err(err) => {
                writeln!(stderr, "{:?}", err).unwrap();
            }
        };

    }
}
