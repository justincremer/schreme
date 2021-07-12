extern crate schreme;

use schreme::{parse, tokenize};

fn main() {
    let exprs = ["(+ 2 2)", "()", "(+ 2 (+ 2 2))"];
    exprs.iter().for_each(|i| parse_exr(i));
}

fn parse_exr(i: &str) {
    let tokens = tokenize(i.to_string());
    let parsed = parse(tokens.as_slice());
    println!("{:?}", parsed);
}
