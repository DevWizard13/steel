use libsteel::*;
use ron;
use untwine::{self, parser, parser_repl};

fn main() {
    println!("{:?}", parser_repl(program));
}

parser! {
    pub program: decs=dec+ -> Ast { Ast::new(decs) }
    dec: ident=id ws* "=" ws* value=val -> Dec { Dec {id: ident.to_string(), val: value} }
    id=<'a'-'z' ('a'-'z' | '0'-'9' | "_")> -> &str;
    val=(flt | int) -> Val;
    int: nums=<num+> -> Val { Val::Int(nums.parse::<i64>().expect("Can't parse numbers"))}
    flt: decimal=<num+ "." num*> ->
        Val { Val::Flt(decimal.parse::<f64>().expect("Faliled to parse flt"))}
    escape = match {
        "n" => '\n',
        "t" => '\t',
        "r" => '\r',
        "u" code=<#[repeat(4)] {|c| c.is_digit(16)}> => {
              char::from_u32(u32::from_str_radix(code, 16)
                  .expect("Can't barse binary"))
                  .expect("Binary parsing failed")
        },
        c=[^"u"] => c,
    } -> char;
    str_char=("\\" escape | [^"\"\\"]) -> char;
    num=<'0'-'9'> -> &str;
    ws=[" /t"] -> char;
}
