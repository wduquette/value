use std::str::FromStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

//-----------------------------------------------------------------------------
// The MyAny Trait and MyWrapper: a tool for handling external types.
// A MyWrapper<T> can be saved as a dyn MyAny.

pub trait MyAny: Any + Display + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + Display + Debug> MyAny for T {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
}

// Not clear that MyWrapper is needed for what I'm trying to do.
#[derive(Debug)]
pub struct MyWrapper<T: ?Sized + std::fmt::Display>(T);

impl<T: 'static + std::fmt::Display> std::fmt::Display for MyWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

//-----------------------------------------------------------------------------
// Datum enum: a sum type for the different kinds of data_reps.

pub type MyList = Vec<MyValue>;

#[derive(Clone,Debug)]
enum Datum {
    Int(i64),
    Flt(f64),
    List(Rc<MyList>),

    // What I really want here is a MyAny, which happens to be an Rc<T>.
    // Could I use a Box instead?
    Other(Rc<MyAny>),
    None
}

// TODO: needs to provide standard TCL list output.
impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Datum::Int(int) => write!(f, "{}", int),
            Datum::Flt(flt) => write!(f, "{}", flt),
            Datum::List(_) => write!(f, "*FAKE LIST*"),
            Datum::Other(other) => write!(f, "{}", other),
            Datum::None => write!(f, ""),
        }
    }
}

#[derive(Clone,Debug)]
pub struct MyValue {
    string_rep: RefCell<Option<Rc<String>>>,
    data_rep: RefCell<Datum>,
}

impl MyValue {
    // A new value (string,none)
    pub fn from_string(str: &str) -> MyValue {
        MyValue {
            string_rep: RefCell::new(Some(Rc::new(str.to_string()))),
            data_rep: RefCell::new(Datum::None),
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
            Datum::Int(int) => Rc::new(int.to_string()),
            Datum::Flt(flt) => Rc::new(flt.to_string()),
            _ =>  Rc::new("".to_string()),
        };

        *string_ref = Some(new_string.clone());

        new_string
    }

