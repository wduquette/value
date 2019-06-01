use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

type MyHashMap = HashMap<MyValue,MyValue>;

type MyList = Vec<MyValue>;

// When I add Other(Rc<Any>) I can no longer derive PartialEq; but that's probably right, as
// I need to compare for equality using to_string().
#[derive(Clone,Debug)]
enum Datum {
    Int(i64),
    Flt(f64),
    List(Rc<MyList>),
    Other(Rc<Any>),
}

#[derive(Clone,Debug)]
struct MyValue {
    string_rep: Option<Rc<String>>,
    data_rep: Option<Datum>,
}

impl MyValue {
    // A new value (string,none)
    pub fn from_string(str: &str) -> MyValue {
        MyValue {
            string_rep: Some(Rc::new(str.to_string())),
            data_rep: None,
        }
    }

    pub fn to_string(&self) -> Rc<String> {
        // FIRST, if there's already a string, return it.
        if let Some(str) = &self.string_rep {
            return str.clone();
        }

        // NEXT, if there's no string there must be data.
        match &self.data_rep {
            Some(Datum::Int(int)) => Rc::new(int.to_string()),
            Some(Datum::Flt(flt)) => Rc::new(flt.to_string()),
            _ =>  Rc::new("".to_string()),
        }
    }

    // A new value, (none,int)
    pub fn from_int(int: i64) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::Int(int)),
        }
    }

    // Tries to return the value as an int
    pub fn to_int(&self) -> Result<i64,String> {
        if let Some(Datum::Int(int)) = self.data_rep {
            Ok(int)
        } else if let Some(str) = &self.string_rep {
            match str.parse::<i64>() {
                Ok(int) => Ok(int),
                Err(_) => Err("Not an integer".to_string()),
            }
        } else {
            Err("Not an integer".to_string())
        }
    }

    // A new value, (none,float)
    pub fn from_float(flt: f64) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::Flt(flt)),
        }
    }

    // Tries to return the value as a float
    pub fn to_flt(&self) -> Result<f64,String> {
        if let Some(Datum::Flt(flt)) = self.data_rep {
            Ok(flt)
        } else if let Some(str) = &self.string_rep {
            match str.parse::<f64>() {
                Ok(flt) => Ok(flt),
                Err(_) => Err("Not a float".to_string()),
            }
        } else {
            Err("Not a float".to_string())
        }
    }

    // A new value, (none,list)
    pub fn from_list(list: MyList) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::List(Rc::new(list))),
        }
    }

    // Incomplete: should try to parse the string_rep, if any, as a list.  But I don't
    // have a list parser in this project.
    pub fn to_list(&self) -> Result<Rc<MyList>,String> {
        if let Some(Datum::List(list)) = &self.data_rep {
            Ok(list.clone())
        } else if let Some(_str) = &self.string_rep {
            // TODO: Fill this in on integration into Molt.  Possibly, make list a Datum::Other.
            panic!("list string_rep not defined!");
        } else {
            Err("Not a list".to_string())
        }
    }

    // A new value, (none,list)
    // How to write this?
    pub fn from_any(value: Rc<Any>) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::Other(value.clone())),
        }
    }

    // Incomplete: should try to parse the string_rep, if any, as a list.  But I don't
    // have a list parser in this project.
    pub fn to_any(&self) -> Result<Rc<Any>,String> {
        if let Some(Datum::Other(value)) = &self.data_rep {
            Ok(value.clone())
        } else if let Some(_str) = &self.string_rep {
            // TODO: Need to work out how to provide the translation for "Any" types.
            panic!("any string_rep not defined!");
        } else {
            Err("Not a list".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        let val = MyValue::from_string("abc");
        assert_eq!(*val.to_string(), "abc".to_string());

        let val2 = val.clone();
        assert_eq!(*val.to_string(), *val2.to_string());
    }

    #[test]
    fn from_to_int() {
        let val = MyValue::from_int(5);
        assert_eq!(*val.to_string(), "5".to_string());
        assert_eq!(val.to_int(), Ok(5));
        assert_eq!(val.to_flt(), Err("Not a float".to_string()));

        let val = MyValue::from_string("7");
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.to_int(), Ok(7));
        assert_eq!(val.to_flt(), Ok(7.0));

        let val = MyValue::from_string("abc");
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
    }

    #[test]
    fn from_to_flt() {
        let val = MyValue::from_float(12.5);
        assert_eq!(*val.to_string(), "12.5".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_flt(), Ok(12.5));

        let val = MyValue::from_string("7.8");
        assert_eq!(*val.to_string(), "7.8".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_flt(), Ok(7.8));

        let val = MyValue::from_string("abc");
        assert_eq!(val.to_flt(), Err("Not a float".to_string()));
    }

    #[test]
    fn from_to_list() {
        let a = MyValue::from_string("abc");
        let b = MyValue::from_float(12.5);
        let listval = MyValue::from_list(vec!(a.clone(), b.clone()));

        // Get it back as Rc<MyList>
        let result = listval.to_list();

        assert!(result.is_ok());

        if let Ok(rclist) = result {
            assert_eq!(rclist.len(), 2);
            assert_eq!(rclist[0].to_string(), a.to_string());
            assert_eq!(rclist[1].to_string(), b.to_string());
        }
    }

    struct Whatsit(i64);

    #[test]
    fn from_to_any() {
        let w = Whatsit(5);

        let a = MyValue::from_any(Rc::new(w));

        // Get it back as Rc<Any>
        let result = a.to_any();

        assert!(result.is_ok());

        if let Ok(rc_any) = result {
            match rc_any.downcast_ref::<Whatsit>() {
                Some(whatsit) => {
                    assert_eq!(whatsit.0, 5);
                }
                None => {
                    panic!("should have been an int");
                }
            }
        }
    }
}
