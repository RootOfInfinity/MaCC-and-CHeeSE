// Hand-written internal lexer
// I've given up on lifetimes for now
pub struct LexEngine {
    cur_ind: usize,
    char_vec: Vec<char>,
    cur_loc: (usize, usize),
    errors: Vec<LexError>,
}

impl LexEngine {
    pub fn new_from_string(string: String) -> Result<Self, &'static str> {
        let char_vec: Vec<char> = string.chars().collect();
        let cur_ind = 0;
        if char_vec.len() == 0 {
            Err("String given is empty")
        } else {
            Ok(LexEngine {
                cur_ind,
                char_vec,
                cur_loc: (1, 1),
                errors: Vec::new(),
            })
        }
    }
    fn next_char(&mut self) -> Result<(), &'static str> {
        if self.cur_ind >= self.char_vec.len() - 1 {
            Err("no more chars")
        } else {
            self.cur_ind += 1;
            if self.cur_char() == '\n' {
                self.cur_loc.0 = 1;
                self.cur_loc.1 += 1;
            } else {
                self.cur_loc.0 += 1;
            }
            Ok(())
        }
    }
    fn cur_char(&self) -> char {
        self.char_vec[self.cur_ind]
    }
    pub fn next_tok(&mut self) -> Option<(Token, Attr, (usize, usize))> {
        while self.cur_char().is_ascii_whitespace() {
            if self.next_char().is_err() {
                return None;
            }
        }
        // check the nonterms and terms first
        if Self::is_lower_letter(self.cur_char()) {
            let mut temp_str = String::new();
            temp_str.push(self.cur_char());
            let start_loc = self.cur_loc;
            self.next_char();
            while Self::is_lower_letter(self.cur_char()) || self.cur_char() == '_' {
                temp_str.push(self.cur_char());
                self.next_char();
                // add error checking if reach end of file
            }
            if temp_str == String::from("null") {
                return Some((Token::NullVal, Attr::Skip, start_loc));
            } else {
                return Some((Token::Term, Attr::AttrString(temp_str), start_loc));
            }
        }
        if Self::is_upper_letter(self.cur_char()) {
            let mut temp_str = String::new();
            temp_str.push(self.cur_char());
            let start_loc = self.cur_loc;
            self.next_char();
            while Self::is_upper_letter(self.cur_char()) || self.cur_char() == '_' {
                temp_str.push(self.cur_char());
                self.next_char();
                // add error checking if reach end of file
            }
            return Some((Token::Nonterm, Attr::AttrString(temp_str), start_loc));
        }
        // now the small amount of symbols
        if self.cur_char() == ';' {
            return Some((Token::Semicolon, Attr::Skip, self.cur_loc));
        }
        if self.cur_char() == '|' {
            return Some((Token::Bar, Attr::Skip, self.cur_loc));
        }
        if self.cur_char() == '$' {
            return Some((Token::EndOfInput, Attr::Skip, self.cur_loc));
        }
        // finally the 'produces' symbol
        if self.cur_char() == ':' {
            let start_loc = self.cur_loc;
            // error checking at all the panics here
            self.next_char();
            if self.cur_char() == ':' {
                self.next_char();
                if self.cur_char() == '=' {
                    return Some((Token::Produces, Attr::Skip, start_loc));
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }

        // put the errorchecking here too
        panic!()
    }
    fn is_lower_letter(c: char) -> bool {
        c >= 'a' && c <= 'z'
    }
    fn is_upper_letter(c: char) -> bool {
        c >= 'A' && c <= 'Z'
    }
}
#[derive(Clone)]
pub enum Token {
    Nonterm,
    Term,
    Semicolon,
    Produces,
    Bar,
    NullVal,
    EndOfInput,
}

#[derive(Clone)]
pub enum Attr {
    Skip,
    AttrString(String),
}

pub struct LexError {
    loc: (usize, usize),
    message: &'static str,
}
