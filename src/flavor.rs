use crate::value::MoltValue;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Flavor {
    SALTY,
    SWEET,
}

impl Flavor {
    // TODO: The error should be a Molt ResultCode.
    pub fn from_molt(value: &MoltValue) -> Result<Rc<Self>, String> {
        if let Some(x) = value.as_other::<Flavor>() {
            Ok(x)
        } else {
            Err("Not a flavor string".to_string())
        }
    }
}

impl FromStr for Flavor {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if value == "salty" {
            Ok(Flavor::SALTY)
        } else if value == "sweet" {
            Ok(Flavor::SWEET)
        } else {
            Err("Not a flavor string".to_string())
        }
    }
}

impl fmt::Display for Flavor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Flavor::SALTY {
            write!(f, "salty")
        } else {
            write!(f, "sweet")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        assert_eq!(Flavor::from_str("Salty"), Ok(Flavor::SALTY));
        assert_eq!(str::parse::<Flavor>("sweet"), Ok(Flavor::SWEET));
        assert_eq!(Flavor::SALTY.to_string(), "salty".to_string());
    }

    #[test]
    fn from_molt() {
        let value = MoltValue::from_other(Flavor::SALTY);

        let out = Flavor::from_molt(&value);

        assert_eq!(*(out.unwrap()), Flavor::SALTY);
    }
}
