use std::{collections::HashMap, str::FromStr};

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
    Sys(Box<Type>, Box<Type>),
}

impl FromStr for Type {
    type Err = TypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Type::Int),
            "str" => Ok(Type::Str),
            "flt" => Ok(Type::Flt),
            "cha" => Ok(Type::Cha),
            "boo" => Ok(Type::Boo),
            unknown => Err(TypeErr(unknown.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct TypeErr(String);

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
