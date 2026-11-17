use libsteel::*;
use ron;
use untwine::{self, parser};

fn main() {
    println!("Hello, world!");
}

parser! {
    dec: id=<'a'-'Z' | "_"> "=" val=value -> Dec { Dec {id: id.to_string(), val} }
    int: nums=<num+> -> Val::Int{Int(nums.parse())}
    num=<'0'-'9'> -> char;
}
