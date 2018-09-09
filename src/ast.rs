use super::Num;
use token::{Token, TokenStream, Symbol};

/// The possible operators, each represents a function on two integers returning a new integer.
/// This is not the same as Symbol token, because it does not include the parentheses
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator {
    Summation, Subtraction, Multiplication,
    LessThanComparison, BiggerThanComparison, EqualityComparison
}

impl Operator {
    /// Applies the operator to two numbers and returns the result
    pub fn apply(&self, left: Num, right: Num) -> Num {
        use self::Operator::*;
        match *self {
            Summation => left + right,
            Subtraction => left - right,
            Multiplication => left * right,
            LessThanComparison => if left < right { 1 } else { 0 },
            BiggerThanComparison => if left > right { 1 } else { 0 },
            EqualityComparison => if left == right { 1 } else { 0 },
        }
    }

    /// Tries to transform the Symbol token into an Operator
    fn from_symbol(symbol: Symbol) -> Option<Self> {
        use self::Symbol::*;
        use self::Operator::*;
        match symbol {
            Plus => Some(Summation),
            Minus => Some(Subtraction),
            Asterisk => Some(Multiplication),
            LessThan => Some(LessThanComparison),
            BiggerThan => Some(BiggerThanComparison),
            Equal => Some(EqualityComparison),
            _ => None,
        }
    }
}

/// An AST node representing an expression
#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    /// A constant literal
    Const(Num),
    /// Operation on two subexpressions
    Action {
        left: Box<Expression>,
        action: Operator,
        right: Box<Expression>,
    },
}

impl Expression {
    /// Evaluates the expression and returns its value
    pub fn evaluate(&self) -> Num {
        use self::Expression::*;
        match *self {
            Const(val) => val,
            Action { ref left, action, ref right } =>
                action.apply(left.evaluate(), right.evaluate()),
        }
    }
}

/// Tries to read a primary from the token stream.
/// A primary is either a constant literal or a subexpression wrapped in parentheses.
///
/// # Panics
/// When the token stream is malformed and the parsed failed to extract a primary
fn read_primary(stream: &mut TokenStream) -> Expression {
    let token = stream.read().unwrap();
    match token {
        Token::Number(num) => {
            stream.advance();
            Expression::Const(num)
        },
        Token::Op(Symbol::LeftParenthesis) => {
            stream.advance();
            let expr = read_relation(stream);
            assert_eq!(stream.read().unwrap(), Token::Op(Symbol::RightParenthesis));
            stream.advance();
            expr
        }
        token => panic!("Unexpected token {:?}", token),
    }
}

/// Tries to read a relation from the token stream.
/// A relation is two subexpressions compared with each other by one of three comparison operators.
///
/// # Panics
/// When the token stream is malformed and the parsed failed to extract a relation
fn read_relation(tokens: &mut TokenStream) -> Expression {
    let mut expr = read_term(tokens);
    fn is_relation_symbol(s: &Token) -> bool {
        if let Token::Op(s) = *s {
            s == Symbol::LessThan || s == Symbol::BiggerThan || s == Symbol::Equal
        } else { false }
    }
    while let Some(Token::Op(s)) = tokens.read().filter(is_relation_symbol) {
        tokens.advance();
        expr = Expression::Action {
            left: Box::new(expr),
            action: Operator::from_symbol(s).unwrap(),
            right: Box::new(read_term(tokens)),
        };
    }
    expr
}

/// Tries to read a term from the token stream.
/// A term is a sum or a difference of two subexpressions.
///
/// # Panics
/// When the token stream is malformed and the parsed failed to extract a term
fn read_term(tokens: &mut TokenStream) -> Expression {
    let mut expr = read_factor(tokens);
    fn is_term_symbol(s: &Token) -> bool {
        if let Token::Op(s) = *s {
            s == Symbol::Plus || s == Symbol::Minus
        } else { false }
    }
    while let Some(Token::Op(s)) = tokens.read().filter(is_term_symbol) {
        tokens.advance();
        expr = Expression::Action {
            left: Box::new(expr),
            action: Operator::from_symbol(s).unwrap(),
            right: Box::new(read_factor(tokens)),
        };
    }
    expr
}

/// Tries to read a factor from the token stream.
/// A factor is a multiplication of two subexpressions.
///
/// # Panics
/// When the token stream is malformed and the parsed failed to extract a factor
fn read_factor(tokens: &mut TokenStream) -> Expression {
    let mut expr = read_primary(tokens);
    fn is_factor_symbol(s: &Token) -> bool {
        if let Token::Op(s) = *s {
            s == Symbol::Asterisk
        } else { false }
    }
    while let Some(Token::Op(s)) = tokens.read().filter(is_factor_symbol) {
        tokens.advance();
        expr = Expression::Action {
            left: Box::new(expr),
            action: Operator::from_symbol(s).unwrap(),
            right: Box::new(read_primary(tokens)),
        };
    }
    expr
}

/// Parses the input vector of tokens into an `Expression`
///
/// # Panics
/// When the token list is malformed
pub fn parse_tokens(tokens: Vec<Token>) -> Expression {
    let mut stream = TokenStream::new(tokens);
    let expr = read_relation(&mut stream);
    assert_eq!(stream.read(), None);
    expr
}

/// Unit tests for the AST stage
#[cfg(test)]
mod tests{
    use super::Token::*;
    use super::Symbol::*;
    use super::parse_tokens;

    #[test]
    fn test_simple_1() {
        let input = vec![Number(2), Op(Plus), Number(2)];
        let expr = parse_tokens(input);
        assert_eq!(expr.evaluate(), 4);
    }

    #[test]
    fn test_simple_2() {
        let input = vec![Number(2), Op(Plus), Number(2), Op(Asterisk), Number(2)];
        let expr = parse_tokens(input);
        assert_eq!(expr.evaluate(), 6);
    }

    #[test]
    fn test_simple_3() {
        let input =
            vec![Op(LeftParenthesis), Number(2), Op(Plus), Number(4), Op(RightParenthesis),
                 Op(Asterisk), Number(3)];
        let expr = parse_tokens(input);
        assert_eq!(expr.evaluate(), 18);
    }

    #[test]
    #[should_panic]
    fn test_fail_1() {
        let input =
            vec![Op(LeftParenthesis), Number(2), Op(Plus), Number(4), Op(LeftParenthesis),
                 Op(Asterisk), Number(3)];
        let _ = parse_tokens(input);
    }

    #[test]
    #[should_panic]
    fn test_fail_2() {
        let input =
            vec![Number(2), Op(Plus), Number(4), Number(4), Op(Asterisk), Number(3)];
        let _ = parse_tokens(input);
    }

    #[test]
    #[should_panic]
    fn test_fail_3() {
        let input =
            vec![Number(2), Op(Plus), Number(4), Number(4), Op(Asterisk), Op(Asterisk), Number(3)];
        let _ = parse_tokens(input);
    }
}