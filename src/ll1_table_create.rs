use std::collections::HashMap;

use crate::data_gleaning::{Derivation, Nonterm, Production, Symbol, Term};

/// Test this func to make sure it works
fn refine_prods_into_hashmap(prods: &Vec<Production>) -> HashMap<Nonterm, Vec<Derivation>> {
    let mut refined_map: HashMap<Nonterm, Vec<Derivation>> = HashMap::new();
    for prod in prods {
        match refined_map.get_mut(&prod.nonterm) {
            Some(deriv_vec) => {
                deriv_vec.push(prod.derivation.clone());
            }
            None => {
                refined_map.insert(prod.nonterm, vec![prod.derivation.clone()]);
            }
        }
    }
    refined_map
}

/// Algorithm from 'The Red Dragon Book', is true if the empty string is also part of it.
fn first(g_string: &Vec<Symbol>, prods: &HashMap<Nonterm, Vec<Derivation>>) -> (Vec<Term>, bool) {
    let mut empty_string_found = true;
    let mut term_vec = Vec::new();
    for sym in g_string {
        match sym {
            &Symbol::T(term) => {
                term_vec.push(term);
                empty_string_found = false;
                break;
            }
            &Symbol::N(nonterm) => {
                let derivations_vec = prods.get(&nonterm).unwrap();
                let mut nonterm_first = Vec::new();
                let mut empty_string = false;
                for derivation in derivations_vec {
                    match derivation {
                        Derivation::Null => {
                            empty_string = true;
                        }
                        Derivation::Symbols(sym_string) => {
                            let (mut first_vec, specific_empty_string) = first(sym_string, prods);
                            nonterm_first.append(&mut first_vec);
                            if specific_empty_string {
                                empty_string = true;
                            }
                        }
                    }
                }
                if !empty_string {
                    empty_string_found = false;
                    break;
                }
            }
        }
    }
    (term_vec, empty_string_found)
}

fn follow(nonterm: &Nonterm) -> Vec<Term> {
    todo!()
}
