use std::fs::{read_to_string, write};
use std::io::prelude::*;

mod token;
mod ast;

type Num = i128;

const INPUT: &'static str = "in.txt";
const OUTPUT: &'static str = "out.txt";

fn read_input() -> String {
    read_to_string(INPUT).expect("something went wrong reading the file")
}

fn write_output(str: &str) {
    write(OUTPUT, str).unwrap()
}

fn main() {
    let test = read_input();
    let mut output = test.lines()
        .map(token::tokenize)
        .map(ast::parse_tokens)
        .map(|i| i.evaluate())
        .fold(String::new(), |a, i| a + &i.to_string() + "\n");
    write_output(&output);
}
