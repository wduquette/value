fn main() {
    println!("Hello, world!");
}
use std::any::TypeId;
use std::any::Any;

trait MoltType {
    fn to_string(&self) -> String;
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MoltPair {
    f1: i32,
    f2: i32,
}

impl MoltType for MoltPair {
    fn to_string(&self) -> String {
        format!("({},{})", self.f1, self.f2)
    }

}

#[allow(dead_code)] // Temp
struct MoltValue {
    string_rep: Option<String>,
    // any_rep: Box<dyn Any>,
    int_rep: Option<Box<dyn MoltType>>,
}

#[allow(clippy::wrong_self_convention)]
impl MoltValue {
    fn to_string(&mut self) -> String {
        if self.string_rep.is_some() {
            self.string_rep.as_ref().unwrap().to_string()
        } else if self.int_rep.is_some() {
            let string_val = self.int_rep.as_ref().unwrap().to_string();

            self.string_rep = Some(string_val.clone());
            string_val
        } else {
            self.string_rep = Some(String::new());
            String::new()
        }
    }
}

fn get_pair(val: &'_ MoltValue) -> Result<&'_ MoltPair, String> {
    let value_any = val.int_rep.as_ref().unwrap() as &Any;

    // Cannot seem to downcast to MoltPair, probably because the original
    // is a MoltType trait object.
    match value_any.downcast_ref::<MoltPair>() {
        Some(mv) => {
            Ok(mv)
        }
        None => {
            Err("no can do!".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let pair = MoltPair { f1: 1, f2: 2};

        let mut mv = MoltValue {
            string_rep: None,
            int_rep: Some(Box::new(pair)),
        };

        let str = mv.to_string();
        assert_eq!(str, "(1,2)");

        let result = get_pair(&mv);
        println!("pair result={:?}", result);

        match result {
            Ok(pair) => {
                assert_eq!(pair.f1, 1);
                assert_eq!(pair.f2, 2);
            }
            _ => {
                assert!(false);
            }
        }
    }
}