    // A new value, (none,int)
    pub fn from_int(int: i64) -> MyValue {
        MyValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Int(int)),
        }
    }

    // Tries to return the value as an int.
    // * Returns the data_rep if it can.
    // * Otherwise, produces a string_rep if there isn't one.
    // * Tries to parse the string_rep as an int.
    // * Saves a new data_rep on success.
    // * Returns an error on failure.
    pub fn to_int(&self) -> Result<i64,String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let mut string_ref = self.string_rep.borrow_mut();

        // FIRST, if we have an integer return it.
        if let Datum::Int(int) = *data_ref {
            return dbg!(Ok(int));
        }

        // NEXT, if we don't have a string_rep, get one.
        if (*string_ref).is_none() {
            *string_ref = Some(Rc::new(data_ref.to_string()));
        }

        // NEXT, if we have a string_rep, try to parse it as an integer
        if let Some(str) = &*string_ref {
            if let Ok(int) = str.parse::<i64>() {
                *data_ref = Datum::Int(int);
                return Ok(int);
            }
        }

        // NEXT, nothing worked.
        Err("Not an integer".to_string())
    }

    // A new value, (none,float)
    pub fn from_float(flt: f64) -> MyValue {
        MyValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Flt(flt)),
        }
    }

    // Tries to return the value as a float.
    // * Returns the data_rep if it can.
    // * Otherwise, produces a string_rep if there isn't one.
    // * Tries to parse the string_rep as a float.
    // * Saves a new data_rep on success.
    // * Returns an error on failure.
    pub fn to_float(&self) -> Result<f64,String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let mut string_ref = self.string_rep.borrow_mut();

        // FIRST, if we have a float, return it.
        if let Datum::Flt(flt) = *data_ref {
            return Ok(flt);
        }

        // NEXT, if we don't have a string_rep, get one.
        if (*string_ref).is_none() {
            *string_ref = Some(Rc::new(data_ref.to_string()));
        }

        // NEXT, if we have a string rep, try to parse it as a float
        if let Some(str) = &*string_ref {
            if let Ok(flt) = str.parse::<f64>() {
                *data_ref = Datum::Flt(flt);
                return Ok(flt);
            }
        }

        // NEXT, nothing worked.
        Err("Not a float".to_string())
    }

    // A new value, (none,list)
    pub fn from_list(list: MyList) -> MyValue {
        MyValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::List(Rc::new(list))),
        }
    }

    // Incomplete: should try to parse the string_rep, if any, as a list.  But I don't
    // have a list parser in this project.
    pub fn to_list(&self) -> Result<Rc<MyList>,String> {
        let data_ref = self.data_rep.borrow_mut();

        if let Datum::List(list) = &*data_ref {
            Ok(list.clone())
        // } else if let Some(_str) = &self.string_rep {
        //     panic!("list string_rep not defined!");
        } else {
            Err("Not a list".to_string())
        }
    }

    pub fn from_other<T: 'static>(value: T) -> MyValue
        where T: Display + Debug
    {
        MyValue {
            string_rep: RefCell::new(None),
            // Use Rc<Rc<T>> === Rc<MyAny>, so that Datum is known to be
            // clonable and the user's data is efficiently clonable and shareable.
            data_rep: RefCell::new(Datum::Other(Rc::new(Rc::new(value))))
        }
    }

    // NOTE: This should possibly return Option<Rc<T>> rather than Result:
    // External types should usually wrap this call, and will want to provide
    // their own appropriate error message.  (This method doesn't know what
    // to call type T in the error message.)
    pub fn to_other<T: 'static>(&self) -> Result<Rc<T>, String>
        where T: Display + Debug + FromStr
    {
        let mut string_ref = self.string_rep.borrow_mut();
        let mut data_ref = self.data_rep.borrow_mut();

        // FIRST, if we have the desired type, return it.
        if let Datum::Other(other) = &*data_ref {
            // other is an &Rc<MyAny>
            let result = (**other).as_any().downcast_ref::<Rc<T>>();

            if result.is_some() {
                // Let's be sure we're really getting what we wanted.
                let out: Rc<T> = result.unwrap().clone();
                return Ok(out);
            }
        }

        // NEXT, if we don't have a string_rep, get one.
        if (*string_ref).is_none() {
            *string_ref = Some(Rc::new(data_ref.to_string()));
        }

        // NEXT, can we parse it as a T?  If so, save it back to
        // the data_rep, and return it.
        if let Some(str) = &*string_ref {
            if let Ok(tval) = str.parse::<T>() {
                let tval = Rc::new(tval);
                let out = tval.clone();
                *data_ref = Datum::Other(Rc::new(tval));
                return Ok(out);
            }
        }

        // NEXT, we couldn't do it; return an error.
        Err("TODO, not a T".to_string())
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

        // TODO: Note, 7.0 might not get converted to "7" long term.
        // In Standard TCL, its string_rep would be "7.0".  Need to address
        // MoltFloat formatting/parsing.
        let val = MyValue::from_float(7.0);
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

        let val = MyValue::from_int(5);
        assert_eq!(val.to_float(), Ok(5.0));

        let val = MyValue::from_string("abc");
        assert_eq!(val.to_float(), Err("Not a float".to_string()));
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

    use crate::rgb::RGB;

    #[test]
    fn from_to_rgb() {
        let rgb = RGB::new(1,2,3);
        let myval = MyValue::from_other(rgb);

        // Get it back as Rc<RGB>
        let result = myval.to_other::<RGB>();
        assert!(result.is_ok());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);

        let myval = MyValue::from_string("#010203");
        let result = myval.to_other::<RGB>();
        assert!(result.is_ok());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);

    }

    fn get_rgb(value: & dyn MyAny) -> Option<&RGB> {
        let myval = value.as_any().downcast_ref::<MyWrapper<RGB>>();
        match myval {
            Some(MyWrapper(rgb)) => Some(rgb),
            _ => None
        }
    }

    #[test]
    fn using_wrapper() {
        let x: MyWrapper<RGB> = MyWrapper(RGB::new(1,2,3));
        let a: &dyn MyAny = &x;
        assert_eq!(a.to_string(), "#010203".to_string());

        let myvar = a.as_any().downcast_ref::<MyWrapper<RGB>>().unwrap();
        assert_eq!(myvar.to_string(), "#010203".to_string());

        assert_eq!(myvar.0.to_string(), "#010203".to_string());

        let a: &dyn MyAny = &x;
        let rgb = get_rgb(a).unwrap();
        assert_eq!(rgb, &RGB::new(1,2,3));
    }

}
