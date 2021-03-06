use crate::paco::{character, digits, sequence, ParseState, Parser, Progress};
use std::env::args;
use std::fs;
use std::vec::Vec;

pub mod paco;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Ok(());
    }
    let buffer: String = fs::read_to_string(args[1].clone())?;
    let ps: ParseState<char> = ParseState::new(&buffer);

    let language_parser = sequence!(digits, character('Z'));
    match language_parser(ps) {
        Progress::Parsed(_, v) => println!("found statement {:?}", v),
        Progress::Failed => println!("parse error!"),
    }
    Ok(())
}
