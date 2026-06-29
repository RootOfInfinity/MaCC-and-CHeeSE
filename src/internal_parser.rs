use crate::internal_lexer::{Attr, LexEngine, Token};

// Hand-written internal recursive descent parser
pub struct ParsingEngine {
    cur_tok: (Token, Attr, (usize, usize)),
    lex_engine: LexEngine,
}

impl ParsingEngine {
    pub fn new_from_lexengine(mut lex_engine: LexEngine) -> Result<Self, &'static str> {
        let cur_tok = lex_engine
            .next_tok()
            .ok_or("No tokens left in lex engine")?;
        Ok(ParsingEngine {
            cur_tok,
            lex_engine,
        })
    }
    fn next_tok(&mut self) -> Result<(), &'static str> {
        let next_tok = self
            .lex_engine
            .next_tok()
            .ok_or("No tokens left in lex engine")?;
        self.cur_tok = next_tok;
        Ok(())
    }
    pub fn start(&mut self) -> Result<Start, &'static str> {
        Ok(Start(self.rules()?))
    }
    pub fn rules(&mut self) -> Result<Rules, &'static str> {
        if let Token::Nonterm = self.cur_tok.0 {
            Ok(Rules(Some((self.rule()?, Box::new(self.rules()?)))))
        } else {
            Ok(Rules(None))
        }
    }
    pub fn rule(&mut self) -> Result<Rule, &'static str> {
        // no error checking, this parser will be generated later anyways
        let gotta_be_nonterm = self.cur_tok.clone();
        self.next_tok();
        let gotta_be_produces = self.cur_tok.clone();
        self.next_tok();
        let symlist = self.sym_list()?;
        let gotta_be_semicolon = self.cur_tok.clone();
        self.next_tok();
        let extrarule = self.extra_rule()?;
        Ok(Rule(
            gotta_be_nonterm,
            gotta_be_produces,
            symlist,
            gotta_be_semicolon,
            extrarule,
        ))
    }
    pub fn sym_list(&mut self) -> Result<SymList, &'static str> {
        // error checking later with generated parser
        Ok(SymList(match self.cur_tok.0 {
            Token::Nonterm | Token::Term => Some((self.symbol()?, Box::new(self.sym_list()?))),
            _ => None,
        }))
    }
    pub fn symbol(&mut self) -> Result<Symbol, &'static str> {
        let gotta_be_nonterm_or_term = self.cur_tok.clone();
        self.next_tok();
        Ok(Symbol(gotta_be_nonterm_or_term))
    }
    pub fn extra_rule(&mut self) -> Result<ExtraRule, &'static str> {
        Ok(ExtraRule(if let Token::Bar = self.cur_tok.0 {
            let gotta_be_bar = self.cur_tok.clone();
            self.next_tok();
            let symlist = self.sym_list()?;
            let gotta_be_semicolon = self.cur_tok.clone();
            self.next_tok();
            let extrarule = self.extra_rule()?;
            Some((
                gotta_be_bar,
                symlist,
                gotta_be_semicolon,
                Box::new(extrarule),
            ))
        } else {
            None
        }))
    }
}

// need a func and an enum/struct for all nonterms

pub struct Start(Rules);

pub struct Rules(Option<(Rule, Box<Rules>)>);

pub struct Rule(Term, Term, SymList, Term, ExtraRule);

pub struct SymList(Option<(Symbol, Box<SymList>)>);

pub struct Symbol(Term);

pub struct ExtraRule(Option<(Term, SymList, Term, Box<ExtraRule>)>);

type Term = (Token, Attr, (usize, usize));
