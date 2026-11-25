use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast(Vec<Dec>);

impl Ast {
    pub fn new(v: Vec<Dec>) -> Ast {
        Ast(v)
    }
}

#[derive(Debug)]
pub struct Dec {
    pub id: String,
    pub val: Val,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Str,
    Flt,
    Cha,
    Boo,
    Lst(Box<Type>),
    Tup(Vec<Type>),
    Map(Box<Type>, Box<Type>),
    Cmd(Box<Type>, Box<Type>),
}

#[derive(Debug)]
pub enum Val {
    Int(i64),
    Str(String),
    Flt(f64),
    Cha(char),
    Boo(bool),
    Lst(Vec<Val>),
    Map(HashMap<Val, Val>),
    Tup(Vec<Val>),
    Sys(Sys),
}

#[derive(Debug)]
pub struct Blk(Vec<Line>);
#[derive(Debug)]
pub struct Line(Vec<LnElem>);

#[derive(Debug)]
pub struct Sys {
    pub input: Type,
    pub block: Blk,
    pub out: Type,
}

#[derive(Debug)]
pub enum LnElem {
    Cmd(Cmd),
    Pipe,
    Write,
    Split,
    Combine,
}

#[derive(Debug)]
pub struct Cmd {
    pub main: Acc,
    pub args: Vec<Acc>,
}

#[derive(Debug)]
pub struct Acc(Vec<String>);
