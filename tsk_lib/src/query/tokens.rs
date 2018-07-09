// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


use chrono::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    GT,
    LT,
    GTE,
    LTE,
    EQ,
    NE,

    AND,
    OR,

    LP,
    RP,

    EOF,

    Str(String),
    Float(f64),
    Date(DateTime<Local>),

    Unexpected(String),
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        match c {
            '(' => Token::LP,
            ')' => Token::RP,
            '>' => Token::GT,
            '<' => Token::LT,
            '=' => Token::EQ,
            _ => Token::Unexpected(c.to_string()),
        }
    }
}

impl<'a> From<&'a str> for Token {
    fn from(s: &str) -> Token {
        if let Ok(num) = s.parse::<f64>() {
            return Token::Float(num);
        }

        if let Ok(date) = Local.datetime_from_str(s, "%Y-%m-%d %r") {
            return Token::Date(date);
        }

        match s {
            ">=" => Token::GTE,
            "<=" => Token::LTE,
            "" => Token::EOF,
            "EOF" => Token::EOF,
            "AND" | "and" => Token::AND,
            "OR" | "or" => Token::OR,
            _ => Token::Str(s.to_string()),
        }
    }
}

impl From<DateTime<Local>> for Token {
    fn from(dte: DateTime<Local>) -> Token {
        Token::Date(dte)
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Str(s) => format!("(String, {})", s),
            Token::Date(d) => format!("(Date, {})", d),
            Token::Float(num) => format!("(Float, {})", num),

            Token::GT => "(GT, >)".to_string(),
            Token::LT => "(LT, <)".to_string(),
            Token::GTE => "(GTE, >=)".to_string(),
            Token::LTE => "(LTE, <=)".to_string(),
            Token::EQ => "(EQ, =)".to_string(),
            Token::NE => "(NE, !=)".to_string(),

            Token::AND => "(AND, AND)".to_string(),
            Token::OR => "(OR, OR)".to_string(),

            Token::LP => "(LP, '(')".to_string(),
            Token::RP => "(RP, ')')".to_string(),

            Token::EOF => "(EOF, EOF)".to_string(),
            Token::Unexpected(c) => format!("(Unexpected, {})", c),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token: {}", self.to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Token::from(">="), Token::GTE);
        assert_eq!(Token::from("<="), Token::LTE);
        assert_eq!(Token::from(""), Token::EOF);
        assert_eq!(Token::from("EOF"), Token::EOF);
        assert_eq!(Token::from("AND"), Token::AND);
        assert_eq!(Token::from("and"), Token::AND);
        assert_eq!(Token::from("OR"), Token::OR);
        assert_eq!(Token::from("or"), Token::OR);
        assert_eq!(Token::from("1.0"), Token::Float(1.0));
        assert_eq!(Token::from("5"), Token::Float(5.0));
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Token::from('('), Token::LP);
        assert_eq!(Token::from(')'), Token::RP);
        assert_eq!(Token::from('>'), Token::GT);
        assert_eq!(Token::from('<'), Token::LT);
        assert_eq!(Token::from('='), Token::EQ);
        assert_eq!(Token::from('*'), Token::Unexpected("*".to_string()));
    }
}