use std::fmt;
use std::num::ParseFloatError;
use std::sync::Arc;

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct DebugExpr {
    inner: Expr,
    location: (u32, u32), // (line, col)
    error: Option<Error>,
}

#[derive(Clone)]
pub enum Expr {
    Symbol(String),
    Number(f64),
    List(Vec<Expr>),
    Fun(fn(&[Expr]) -> Result<Expr, Error>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Expr").finish()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(String),
    // ParseError(String, u32, u32),
}

#[derive(Debug, Clone)]
pub struct Lambda {
    params: Arc<Expr>,
    body: Arc<Expr>,
}

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Symbol(k) => env
            .inner
            .get(k)
            .ok_or(Error::ParseError(format!("unexpected symbol k='{}'", k)))
            .map(|x| x.clone()),
        Expr::Number(_a) => Ok(expr.clone()),
        Expr::List(list) => {
            let first_form = list
                .first()
                .ok_or(Error::ParseError("list must not be empty".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(env, first_form)?;
            match first_eval {
                Expr::Fun(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(env, x))
                        .collect::<Result<Vec<Expr>, Error>>();

                    f(&args_eval?)
                }
                _ => Err(Error::ParseError(
                    "first form must be a function".to_string(),
                )),
            }
        }
        Expr::Fun(_) => Err(Error::ParseError("unexpected form".to_string())),
    }
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Error::ParseError("failed to parse token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(Error::ParseError("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn tokenize(i: String) -> Vec<String> {
    i.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|atom| atom.to_string())
        .collect()
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), Error> {
    let mut res: Vec<Expr> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(Error::ParseError("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((Expr::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> Expr {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => Expr::Number(v),
        Err(_) => Expr::Symbol(token.to_string()),
    }
}

fn parse_float(exp: &Expr) -> Result<f64, Error> {
    match exp {
        Expr::Number(num) => Ok(*num),
        _ => Err(Error::ParseError("expected a number".to_string())),
    }
}

pub fn parse_float_list(args: &[Expr]) -> Result<Vec<f64>, Error> {
    args.iter().map(|x| parse_float(x)).collect()
}
