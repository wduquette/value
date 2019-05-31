use std::rc::Rc;

type MyString = Rc<String>;
type MyList = Rc<Vec<MyValue>>;

#[derive(Clone,Debug,PartialEq)]
enum Datum {
    Int(i64),
    Flt(f64),
    List(MyList),
}

#[derive(Clone,Debug,PartialEq)]
struct MyValue {
    string_rep: Option<MyString>,
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

    // A new value; fills in the string_rep if it was empty.
    pub fn from_data(data: MyValue) -> MyValue {
        MyValue {
            string_rep: Some(data.to_string()),
            data_rep: data.data_rep.clone(),
        }
    }


    // A new value, (none,list)
    pub fn from_list(list: Vec<MyValue>) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::List(Rc::new(list))),
        }
    }

    // A new value, (none,int)
    pub fn from_int(int: i64) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::Int(int)),
        }
    }

    // A new value, (none,float)
    pub fn from_float(flt: f64) -> MyValue {
        MyValue {
            string_rep: None,
            data_rep: Some(Datum::Flt(flt)),
        }
    }

    pub fn to_string(&self) -> MyString {
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

    // Not quite right.  If there's no datum, should try to parse string.
    pub fn to_int(&self) -> Result<i64,String> {
        if let Some(Datum::Int(int)) = self.data_rep {
            Ok(int)
        } else {
            Err("Not an integer".to_string())
        }
    }

    // Not quite right.  If there's no datum, should try to parse string.
    pub fn to_flt(&self) -> Result<f64,String> {
        if let Some(Datum::Flt(flt)) = self.data_rep {
            Ok(flt)
        } else {
            Err("Not a float".to_string())
        }
    }

    // Not quite right.  If there's no datum, should try to parse string.
    pub fn to_list(&self) -> Result<MyList,String> {
        if let Some(Datum::List(list)) = &self.data_rep {
            Ok(list.clone())
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

        assert_eq!(val, val2);
    }

    #[test]
    fn from_to_int() {
        let val = MyValue::from_int(5);
        assert_eq!(*val.to_string(), "5".to_string());
        assert_eq!(val.to_int(), Ok(5));
        assert_eq!(val.to_flt(), Err("Not a float".to_string()));
    }

    #[test]
    fn from_to_flt() {
        let val = MyValue::from_float(12.5);
        assert_eq!(*val.to_string(), "12.5".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_flt(), Ok(12.5));
    }
}
