pub mod values;
use crate::values::*;

struct Ast(Vec<Dec>);

struct Dec {
    id: String,
    val: Val,
}
