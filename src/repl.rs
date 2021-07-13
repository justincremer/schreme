use crate::{
    env::Env,
    parser::{eval, parse, tokenize, Expr},
};
use rustyline::{error::ReadlineError, Editor};

const HIST_FILE: &'static str = "repl.hist";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn init() {
    let mut rl = Editor::<()>::new();
    let mut env = Env::new();

    println!("Schreme version {}", VERSION);
    if rl.load_history(HIST_FILE).is_err() {
        eprintln!("No previous history file exists");
    }

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let tokens = tokenize(line);
                let parsed = parse(tokens.as_slice()).unwrap().0;
                match eval(&mut env, &parsed) {
                    Ok(v) => {
                        let res = match v {
                            Expr::Symbol(s) => s.clone(),
                            Expr::Number(n) => n.to_string(),
                            Expr::List(list) => {
                                let xs: Vec<String> = list
                                    .iter()
                                    .map(|x| format!("{:?}", eval(&mut env, x)))
                                    .collect();
                                format!("({})", xs.join(","))
                            }
                            Expr::Fun(_) => "Function {}".to_string(),
                        };
                        println!("{:?}", v);
                    }
                    Err(_) => {}
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HIST_FILE).unwrap();
}

fn parse_line(i: &String) {
    let tokens = tokenize(i.to_owned());
    let parsed = parse(tokens.as_slice());
    println!("{:?}", parsed);
}
