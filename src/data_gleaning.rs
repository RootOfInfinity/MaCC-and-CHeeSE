use crate::internal_parser::Start;

pub fn data_from_parse(ast: &Start) -> ParserData {
    let prods = all_productions_in_ast(ast);
    let nonterms = all_nonterms_in_productions(&prods);
    let terms = all_terms_in_productions(&prods);
    ParserData {
        productions: prods,
        nonterms,
        terms,
    }
}

fn all_productions_in_ast(ast: &Start) -> Vec<Production> {
    todo!()
}
fn all_nonterms_in_productions(prods: &Vec<Production>) -> Vec<String> {
    todo!()
}
fn all_terms_in_productions(prods: &Vec<Production>) -> Vec<String> {
    todo!()
}

pub struct Production {
    pub nonterm: String,
    pub symbols: Vec<String>,
}

pub struct ParserData {
    pub productions: Vec<Production>,
    pub nonterms: Vec<String>,
    pub terms: Vec<String>,
}
