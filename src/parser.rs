use std::collections::HashMap;
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub enum Expression {
    Symbol(String),
    Number(f64),
    List(Vec<Expression>),
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    // ParseError(String, u32, u32),
}

#[derive(Debug, Clone)]
pub struct Environment {
    data: HashMap<String, Expression>,
}

pub fn tokenize(i: String) -> Vec<String> {
    i.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|atom| atom.to_string())
        .collect()
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Error::ParseError("failed to parse token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(Error::ParseError("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let mut res: Vec<Expression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(Error::ParseError("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((Expression::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> Expression {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => Expression::Number(v),
        Err(_) => Expression::Symbol(token.to_string().clone()),
    }
}
