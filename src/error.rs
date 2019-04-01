/* This file incorporates source code copied and derived from "ullage"
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

/// Parser result type
///
/// Returned from parsing functions when success can't be guaranteed.
pub type EvalResult<T> = ::std::result::Result<T, EvalError>;

/// Parser error type
///
/// This distinguishes between the different
/// kinds of errors that the `Parser` can encounter.
///
/// TODO: Both variants of this type should have more data
/// attached. It would be nice to know _what_ token was unexpected or
/// what the incomplete expression could have continued with (for
/// error recovery). It probably makes sense to roll this in with
/// adding position information to the parser tokens and errors
/// though.
#[derive(Fail, Debug, PartialEq)]
pub enum EvalError {
    /// Unexpected token.
    #[fail(display = "unexpected token")]
    Unexpected,

    /// Incomplete data
    #[fail(display = "incomplete expression")]
    Incomplete,
}

/// Parser result type
///
/// Returned from parsing functions when success can't be guaranteed.
pub type ParseResult<T> = ::std::result::Result<T, ParseError>;

/// Parser error type
///
/// This distinguishes between the different
/// kinds of errors that the `Parser` can encounter.
///
/// TODO: Both variants of this type should have more data
/// attached. It would be nice to know _what_ token was unexpected or
/// what the incomplete expression could have continued with (for
/// error recovery). It probably makes sense to roll this in with
/// adding position information to the parser tokens and errors
/// though.
#[derive(Fail, Debug, PartialEq)]
pub enum ParseError {
    /// Unexpected token.
    #[fail(display = "unexpected token")]
    Unexpected,

    /// Incomplete data
    #[fail(display = "incomplete expression")]
    Incomplete,
}
