use std::cell::RefCell;
use std::rc::Rc;

type MoltList = Vec<Datum>;

struct MyValue {
    string_rep: RefCell<Option<Rc<String>>>,
}

struct MyValue2 {
    string_rep: String,
}

struct MyValue3 {
    string_rep: Option<Rc<String>>,
}

impl MyValue {
    fn new(str: String) -> Self {
        Self {
            string_rep: RefCell::new(Some(Rc::new(str))),
        }
    }
}

#[derive(Clone, Debug)]
enum Datum {
    Str(Rc<String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_str1(datum: &Datum) -> &str {
        match datum {
            Datum::Str(rcval) => {
                rcval
            }
        }
    }

    fn get_str2(myval: &MyValue2) -> &str {
        &myval.string_rep
    }

    // fn get_str3(myval: &MyValue3) -> &str {
    //     // Guaranteed to be Some
    //     &myval.string_rep.unwrap()
    // }

    #[test]
    fn from_to() {
        let d = Datum::Str(Rc::new("abc".to_string()));

        let str1 = get_str1(&d);
        assert_eq!(str1, "abc");

        let str2 = get_str1(&d);
        assert_eq!(str1, str2);
    }
}
