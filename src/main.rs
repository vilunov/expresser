use std::fs::{read_to_string, write};

mod token;
mod ast;

type Num = i128;

const INPUT: &'static str = "in.txt";
const OUTPUT: &'static str = "out.txt";

fn main() {
    let test =
        read_to_string(INPUT).expect("something went wrong reading the file");
    let output = test.lines()
        .map(|i| ast::parse_tokens(token::tokenize(i)).evaluate())
        .fold(String::new(), |a, i| a + &i.to_string() + "\n");
    write(OUTPUT, map).unwrap()
}

/// Tests for the whole pipeline from
#[cfg(test)]
mod tests {
    use super::{execute, Num};
    const TESTS: &[(&'static str, Num)] = &[
        ("1>0", 1),
        ("1<0", 0),
        ("1=1", 1),
        ("1=0", 0),
        ("1+2", 3),
        ("1+2*3", 7),
        ("(1+2)*3", 9),
    ];

    #[test]
    fn test() {
        for (input, output) in TESTS {
            println!("Evaluating {}", *input);
            assert_eq!(*output, execute(*input));
        }
    }
}