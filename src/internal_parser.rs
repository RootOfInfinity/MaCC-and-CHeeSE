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
}
