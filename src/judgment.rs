// This code is copied from
// https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html, as
// a convenient example of FromStr, and then extended to provide fmt::Display as well.
// (with tests to make sure I didn't screw it up.)
use std::fmt;
use std::str::FromStr;
use std::rc::Rc;
use crate::value::MoltValue;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Judgment {
    GOOD,
    BAD
}

impl Judgment {
    // TODO: The error should be a Molt ResultCode.
    pub fn from_molt(value: &MoltValue) -> Result<Rc<Self>,String> {
        if let Some(x) = value.as_other::<Judgment>() {
            Ok(x)
        } else {
            Err("Not a Judgment string".to_string())
        }
    }
}

impl FromStr for Judgment {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if value == "good" {
            Ok(Judgment::GOOD)
        } else if value == "bad" {
            Ok(Judgment::BAD)
        } else {
            Err("Not a Judgment string".to_string())
        }
    }
}

impl fmt::Display for Judgment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Judgment::GOOD {
            write!(f, "good")
        } else {
            write!(f, "bad")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        assert_eq!(Judgment::from_str("Good"), Ok(Judgment::GOOD));
        assert_eq!(str::parse::<Judgment>("bad"), Ok(Judgment::BAD));
        assert_eq!(Judgment::GOOD.to_string(), "good".to_string());
    }

    #[test]
    fn from_molt() {
        let value = MoltValue::from_other(Judgment::GOOD); 

        let out = Judgment::from_molt(&value);

        assert_eq!(*(out.unwrap()), Judgment::GOOD);
    }
}
