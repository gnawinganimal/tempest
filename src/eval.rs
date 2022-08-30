use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Fn { ident: &'a str, body: Box<Self>},
    Blk(Vec<Self>),
    Rec(HashMap<&'a str, Self>),
    Str(&'a str),
    Num(f64),
    Bool(bool),
    Null,    
}

pub fn expand(expr: &Expr) -> String {
    match expr {
        Expr::Blk(inner) => {
            inner
            .iter()
            .map(|expr| expand(expr))
            .collect::<Vec<String>>()
            .join("\n")
        },
        Expr::Str(value) => value.to_string(),
        Expr::Num(value) => value.to_string(),
        Expr::Bool(value) => {
            if *value {
                "true".to_string()
            } else {
                "false".to_string()
            }
        },
        _ => panic!("Not implemeneted"),
    }
}
