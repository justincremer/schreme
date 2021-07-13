use crate::parser::{parse_float_list, Error, Expr};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    pub inner: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Env::default()
    }
}

impl Default for Env {
    fn default() -> Self {
        let mut inner: HashMap<String, Expr> = HashMap::new();

        inner.insert(
            "+".to_string(),
            Expr::Fun(|args: &[Expr]| -> Result<Expr, Error> {
                Ok(Expr::Number(
                    parse_float_list(args)?.as_slice().iter().sum(),
                ))
            }),
        );

        inner.insert(
            "-".to_string(),
            Expr::Fun(|args: &[Expr]| -> Result<Expr, Error> {
                let floats = parse_float_list(args)?;
                let x = floats.first().expect("list must not be empty");
                let xs = -(floats[1..].iter().sum::<f64>());

                Ok(Expr::Number([x + 0.0, xs].iter().sum()))
            }),
        );

        Env { inner }
    }
}
