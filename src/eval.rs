use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Fn { ident: &'a str, body: Box<Self>},
    Tag { ident: &'a str, args: HashMap<&'a str, Self>, body: Box<Self>},
    Blk(Vec<Self>),
    Rec(HashMap<&'a str, Self>),
    Str(&'a str),
    Num(f64),
    Bool(bool),
    Null,    
}

pub fn expand(expr: &Expr) -> String {
    match expr {
        Expr::Tag { ident, args, body } => {
            let args = args
                .iter()
                .map(|(key, value)| format!(" {}={}", key, expand(value)))
                .collect::<Vec<String>>()
                .join("");

            format!("<{}{}>{}</{}>", ident, args, expand(body), ident)
        },
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
