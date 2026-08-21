use std::collections::{HashMap, HashSet};

use crate::{
    internal_lexer::{Attr, Token},
    internal_parser::{self, Rule, Rules, Start, Store},
};

pub fn data_from_parse(ast: &Start) -> ParserData {
    let Start(store, rules) = ast;
    let mut term_map = HashMap::new();
    let mut nonterm_map = HashMap::new();

    nonterm_map.insert(String::from("S"), 0);
    term_map.insert(String::from("$"), 0);

    // 0 is the nonterm id for 'S' or 'Start'
    //     They need to have an S nonterm, so it is garunteed to appear
    // 0 is the term id for '$' or the end character
    //     They cannot name terms $, so it is garunteed to not be taken

    let prods = all_productions_in_ast(rules, &mut term_map, &mut nonterm_map);
    let terms_to_store = all_stored_terms_in_ast(store, &term_map);

    let mut reverse_term_map = HashMap::new();
    for (term_string, term_num) in term_map {
        reverse_term_map.insert(Term(term_num), term_string);
    }
    let mut reverse_nonterm_map = HashMap::new();
    for (nonterm_string, nonterm_num) in nonterm_map {
        reverse_nonterm_map.insert(Nonterm(nonterm_num), nonterm_string);
    }

    ParserData {
        productions: prods,
        terms_to_store,
        reverse_term_map,
        reverse_nonterm_map,
    }
}

fn all_productions_in_ast(
    rules: &Rules,
    term_map: &mut HashMap<String, i32>,
    nonterm_map: &mut HashMap<String, i32>,
) -> Vec<Production> {
    let mut prodvec = Vec::new();
    let mut cur_term_id = 1;
    let mut cur_nonterm_id = 1;
    for Rule(term, derivation, extrarule) in rules.0.iter() {
        let (Token::Nonterm, Attr::AttrString(nonterm), _) = term else {
            // can only get here with programmer error once error
            // handling is fixed in internal_parser and internal_lexer
            panic!();
        };
        if !nonterm_map.contains_key(nonterm) {
            nonterm_map.insert(nonterm.clone(), cur_nonterm_id);
            cur_nonterm_id += 1;
        }
        let mut symvec = Vec::new();
        if let internal_parser::Derivation::B(symlist) = derivation {
            for crate::internal_parser::Symbol(term) in symlist.0.iter() {
                let (token, attr, _) = term;
                let Attr::AttrString(string) = attr else {
                    panic!();
                };
                match token {
                    Token::Nonterm => {
                        if !nonterm_map.contains_key(string) {
                            nonterm_map.insert(string.clone(), cur_nonterm_id);
                            cur_nonterm_id += 1;
                        }
                        symvec.push(Symbol::N(Nonterm(*nonterm_map.get(string).unwrap())));
                    }
                    Token::Term => {
                        if !term_map.contains_key(string) {
                            term_map.insert(string.clone(), cur_term_id);
                            cur_term_id += 1;
                        }
                        symvec.push(Symbol::T(Term(*term_map.get(string).unwrap())));
                    }
                    _ => (),
                }
            }
            prodvec.push(Production {
                nonterm: Nonterm(*nonterm_map.get(nonterm).unwrap()),
                derivation: Derivation::Symbols(symvec),
            });
        } else {
            prodvec.push(Production {
                nonterm: Nonterm(*nonterm_map.get(nonterm).unwrap()),
                derivation: Derivation::Null,
            });
        }
        // extra rules time
        for extra_derivation in extrarule.0.iter() {
            let mut symvec = Vec::new();
            if let internal_parser::Derivation::B(symlist) = extra_derivation {
                for crate::internal_parser::Symbol(term) in symlist.0.iter() {
                    let (token, attr, _) = term;
                    let Attr::AttrString(string) = attr else {
                        panic!();
                    };
                    match token {
                        Token::Nonterm => {
                            if !nonterm_map.contains_key(string) {
                                nonterm_map.insert(string.clone(), cur_nonterm_id);
                                cur_nonterm_id += 1;
                            }
                            symvec.push(Symbol::N(Nonterm(*nonterm_map.get(string).unwrap())));
                        }
                        Token::Term => {
                            if !term_map.contains_key(string) {
                                term_map.insert(string.clone(), cur_term_id);
                                cur_term_id += 1;
                            }
                            symvec.push(Symbol::T(Term(*term_map.get(string).unwrap())));
                        }
                        _ => (),
                    }
                }
                prodvec.push(Production {
                    nonterm: Nonterm(*nonterm_map.get(nonterm).unwrap()),
                    derivation: Derivation::Symbols(symvec),
                });
            } else {
                prodvec.push(Production {
                    nonterm: Nonterm(*nonterm_map.get(nonterm).unwrap()),
                    derivation: Derivation::Null,
                });
            }
        }
    }
    prodvec
}
fn all_stored_terms_in_ast(store: &Store, term_map: &HashMap<String, i32>) -> HashSet<Term> {
    // should have caught if they weren't terminal identifers in internal_parser
    // therefore, panics are a bug in my programming, not seen by users
    let mut set = HashSet::new();
    for (tok, attrstring, _) in store.0.iter() {
        let Token::Term = tok else { panic!() };
        let Attr::AttrString(term_to_store) = attrstring else {
            panic!()
        };
        set.insert(Term(*term_map.get(term_to_store).unwrap()));
    }
    set
}
// Previous implementation inserted right into all_productions_in_ast
// fn all_nonterms_in_productions(prods: &Vec<Production>) -> HashSet<String> {
//     // get all the nonterms on the left side, but make sure there isn't
//     // any wild nonterms on the right side of productions by throwing
//     // error with panic and crashing
//     let mut nonterm_set = HashSet::new();
//     for prod in prods.iter() {
//         nonterm_set.insert(prod.nonterm.clone());
//     }
//     // check lap
//     for prod in prods.iter() {
//         for symbol in prod.symbols.iter() {
//             if let Token::Nonterm = symbol.0 {
//                 if !nonterm_set.contains(&symbol.1) {
//                     // does need to panic
//                     panic!("There is a nonterm without a definition");
//                 }
//             }
//         }
//     }
//     nonterm_set
// }
// fn all_terms_in_productions(prods: &Vec<Production>) -> HashSet<String> {
//     let mut term_set = HashSet::new();
//     for prod in prods.iter() {
//         for symbol in prod.symbols.iter() {
//             if let Token::Term = symbol.0 {
//                 term_set.insert(symbol.1.clone());
//             }
//         }
//     }
//     term_set
// }

#[derive(Clone, Debug)]
pub struct Production {
    pub nonterm: Nonterm,
    pub derivation: Derivation,
}

#[derive(Clone)]
pub struct ParserData {
    pub productions: Vec<Production>,
    // pub nonterms: HashSet<Nonterm>,
    // pub terms: HashSet<Term>,
    pub terms_to_store: HashSet<Term>,
    pub reverse_term_map: HashMap<Term, String>,
    pub reverse_nonterm_map: HashMap<Nonterm, String>,
}
#[derive(Clone, Debug)]
pub enum Derivation {
    Null,
    Symbols(Vec<Symbol>),
}
#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub struct Term(pub i32);
#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub struct Nonterm(pub i32);
#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum Symbol {
    T(Term),
    N(Nonterm),
}
