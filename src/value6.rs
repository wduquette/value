use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

//-----------------------------------------------------------------------------
// The MyAny Trait and MyWrapper: a tool for handling external types.
// A MyWrapper<T> can be saved as a dyn MyAny.

pub trait MyAny: Any + std::fmt::Display + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + std::fmt::Display + std::fmt::Debug> MyAny for T {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
}

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
    // Other(Rc<dyn MyAny>),
    None
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

    // Tries to return the value as an int
    pub fn to_int(&self) -> Result<i64,String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let string_ref = self.string_rep.borrow();

        if let Datum::Int(int) = *data_ref {
            Ok(int)
        } else if let Some(str) = &*string_ref {
            match str.parse::<i64>() {
                Ok(int) => {
                    *data_ref = Datum::Int(int);
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
            data_rep: RefCell::new(Datum::Flt(flt)),
        }
    }

    // Tries to return the value as a float
    pub fn to_float(&self) -> Result<f64,String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let string_ref = self.string_rep.borrow();

        if let Datum::Flt(flt) = *data_ref {
            Ok(flt)
        } else if let Some(str) = &*string_ref {
            match str.parse::<f64>() {
                Ok(flt) => {
                    *data_ref = Datum::Flt(flt);
                    Ok(flt)
                }
                Err(_) => Err("Not a float".to_string()),
            }
        } else {
            Err("Not a float".to_string())
        }
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


    // // A new value, (none,other)
    // // Hmmmm: not using MyWrapper.  Does that matter?
    // pub fn from_other(value: Rc<dyn MyAny>) -> MyValue {
    //     MyValue {
    //         string_rep: RefCell::new(None),
    //         data_rep: RefCell::new(Some(Datum::Other(value.clone()))),
    //     }
    // }

    // pub fn from_other<T: 'static +  Display + Debug>(value: Rc<MyWrapper<T>>) -> MyValue {
    //     MyValue {
    //         string_rep: RefCell::new(None),
    //         data_rep: RefCell::new(Some(Datum::Other(value)))
    //     }
    // }

    // Tries to return the value as an int
    // pub fn to_other<T: 'static>(&self) -> Result<&T,String> {
        // let mut data_ref = self.data_rep.borrow_mut();
        // // let string_ref = self.string_rep.borrow();
        //
        // if let Some(Datum::Other(other)) = &*data_ref {
        //     let other = other.clone();
        //     if let Some(myval) = other.as_any().downcast_ref::<T>() {
        //         return Ok(myval);
        //     }
        // }

        // if let Some(str) = &*string_ref {
        //     match str.parse::<i64>() {
        //         Ok(int) => {
        //             *data_ref = Some(Datum::Int(int));
        //             Ok(int)
        //         }
        //         Err(_) => Err("Not an integer".to_string()),
        //     }
        // }
    //
    //     Err("Could not retrieve".to_string())
    // }
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
