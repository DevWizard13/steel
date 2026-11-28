use ron::*;
use serde::{self, Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Ast(Vec<Dec>);

impl Ast {
    pub fn new(v: Vec<Dec>) -> Ast {
        Ast(v)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dec {
    pub id: String,
    pub val: Val,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    Custom(String, Vec<Type>),
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TypeErr(String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Val {
    Int(i64),
    Str(String),
    Flt(f64),
    Cha(char),
    Boo(bool),
    Lst(Vec<Val>),
    Map(HashMap<String, Box<Val>>),
    Tup(Vec<Val>),
    Sys(Sys),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Blk(Vec<Line>);
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Line(Vec<LnElem>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Sys {
    pub input: Type,
    pub block: Blk,
    pub out: Type,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LnElem {
    Cmd(Cmd),
    Pipe,
    Write,
    Split,
    Combine,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Cmd {
    pub main: Acc,
    pub args: Vec<Acc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Acc(Vec<String>);
