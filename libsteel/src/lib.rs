use std::collections::HashMap;

pub struct Ast(Vec<Dec>);

pub struct Dec {
    pub id: String,
    pub val: Val,
}

pub enum Type {
    Int,
    Str,
    Num,
    Cha,
    Boo,
    Lst(Box<Type>),
    Tup(Vec<Type>),
    Map(Box<Type>, Box<Type>),
    Cmd(Box<Type>, Box<Type>),
}

pub enum Val {
    Int(Int),
    Str(Str),
    Num(Num),
    Cha(Cha),
    Boo(Boo),
    Lst(Lst),
    Map(Map),
    Tup(Tup),
    Sys(Sys),
}

impl Val {
    fn from<T>(v: T) {
        match v {}
    }
}

pub struct Int(i64);
pub struct Str(String);
pub struct Num(f64);
pub struct Cha(char);
pub struct Boo(bool);
pub struct Lst(Vec<Val>);
pub struct Tup(Vec<Val>);
pub struct Map(HashMap<Val, Val>);
pub struct Blk(Vec<Line>);
pub struct Line(Vec<LnElem>);

pub struct Sys {
    pub input: Type,
    pub block: Blk,
    pub out: Type,
}

pub enum LnElem {
    Cmd(Cmd),
    Pipe,
    Write,
    Split,
    Combine,
}

pub struct Cmd {
    pub main: Acc,
    pub args: Vec<Acc>,
}

pub struct Acc(Vec<String>);
