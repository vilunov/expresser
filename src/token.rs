use super::Num;

/// Any non-numeric and non-whitespace characters,
/// currently limited to operators and parentheses
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Symbol {
    Plus, Minus,
    Asterisk,
    LessThan, BiggerThan, Equal,
    LeftParenthesis, RightParenthesis,
}

impl Symbol {
    fn parse_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Symbol::Plus),
            '-' => Some(Symbol::Minus),
            '*' => Some(Symbol::Asterisk),
            '>' => Some(Symbol::BiggerThan),
            '<' => Some(Symbol::LessThan),
            '=' => Some(Symbol::Equal),
            '(' => Some(Symbol::LeftParenthesis),
            ')' => Some(Symbol::RightParenthesis),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Token {
    Op(Symbol),
    Number(Num),
    Whitespace(char),
}

/// Wrapper for the vector of tokens providing stream-like API
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    /// Moves the cursor one token ahead
    pub fn advance(&mut self) {
        self.pos += 1;
    }

    /// Looks up the token under the cursor,
    /// returns None if the stream has finished
    pub fn read(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos])
        } else {
            None
        }
    }

    /// Creates a new stream from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, pos: 0 }
    }
}


/// Transforms the input string into a vector of tokens
///
/// # Panics
///
/// Panics on encountering incorrect characters or number literals
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut current_number: Option<Num> = None;
    let mut tokens = vec![];
    let mut i: u32 = 0;

    for c in input.chars() {
        match (c, c.to_digit(10), Symbol::parse_char(c), current_number) {
            // Handle whitespace chars
            (c, _, _, Some(num)) if c.is_whitespace() => {
                tokens.push(Token::Number(num));
                current_number = None;
                tokens.push(Token::Whitespace(c))
            }
            (c, _, _, None) if c.is_whitespace() =>
                tokens.push(Token::Whitespace(c)),

            // Handle digits
            (_, Some(digit), _, Some(num)) if num != 0 =>
                current_number = Some(num * 10 + digit as Num),
            (_, Some(digit), _, None) =>
                current_number = Some(digit as Num),

            // Handle operators
            (_, None, Some(op), Some(num)) => {
                tokens.push(Token::Number(num));
                current_number = None;
                tokens.push(Token::Op(op))
            },
            (_, None, Some(op), None) =>
                tokens.push(Token::Op(op)),

            // TODO error handling
            _ => panic!("Tokenization error at location {}", i)
        }
        i += 1;
    }
    if let Some(num) = current_number {
        tokens.push(Token::Number(num));
    }
    tokens
}

/// Unit tests for the tokenizer stage
#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::*;
    use super::Symbol::*;
    struct TestCase(&'static str, &'static [Token]);

    const TESTS_POSITIVE: &[TestCase] = &[
        TestCase("2+2",
                 &[Number(2), Op(Plus), Number(2)]),
        TestCase("2++2",
                 &[Number(2), Op(Plus), Op(Plus), Number(2)]),
        TestCase("",
                 &[]),
        TestCase("((2+555)+100)0",
                 &[Op(LeftParenthesis), Op(LeftParenthesis), Number(2), Op(Plus), Number(555),
                     Op(RightParenthesis), Op(Plus), Number(100), Op(RightParenthesis), Number(0)]),
        TestCase("2 * 10",
                 &[Number(2), Whitespace(' '), Op(Asterisk), Whitespace(' '), Number(10)]),
    ];

    #[test]
    fn positive() {
        for TestCase(input, output) in TESTS_POSITIVE {
            let tokens = tokenize(input);
            assert_eq!(tokens, *output);
            println!("{} converted to {:?} successfully", input, tokens);
        }
    }

    #[test]
    #[should_panic]
    fn negative_1() {
        tokenize("0001");
    }

    #[test]
    #[should_panic]
    fn negative_2() {
        tokenize("abc");
    }
}