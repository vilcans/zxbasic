use keywords::Keyword;

mod keywords;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    String(Vec<u8>),
    Number(f64),
    Symbol(Vec<u8>),
}
