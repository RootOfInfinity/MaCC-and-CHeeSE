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
        println!("CUR TOK: {:?}", cur_tok);
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
        println!("CUR TOK: {:?}", self.cur_tok);
        Ok(())
    }
    pub fn start(&mut self) -> Result<Start, &'static str> {
        Ok(Start(self.store()?, self.rules()?))
    }
    fn store(&mut self) -> Result<Store, &'static str> {
        // assume first term is 'Store'
        let Token::Store = self.cur_tok.0 else {
            println!("{:?}", self.cur_tok);
            return Err("Expected token: store");
        };
        self.next_tok();
        // assume second term is 'LeftArrow'
        let Token::LeftArrow = self.cur_tok.0 else {
            return Err("Expected token: left_arrow");
        };
        self.next_tok();
        let mut termvec = Vec::new();
        while let Token::Term = self.cur_tok.0 {
            let gotta_be_term = self.cur_tok.clone();
            self.next_tok();
            termvec.push(gotta_be_term);
        }
        // assume last term is 'RightArrow'
        let Token::RightArrow = self.cur_tok.0 else {
            return Err("Expected token: right_arrow");
        };
        self.next_tok();
        Ok(Store(termvec))
    }
    fn rules(&mut self) -> Result<Rules, &'static str> {
        // if let Token::Nonterm = self.cur_tok.0 {
        //     Ok(Rules(Some((self.rule()?, Box::new(self.rules()?)))))
        // } else {
        //     Ok(Rules(None))
        // }
        let mut rulevec = Vec::new();
        while let Token::Nonterm = self.cur_tok.0 {
            rulevec.push(self.rule()?);
        }
        Ok(Rules(rulevec))
    }
    fn rule(&mut self) -> Result<Rule, &'static str> {
        let Token::Nonterm = self.cur_tok.0 else {
            return Err("Expected token: nonterm");
        };
        let gotta_be_nonterm = self.cur_tok.clone();
        self.next_tok();
        // let gotta_be_produces = self.cur_tok.clone();
        let Token::Produces = self.cur_tok.0 else {
            return Err("Expected token: produces");
        };
        self.next_tok();
        let derivation = self.derivation()?;
        // let gotta_be_semicolon = self.cur_tok.clone();
        let Token::Semicolon = self.cur_tok.0 else {
            return Err("Expected token: semicolon");
        };
        self.next_tok();
        let extrarule = self.extra_rule()?;
        Ok(Rule(gotta_be_nonterm, derivation, extrarule))
    }
    fn sym_list(&mut self) -> Result<SymList, &'static str> {
        // error checking later with generated parser
        // Ok(SymList(match self.cur_tok.0 {
        //     Token::Nonterm | Token::Term => Some((self.symbol()?, Box::new(self.sym_list()?))),
        //     _ => None,
        // }))
        let mut symvec = Vec::new();
        while let Token::Nonterm | Token::Term = self.cur_tok.0 {
            symvec.push(self.symbol()?);
        }
        Ok(SymList(symvec))
    }
    fn derivation(&mut self) -> Result<Derivation, &'static str> {
        match self.cur_tok.0 {
            Token::NullVal => {
                self.next_tok();
                Ok(Derivation::A)
            }
            _ => Ok(Derivation::B(self.sym_list()?)),
        }
    }
    fn symbol(&mut self) -> Result<Symbol, &'static str> {
        match self.cur_tok.0 {
            Token::Nonterm | Token::Term => (),
            _ => {
                return Err("Expected token: term OR nonterm");
            }
        }
        let gotta_be_nonterm_or_term = self.cur_tok.clone();
        self.next_tok();
        Ok(Symbol(gotta_be_nonterm_or_term))
    }
    fn extra_rule(&mut self) -> Result<ExtraRule, &'static str> {
        // Ok(ExtraRule(if let Token::Bar = self.cur_tok.0 {
        //     let gotta_be_bar = self.cur_tok.clone();
        //     self.next_tok();
        //     let symlist = self.sym_list()?;
        //     let gotta_be_semicolon = self.cur_tok.clone();
        //     self.next_tok();
        //     let extrarule = self.extra_rule()?;
        //     Some((
        //         gotta_be_bar,
        //         symlist,
        //         gotta_be_semicolon,
        //         Box::new(extrarule),
        //     ))
        // } else {
        //     None
        // }))
        let mut extrarulevec = Vec::new();
        while let Token::Bar = self.cur_tok.0 {
            // let gotta_be_bar = self.cur_tok.clone();

            let Token::Bar = self.cur_tok.0 else {
                return Err("Expected token: bar");
            };
            self.next_tok();
            let derivation = self.derivation()?;
            // let gotta_be_semicolon = self.cur_tok.clone();
            let Token::Semicolon = self.cur_tok.0 else {
                return Err("Expected token: semicolon");
            };
            self.next_tok();
            extrarulevec.push(derivation);
        }
        Ok(ExtraRule(extrarulevec))
    }
}

// need a func and an enum/struct for all nonterms

#[derive(Debug)]
pub struct Start(pub Store, pub Rules);

#[derive(Debug)]
pub struct Store(pub Vec<Term>);

// pub struct Rules(Option<(Rule, Box<Rules>)>);
#[derive(Debug)]
pub struct Rules(pub Vec<Rule>);

#[derive(Debug)]
pub struct Rule(pub Term, pub Derivation, pub ExtraRule);

#[derive(Debug)]
pub enum Derivation {
    A,
    B(SymList),
}

// pub struct SymList(Option<(Symbol, Box<SymList>)>);
#[derive(Debug)]
pub struct SymList(pub Vec<Symbol>);

#[derive(Debug)]
pub struct Symbol(pub Term);

// pub struct ExtraRule(Option<(Term, SymList, Term, Box<ExtraRule>)>);
#[derive(Debug)]
pub struct ExtraRule(pub Vec<Derivation>);

type Term = (Token, Attr, (usize, usize));
