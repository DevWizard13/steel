use libsteel::*;
use ron;
use untwine::{self, parser, parser_repl};

fn main() {
    println!("{:?}", parser_repl(dec));
}

parser! {
    pub program: decs=dec$(ws)+ -> Ast { Ast::new(decs) }

    pub dec: ident=id ws "=" ws value=val -> Dec { Dec {id: ident.to_string(), val: value} }
    id=<'a'-'z' ('a'-'z' | '0'-'9' | "_")*> -> &str;
    val=(flt | int | str_val | cha | boo) -> Val;

    str_val: "\"" chars=<str_char*> "\"" -> Val { Val::Str(chars.to_string()) }
    int: nums=<num+> -> Val { Val::Int(nums.parse::<i64>().expect("Can't parse numbers"))}
    flt: decimal=<num+ "." num*> ->
        Val { Val::Flt(decimal.parse::<f64>().expect("Faliled to parse flt"))}
    cha: "'" c=str_char "'" -> Val { Val::Cha(c) }
    pub boo: b=<"tru"|"fal"> -> Val { let hi = if b == "tru" {Val::Boo(true)} else {Val::Boo(false)};
        println!("{:?}", hi); hi}
    //pub boo = match { "fal" => Val::Boo(false),"tru" => Val::Boo(true),} -> Val;

    str_char=("\\" escape | [^"\"\\"]) -> char;
    escape = match {
        "n" => '\n',
        "t" => '\t',
        "r" => '\r',
        "u" code=<#[repeat(4)] {|c| c.is_digit(16)}> => {
              char::from_u32(u32::from_str_radix(code, 16)
                  .expect("Not binary"))
                  .expect("Binary parsing failed")
        },
        c=[^"u"] => c,
    } -> char;
    num=<'0'-'9'> -> &str;
    ws = #["\t "]*;
}
