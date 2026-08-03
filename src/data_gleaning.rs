use crate::internal_parser::Start;

pub fn data_from_parse(ast: Start) -> (Vec<Production>, Vec<String>, Vec<String>) {
    todo!()
}

pub struct Production {
    nonterm: String,
    symbols: Vec<String>,
}
