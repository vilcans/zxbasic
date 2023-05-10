use std::io::Write;

use crate::keywords::Keyword;
use zxnumber::ZXNumber;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    String(Vec<u8>),
    Number(f64),
    Symbol(Vec<u8>),
}

impl Token {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Token::Keyword(keyword) => vec![keyword.to_byte()],
            Token::String(s) => {
                let mut v = Vec::with_capacity(s.len() + 2);
                v.push(b'"');
                v.extend_from_slice(s);
                v.push(b'"');
                v
            }
            Token::Number(number) => {
                let zxnum = ZXNumber::from_f64(*number);
                let mut v = Vec::new();
                write!(v, "{number}").unwrap();
                v.push(0x0e);
                v.extend_from_slice(&zxnum.raw());
                v
            }
            Token::Symbol(s) => s.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keyword_to_bytes() {
        assert_eq!(Token::Keyword(Keyword::Beep).to_bytes(), &[215]);
    }

    #[test]
    fn test_string_to_bytes() {
        assert_eq!(
            Token::String(b"ABC".to_vec()).to_bytes(),
            &[34, 65, 66, 67, 34]
        );
    }

    #[test]
    fn test_number_to_bytes() {
        assert_eq!(
            Token::Number(4711.0).to_bytes(),
            &[0x34, 0x37, 0x31, 0x31, 0x0e, 0x00, 0x00, 0x67, 0x12, 0x00]
        );
    }

    #[test]
    fn test_symbol_to_bytes() {
        assert_eq!(
            Token::Symbol(b"abc".to_vec()).to_bytes(),
            &[0x61, 0x62, 0x63]
        );
    }
}
