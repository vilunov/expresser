use std::fs::{read_to_string, write};

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

fn execute(expr: &str) -> Num {
    ast::parse_tokens(token::tokenize(expr)).evaluate()
}

fn main() {
    let test = read_input();
    let output = test.lines()
        .map(execute)
        .fold(String::new(), |a, i| a + &i.to_string() + "\n");
    write_output(&output);
}

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