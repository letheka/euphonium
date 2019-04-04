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

use std::f32;

pub use crate::error::{EvalError, EvalResult};

pub mod parse;
use self::parse::{parse_single, Expression, InfixOp, PrefixOp};

use super::samplegen::{Params, SampleGen};

#[derive(Clone)]
pub struct Waveform {
    equation: Expression,
}

impl Waveform {
    pub fn new(e: String) -> Waveform {
        Waveform {
            equation: parse_single(e).unwrap(),
        }
    }

    fn eval(&self, exp: &Expression, p: &Params) -> EvalResult<f32> {
        match exp {
            Expression::Prefix(op, arg) => {
                let arg_num = self.eval(arg, &p)?;
                let n = match op {
                    PrefixOp::Negate => -(arg_num),
                    PrefixOp::Abs => arg_num.abs(),
                    PrefixOp::Sgn => arg_num.signum(),
                    PrefixOp::Sin => (arg_num * f32::consts::PI).sin(),
                    PrefixOp::Cos => (arg_num * f32::consts::PI).cos(),
                    // _ => unreachable!()
                };
                Ok(n)
            }
            Expression::Infix(arg1, op, arg2) => {
                let arg1_num = self.eval(arg1, &p)?;
                let arg2_num = self.eval(arg2, &p)?;
                let n = match op {
                    InfixOp::Add => arg1_num + arg2_num,
                    InfixOp::Sub => arg1_num - arg2_num,
                    InfixOp::Mul => arg1_num * arg2_num,
                    InfixOp::Div => arg1_num / arg2_num,
                    InfixOp::Mod => arg1_num % arg2_num,
                    // _ => unreachable!()
                };
                Ok(n)
            }
            Expression::Number(x) => Ok(*x),
            Expression::Identifier(x) => {
                if p.contains_key(x) {
                    Ok(p[x])
                } else {
                    Err(EvalError::Incomplete)
                }
            }
            _ => Err(EvalError::Unexpected),
        }
    }
}

impl SampleGen for Waveform {
    fn cache(&mut self, _p: &Params) {}

    fn get_sample(&self, p: &Params) -> Option<f32> {
        self.eval(&self.equation, p).ok()
    }

    fn get_mod_sample(&self, p: &Params) -> Option<f32> {
        Some((self.eval(&self.equation, p).unwrap() + 1.0) / 2.0)
    }
}
