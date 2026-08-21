use crate::{
    data_gleaning::{ParserData, data_from_parse},
    internal_lexer::LexEngine,
    internal_parser::ParsingEngine,
    ll1_table_create::create_ll1_parsing_table,
};

mod data_gleaning;
// Will be replaced with generated Lexers and Parsers
mod internal_lexer;
mod internal_parser;
// LL(1)
mod ll1_parser;
mod ll1_table_create;

fn main() {
    println!("Hello, world!");

    // let grammar_text = String::from(
    //     "
    //     Store<nonterm term> \n
    //     S ::= StoreVal Rules; \n
    //     StoreVal ::= store left_arrow TermList right_arrow; \n
    //     TermList ::= term TermList; | null; \n
    //     Rules ::= Rule Rules; | null; \n
    //     Rule ::= nonterm produces Derivation semicolon ExtraRule; \n
    //     Derivation ::= null_val; | SymList; \n
    //     SymList ::= Symbol SymList; | null; \n
    //     Symbol ::= nonterm; | term; \n
    //     ExtraRule ::= bar Derivation semicolon ExtraRule; | null;     \n
    //     ",
    // );
    let grammar_text = String::from(
        "
        Store<id> \n
        S ::= E; \n
        E ::= T Ep; \n
        Ep ::= plus T Ep; | null; \n
        T ::= F Tp; \n
        Tp ::= times F Tp; | null; \n
        F ::= left_paren E right_paren; | id; \n
        ",
    );

    let lex = LexEngine::new_from_string(grammar_text).unwrap();
    let mut par = ParsingEngine::new_from_lexengine(lex).unwrap();
    let ast = par.start().unwrap();

    // println!("AST: {:#?}", ast);
    let parse_data = data_from_parse(&ast);
    let ParserData {
        productions: prods,
        reverse_term_map,
        reverse_nonterm_map,
        terms_to_store: _,
    } = parse_data.clone();

    println!("Productions: {:#?}", prods);
    println!("Nonterm map: {:?}", reverse_nonterm_map.clone());
    println!("Term map: {:?}", reverse_term_map.clone());
    let ll1_table = create_ll1_parsing_table(&parse_data);

    println!("TABLE: \n{:#?}", ll1_table);
}
