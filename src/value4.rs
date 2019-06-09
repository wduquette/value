use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type MyHashMap = HashMap<MyValue, MyValue>;

type MyList = Vec<MyValue>;

#[derive(Clone, Debug)]
enum Datum {
    Int(i64),
    Flt(f64),
}

#[derive(Clone, Debug)]
pub struct MyValue {
    string_rep: RefCell<Option<Rc<String>>>,
    data_rep: RefCell<Option<Datum>>,
}

impl MyValue {
    // A new value (string,none)
    pub fn from_string(str: &str) -> MyValue {
        MyValue {
            string_rep: RefCell::new(Some(Rc::new(str.to_string()))),
            data_rep: RefCell::new(None),
        }
    }

    pub fn to_string(&self) -> Rc<String> {
        // FIRST, if there's already a string, return it.
        let mut string_ref = self.string_rep.borrow_mut();

        if let Some(str) = &*string_ref {
            return str.clone();
        }

        // NEXT, if there's no string there must be data.  Convert the data to a string,
        // and save it for next time.
        let data_ref = self.data_rep.borrow();
        let new_string = match *data_ref {
            Some(Datum::Int(int)) => Rc::new(int.to_string()),
            Some(Datum::Flt(flt)) => Rc::new(flt.to_string()),
            _ => Rc::new("".to_string()),
        };

        *string_ref = Some(new_string.clone());

        new_string
    }

    // A new value, (none,int)
    pub fn from_int(int: i64) -> MyValue {
        MyValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Some(Datum::Int(int))),
        }
    }

    // Tries to return the value as an int
    pub fn to_int(&self) -> Result<i64, String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let string_ref = self.string_rep.borrow();

        if let Some(Datum::Int(int)) = *data_ref {
            Ok(int)
        } else if let Some(str) = &*string_ref {
            match str.parse::<i64>() {
                Ok(int) => {
                    *data_ref = Some(Datum::Int(int));
                    Ok(int)
                }
                Err(_) => Err("Not an integer".to_string()),
            }
        } else {
            Err("Not an integer".to_string())
        }
    }

    // A new value, (none,float)
    pub fn from_float(flt: f64) -> MyValue {
        MyValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Some(Datum::Flt(flt))),
        }
    }

    // Tries to return the value as a float
    pub fn to_float(&self) -> Result<f64, String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let string_ref = self.string_rep.borrow();

        if let Some(Datum::Flt(flt)) = *data_ref {
            Ok(flt)
        } else if let Some(str) = &*string_ref {
            match str.parse::<f64>() {
                Ok(flt) => {
                    *data_ref = Some(Datum::Flt(flt));
                    Ok(flt)
                }
                Err(_) => Err("Not a float".to_string()),
            }
        } else {
            Err("Not a float".to_string())
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
        assert_eq!(val.to_float(), Ok(5.0));

        let val = MyValue::from_string("7");
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.to_int(), Ok(7));
        assert_eq!(val.to_float(), Ok(7.0));

        let val = MyValue::from_string("abc");
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
    }

    #[test]
    fn from_to_float() {
        let val = MyValue::from_float(12.5);
        assert_eq!(*val.to_string(), "12.5".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_float(), Ok(12.5));

        let val = MyValue::from_string("7.8");
        assert_eq!(*val.to_string(), "7.8".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_float(), Ok(7.8));

        let val = MyValue::from_string("abc");
        assert_eq!(val.to_float(), Err("Not a float".to_string()));
    }

}
