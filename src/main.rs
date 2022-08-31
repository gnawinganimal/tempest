
mod eval;

use std::{fs, collections::HashMap};
use eval::{Expr, expand};

fn main() {
    let expr = Expr::Tag { 
        ident: "div",
        args: HashMap::new(),
        body: Box::new(Expr::Tag {
            ident: "p",
            args: HashMap::new(), 
            body: Box::new(Expr::Str("Bitch please")),
        }),
    };
    let output = expand(&expr);
    println!("{}", output);
    fs::write("output.txt", output).expect("Could not write file");
}
