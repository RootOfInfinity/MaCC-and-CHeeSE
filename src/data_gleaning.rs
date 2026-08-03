use std::collections::HashSet;

use crate::{
    internal_lexer::{Attr, Token},
    internal_parser::{Rules, Start, Store},
};

pub fn data_from_parse(ast: &Start) -> ParserData {
    let Start(store, rules) = ast;
    let prods = all_productions_in_ast(rules);
    let terms_to_store = all_stored_terms_in_ast(store);
    let nonterms = all_nonterms_in_productions(&prods);
    let terms = all_terms_in_productions(&prods);
    ParserData {
        productions: prods,
        nonterms,
        terms,
        terms_to_store,
    }
}

fn all_productions_in_ast(rules: &Rules) -> Vec<Production> {
    todo!()
}
fn all_stored_terms_in_ast(store: &Store) -> HashSet<String> {
    // should have caught if they weren't terminal identifers in internal_parser
    // therefore, panics are a bug in my programming, not seen by users
    let mut set = HashSet::new();
    for (tok, attrstring, _) in store.0.iter() {
        let Token::Term = tok else { panic!() };
        let Attr::AttrString(term_to_store) = attrstring else {
            panic!()
        };
        set.insert(term_to_store.clone());
    }
    set
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
    pub terms_to_store: HashSet<String>,
}
