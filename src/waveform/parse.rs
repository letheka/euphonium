/* This file incorporates source code copied and modified from "ullage"
(https://github.com/iwillspeak/ullage) by Will Speak.
The original license is reproduced below.

Copyright (c) 2016 Will Speak

Permission is hereby granted, free of charge, to any person obtaining a copy of this software
and associated documentation files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

pub use crate::error::{ParseError, ParseResult};
use std::iter::Peekable;

/// Represents an AST prefix operator.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrefixOp {
    Negate,
    Abs,
    Sgn,
    Sin,
    Cos,
}

/// Represents an AST infix operator
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Represents an AST expression.
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum Expression {
    Identifier(String),
    Number(f32),
    Prefix(PrefixOp, Box<Expression>),
    Infix(Box<Expression>, InfixOp, Box<Expression>),
}

impl Expression {
    /// # New Identifier Expression
    ///
    /// A reference to an identifier, either as a variable reference
    /// or declaration, part of a function definition or function call.
    pub fn identifier(s: String) -> Self {
        Expression::Identifier(s)
    }

    /// # New Numeric Constant
    ///
    /// A constant numeric value, either specified inline using a
    /// numeric literal or computed from other known compile-time
    /// constants.
    pub fn constant_num(n: f32) -> Self {
        Expression::Number(n)
    }

    /// # New Prefix Operator Expression
    ///
    /// Represents the application of a prefix unary operator to
    /// another expression.
    pub fn prefix(op: PrefixOp, expr: Expression) -> Self {
        Expression::Prefix(op, Box::new(expr))
    }

    /// # New Infix Operator Expression
    ///
    /// Represents the application of an infix binary operator to two
    /// expression operands.
    pub fn infix(lhs: Expression, op: InfixOp, rhs: Expression) -> Self {
        Expression::Infix(Box::new(lhs), op, Box::new(rhs))
    }
}

/// Parse a Single Expression
///
/// Runs the tokeniser and parser of the given input string, returning
/// the first expression parsed.
pub fn parse_single<S: AsRef<str>>(s: S) -> ParseResult<Expression> {
    let t = Tokeniser::new_from_str(s.as_ref());
    let mut p = Parser::new(t);
    p.single_expression()
}

/// This structure represents a single token from the input source
/// buffer.
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    /// Represents a string of alphabetic characters. This could be a
    /// language keyword or a variable or type identifier.
    Word(&'a str),
    /// Whitespace trivia
    Whitespace(&'a str),
    /// Constant numerical value.
    Number(f32),
    /// The `+` character
    Plus,
    /// The `-` character
    Minus,
    /// The `*` character
    Star,
    /// The `/` character
    Slash,
    /// The `(` character
    Percent,
    /// The `%` character
    OpenBracket,
    /// The `)` character
    CloseBracket,
    /// An unrecognised token
    Unknown(char),
}

/// Tokeniser
///
/// An object which can run a regex state machine over an input
/// buffer, producing tokens when each new lexeme is matched.
struct Tokeniser<'a> {
    buff: &'a str,
    idx: usize,
}

impl<'a> Tokeniser<'a> {
    /// Creates a new tokeniser from the given string slice.
    pub fn new_from_str(source: &'a str) -> Tokeniser {
        Tokeniser {
            buff: source,
            idx: 0,
        }
    }

    /// Retrieve the next 'raw' token. This is the next lexical match
    /// in the buffer, and may include whitespace and other trivia
    /// tokens.
    fn next_raw(&mut self) -> Option<Token<'a>> {
        let ts = self.idx;
        let mut te = ts;
        let mut chars = self.buff[ts..].chars();
        let tok = chars.next().and_then(|c| {
            te += c.len_utf8();
            match c {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Star),
                '/' => Some(Token::Slash),
                '%' => Some(Token::Percent),
                '(' => Some(Token::OpenBracket),
                ')' => Some(Token::CloseBracket),
                '#' => {
                    te += chars
                        .take_while(|c| *c != '\n')
                        .fold(0, |l, c| l + c.len_utf8());
                    Some(Token::Whitespace(&self.buff[ts..te]))
                }
                '0'...'9' => {
                    te += chars
                        .take_while(|c| (*c >= '0' && *c <= '9') || (*c == '.'))
                        .count();
                    let token_str = &self.buff[ts..te];
                    // we have cheked that it's a valid numeric literal,
                    // so unwrap is fine here.
                    Some(Token::Number(token_str.parse::<f32>().unwrap()))
                }
                c if c.is_alphabetic() || c == '_' => {
                    te += chars
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .fold(0, |l, c| l + c.len_utf8());
                    Some(Token::Word(&self.buff[ts..te]))
                }
                c if c.is_whitespace() => {
                    te += chars
                        .take_while(|c| c.is_whitespace())
                        .fold(0, |l, c| l + c.len_utf8());
                    Some(Token::Whitespace(&self.buff[ts..te]))
                }
                _ => Some(Token::Unknown(c)),
            }
        });
        self.idx = te;
        tok
    }
}

/// Tokeniser Iterator implementation.
///
/// This allows iterator adapters to be used with the token
/// stream. The next method filters out whitespace tokens from the
/// tokeniser too. This means the `Tokeniser` doesn't have to worry
/// about skipping certain lexemes in the grammar.
impl<'a> Iterator for Tokeniser<'a> {
    type Item = Token<'a>;

    /// Iterator next method. This method returns the next
    /// non-whitespace token in the `Tokeniser`'s stream of `Token`s.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tok) = self.next_raw() {
            if let Token::Whitespace(_) = tok {
            } else {
                return Some(tok);
            }
        }
        None
    }
}

/// Expression parser. Given a stream of tokens this will produce
/// an expression tree, or a parse error.
struct Parser<'a> {
    lexer: Peekable<Tokeniser<'a>>,
}

impl<'a> Parser<'a> {
    /// Create a new Parser from a given token stream.
    pub fn new(t: Tokeniser<'a>) -> Self {
        Parser {
            lexer: t.peekable(),
        }
    }

    /// Moves the token stream on by a single token, if the
    /// token's lexeme is of the given type.
    pub fn expect(&mut self, expected: Token) -> ParseResult<()> {
        match self.lexer.peek() {
            Some(token) if token == &expected => Ok(()),
            Some(_) => Err(ParseError::Unexpected),
            None => Err(ParseError::Incomplete),
        }
        .map(|ok| {
            self.lexer.next();
            ok
        })
    }

    // /// Attempt to parse an identifier
    // pub fn identifier(&mut self) -> ParseResult<String> {
    //     match self.expression(100)? {
    //         Expression::Identifier(string) => Ok(string),
    //         _ => Err(ParseError::Unexpected),
    //     }
    // }

    /// Attempt to parse a single expression
    ///
    /// Parses a expresison with the given precedence. To parse a
    /// single expression use `single_expression`.
    pub fn expression(&mut self, rbp: u32) -> ParseResult<Expression> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter_than(rbp) {
            left = self.parse_led(left)?;
        }
        Ok(left)
    }

    /// Attempt to Parse a Single Expression
    ///
    /// Used to parse 'top-level' expressions.
    pub fn single_expression(&mut self) -> ParseResult<Expression> {
        self.expression(0)
    }

    /// Returns true if the next token's lbp is > the given rbp
    fn next_binds_tighter_than(&mut self, rbp: u32) -> bool {
        self.lexer.peek().map_or(false, |t| t.lbp() > rbp)
    }

    /// Prefix Operator
    ///
    /// Parses the trailing expression for a prefix operator.
    fn prefix_op(&mut self, op: PrefixOp) -> ParseResult<Expression> {
        let rhs = self.expression(100)?;
        Ok(Expression::prefix(op, rhs))
    }

    /// Attempt to parse a single left denotation
    fn parse_led(&mut self, lhs: Expression) -> ParseResult<Expression> {
        self.lexer
            .next()
            .map_or(Err(ParseError::Incomplete), |t| t.led(self, lhs))
    }

    /// Attempt to parse a single null denotation
    fn parse_nud(&mut self) -> ParseResult<Expression> {
        self.lexer
            .next()
            .map_or(Err(ParseError::Incomplete), |t| t.nud(self))
    }
}

impl<'a> Token<'a> {
    /// Left binding power. This controls the precedence of
    /// the symbol when being parsed as an infix operator.
    ///
    /// Returns the associativity, or binding power, for the given
    /// token. This is used when deciding if to parse the `led()`
    /// of this token.
    fn lbp(&self) -> u32 {
        match *self {
            // Arithmetic operators
            Token::Plus | Token::Minus => 50,
            Token::Star | Token::Slash | Token::Percent => 60,
            // Grouping operators
            Token::OpenBracket => 80,
            _ => 0,
        }
    }

    /// Null denotation. This is the parse of the symbol when it
    /// doesn't have any expression to the left hand side of it.
    ///
    /// This is responsible for parsing literals and variable
    /// references into expressions, as well as parsing prefix
    /// expressions
    fn nud(&self, parser: &mut Parser) -> ParseResult<Expression> {
        match *self {
            Token::Word("abs") => parser.prefix_op(PrefixOp::Abs),
            Token::Word("sgn") => parser.prefix_op(PrefixOp::Sgn),
            Token::Word("sin") => parser.prefix_op(PrefixOp::Sin),
            Token::Word("cos") => parser.prefix_op(PrefixOp::Cos),
            Token::Word(word) => Ok(Expression::identifier(String::from(word))),
            Token::Number(n) => Ok(Expression::constant_num(n)),
            Token::Plus => parser.expression(100),
            Token::Minus => parser.prefix_op(PrefixOp::Negate),
            Token::OpenBracket => {
                let expr = parser.single_expression()?;
                parser.expect(Token::CloseBracket)?;
                Ok(expr)
            }
            _ => Err(ParseError::Unexpected),
        }
    }

    /// Left denotation. This is the parse of the symbol when it
    /// has an expression to the left hand side of it.
    ///
    /// This is responsible for parsing infix operators.
    fn led(&self, parser: &mut Parser, lhs: Expression) -> ParseResult<Expression> {
        match *self {
            // Binary infix operator
            Token::Plus => self.infix(parser, lhs, InfixOp::Add),
            Token::Minus => self.infix(parser, lhs, InfixOp::Sub),
            Token::Star => self.infix(parser, lhs, InfixOp::Mul),
            Token::Slash => self.infix(parser, lhs, InfixOp::Div),
            Token::Percent => self.infix(parser, lhs, InfixOp::Mod),
            _ => Err(ParseError::Incomplete),
        }
    }

    /// Attempt to Parse an Infix Expression
    fn infix(&self, parser: &mut Parser, lhs: Expression, op: InfixOp) -> ParseResult<Expression> {
        let rhs = parser.expression(self.lbp())?;
        Ok(Expression::infix(lhs, op, rhs))
    }
}
