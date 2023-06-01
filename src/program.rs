//! Functionality for managing complete lines of BASIC code.

use crate::Token;

/// Create data for a BASIC line from raw data.
pub fn raw_line(number: u16, content: &[u8]) -> Vec<u8> {
    let mut data = Vec::with_capacity(content.len() + 5);
    let len = content.len() + 1;
    data.extend_from_slice(&[
        (number >> 8) as u8,
        number as u8,
        len as u8,
        (len >> 8) as u8,
    ]);
    data.extend_from_slice(content);
    data.push(0x0d);
    data
}

pub fn line(number: u16, tokens: &[Token]) -> Vec<u8> {
    let mut data = Vec::new();
    for token in tokens {
        data.extend(token.to_bytes().into_iter());
    }
    raw_line(number, &data)
}
