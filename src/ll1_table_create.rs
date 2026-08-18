use std::collections::{HashMap, HashSet};

const START_ID: i32 = 0;
const END_ID: i32 = 0;

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
fn first(g_string: &[Symbol], prods: &HashMap<Nonterm, Vec<Derivation>>) -> (HashSet<Term>, bool) {
    let mut empty_string_found = true;
    let mut term_set = HashSet::new();
    for sym in g_string {
        match sym {
            &Symbol::T(term) => {
                term_set.insert(term);
                empty_string_found = false;
                break;
            }
            &Symbol::N(nonterm) => {
                let derivations_vec = prods.get(&nonterm).unwrap();
                let mut nonterm_first = HashSet::new();
                let mut empty_string = false;
                for derivation in derivations_vec {
                    match derivation {
                        Derivation::Null => {
                            empty_string = true;
                        }
                        Derivation::Symbols(sym_string) => {
                            let (first_set, specific_empty_string) = first(sym_string, prods);
                            nonterm_first.extend(first_set.into_iter());
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
    (term_set, empty_string_found)
}
/// Algorithm from 'The Red Dragon Book', calls FIRST(X) as well as itself, could benefit from cache
/// Needs to go through all derivations, so might as well do all at once.
fn follow_all(
    prods: &Vec<Production>,
    cool_prods: &HashMap<Nonterm, Vec<Derivation>>,
) -> HashMap<Nonterm, HashSet<Term>> {
    let mut follow_map = HashMap::new();
    let mut start_hashset = HashSet::new();
    start_hashset.insert(Term(END_ID));
    follow_map.insert(Nonterm(START_ID), start_hashset);
    for prod in prods {
        let left_nonterm = prod.nonterm;
        let derivation = &prod.derivation;
        match derivation {
            Derivation::Null => (),
            Derivation::Symbols(symbols) => {
                for sym_idx in 0..symbols.len() {
                    if let Symbol::N(cur_nonterm) = symbols[sym_idx] {
                        let (mut cur_first_vec, _empty_string) =
                            first(&symbols[sym_idx..symbols.len()], cool_prods);
                        let mut cur_follow_vec = HashSet::new();
                        cur_follow_vec.extend(cur_first_vec.into_iter());
                        if !follow_map.contains_key(&cur_nonterm) {
                            follow_map.insert(cur_nonterm, cur_follow_vec);
                        } else {
                            follow_map
                                .get_mut(&cur_nonterm)
                                .unwrap()
                                .extend(cur_follow_vec.into_iter());
                        }
                        // need to get follow from left_nonterm if empty string is true,
                        // lets just do another round until it cannot make any more.
                    } else {
                        continue;
                    }
                }
                loop {
                    let mut nothing_changed = true;
                    for sym_idx in 0..symbols.len() {
                        if let Symbol::N(cur_nonterm) = symbols[sym_idx] {
                            let (_cur_first_vec, empty_string) =
                                first(&symbols[sym_idx..symbols.len()], cool_prods);
                            if empty_string {
                                // need to check if we hadn't already added it
                                if let Some(left_nonterm_follow) = follow_map.get(&left_nonterm) {
                                    let left_nonterm_follow = left_nonterm_follow.clone();
                                    if let Some(cur_nonterm_follow) =
                                        follow_map.get_mut(&cur_nonterm)
                                    {
                                        let mut all_left_nonterm_follow_is_in_cur_nonterm_follow =
                                            true;
                                        for term in left_nonterm_follow.iter() {
                                            if !cur_nonterm_follow.contains(&term) {
                                                nothing_changed = false;
                                                all_left_nonterm_follow_is_in_cur_nonterm_follow =
                                                    false;
                                                break;
                                            }
                                        }
                                        if all_left_nonterm_follow_is_in_cur_nonterm_follow {
                                            cur_nonterm_follow.extend(left_nonterm_follow);
                                        }
                                    }
                                }
                            }
                        } else {
                            continue;
                        }
                    }
                    if nothing_changed {
                        break;
                    }
                }
            }
        }
    }

    follow_map
}
