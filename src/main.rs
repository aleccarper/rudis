use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use std::sync::RwLock;
mod database;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2435").unwrap();

    let database = database::Database::new();
    let arc_database = Arc::new(RwLock::new(database));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let database = Arc::clone(&arc_database);
        thread::spawn(move || handle_connection(stream, database));
    }
}

fn handle_connection(stream: TcpStream, database: Arc<RwLock<database::Database>>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut line = String::new();
        _ = reader.read_line(&mut line);
        match parse_command(line) {
            Command::Read(key) => {
                let response;
                match database.read().unwrap().read(key) {
                    Some(value) => response = value.clone(),
                    None => response = "not found".to_string(),
                }
                writer.write(response.as_bytes()).unwrap();
            }
            Command::Write(key, value) => {
                database.write().unwrap().insert(key, value.clone()) ;
                writer.write(value.as_bytes()).unwrap();
            }
            Command::Terminate => {
                writer.write("Command::Terminate".as_bytes()).unwrap();
            }
            Command::Ping => {
                writer.write("pong".as_bytes()).unwrap();
            }
            _ => {
                writer.write("Closing".as_bytes()).unwrap();
            }
        }
        writer.flush();
    }
}

enum Command {
    Read(String),
    Write(String, String),
    Terminate,
    Ping,
}

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("read")]
    Read,
    #[token("write")]
    Write,
    #[token("ping")]
    Ping,
    #[regex("[a-zA-Z]+")]
    Input,
    #[regex(r#"("[^']*")|:\w+"#)]
    Data,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn parse_command(input: String) -> Command {
    let mut lex = Token::lexer(input.as_str().trim());
    let command = lex.next();

    return match command {
        Some(Token::Read) =>  {
            lex.next();
            let key = lex.slice();
            Command::Read(key.to_string())
        },
        Some(Token::Write) =>  {
            lex.next();
            let key = lex.slice();
            lex.next();
            let value = lex.slice();
            println!("'{}', {:?}, key: {}, value: {}", input, lex, key, value);
            Command::Write(key.to_string(), value.to_string())
        },
        Some(Token::Ping) =>  Command::Ping,
        _ =>  Command::Terminate
    }
}


