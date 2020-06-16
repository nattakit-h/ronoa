/*
 * Copyright (c) 2020 Nattakit Hosapsin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenKind
{
    Newline,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Ampersand,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Equal,
    EqualEqual,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
    Bang,
    BangEqual,
    Arrow,
    Identifier,
    Int,
    Float,
    String,
    If,
    Else,
    For,
    Public,
    Ref,
    Const,
    Mutate,
    Let,
    Var,
    Fun,
    Return,
}

impl fmt::Display for TokenKind
{
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result
    {
        return write!(formater, "{:?}", self);
    }
}

pub struct Token
{
    pub kind: TokenKind,
    pub data: String,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Token
{
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result
    {
        let value;
        if let TokenKind::String = self.kind {
            value = format!("\"{}\"", &self.data);
            return write!(
                formater,
                "{:12} {:20} {}:{}",
                self.kind.to_string(),
                value,
                self.row,
                self.col
            );
        } else {
            return write!(
                formater,
                "{:12} {:20} {}:{}",
                self.kind.to_string(),
                self.data,
                self.row,
                self.col
            );
        }
    }
}
