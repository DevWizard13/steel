use std::collections::HashMap;

pub enum Type {
    Int,
    Str,
    Num,
    Cha,
    Boo,
    Lst(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tup(Vec<Type>),
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

pub struct Int(i64);
pub struct Str(String);
pub struct Num(f64);
pub struct Cha(char);
pub struct Boo(bool);
pub struct Sys {
    input: Type,
    block: Blk,
    out: Type,
}
pub struct Lst(Vec<Val>);
pub struct Map(HashMap<Val, Val>);
pub struct Tup(Vec<Val>);

pub struct Blk(Vec<Lin>);

pub struct Lin(Vec<LnE>);

pub enum LnE {
    Cmd(Cmd),
    Pipe,
    Write,
    Split,
    Combine,
}

pub struct Cmd {
    main: Acc,
    args: Vec<Acc>,
}

pub struct Acc(Vec<String>);
