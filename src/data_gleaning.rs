use std::collections::HashSet;

use crate::{
    internal_lexer::{Attr, Token},
    internal_parser::{Rule, Rules, Start, Store, Symbol},
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
    let mut prodvec = Vec::new();
    for Rule(term, symlist, extrarule) in rules.0.iter() {
        let (Token::Nonterm, Attr::AttrString(nonterm), _) = term else {
            // can only get here with programmer error once error
            // handling is fixed in internal_parser and internal_lexer
            panic!();
        };
        let mut symvec = Vec::new();
        for Symbol(term) in symlist.0.iter() {
            let (token, attr, _) = term;
            let Attr::AttrString(string) = attr else {
                panic!();
            };
            symvec.push((token.clone(), string.clone()));
        }
        prodvec.push(Production {
            nonterm: nonterm.clone(),
            symbols: symvec,
        });
        // extra rules time
        for extra_symbols in extrarule.0.iter() {
            let mut symvec = Vec::new();
            for Symbol(term) in extra_symbols.0.iter() {
                let (token, attr, _) = term;
                let Attr::AttrString(string) = attr else {
                    panic!();
                };
                symvec.push((token.clone(), string.clone()));
            }
            prodvec.push(Production {
                nonterm: nonterm.clone(),
                symbols: symvec,
            });
        }
    }
    prodvec
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
fn all_nonterms_in_productions(prods: &Vec<Production>) -> HashSet<String> {
    // get all the nonterms on the left side, but make sure there isn't
    // any wild nonterms on the right side of productions by throwing
    // error with panic and crashing
    let mut nonterm_set = HashSet::new();
    for prod in prods.iter() {
        nonterm_set.insert(prod.nonterm.clone());
    }
    // check lap
    for prod in prods.iter() {
        for symbol in prod.symbols.iter() {
            if let Token::Nonterm = symbol.0 {
                if nonterm_set.contains(&symbol.1) {
                    // does need to panic
                    panic!("There is a nonterm without a definition");
                }
            }
        }
    }
    nonterm_set
}
fn all_terms_in_productions(prods: &Vec<Production>) -> HashSet<String> {
    let mut term_set = HashSet::new();
    for prod in prods.iter() {
        for symbol in prod.symbols.iter() {
            if let Token::Term = symbol.0 {
                term_set.insert(symbol.1.clone());
            }
        }
    }
    term_set
}

pub struct Production {
    pub nonterm: String,
    pub symbols: Vec<(Token, String)>,
}

pub struct ParserData {
    pub productions: Vec<Production>,
    pub nonterms: HashSet<String>,
    pub terms: HashSet<String>,
    pub terms_to_store: HashSet<String>,
}
