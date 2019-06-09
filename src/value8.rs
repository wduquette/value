use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

//-----------------------------------------------------------------------------
// Public Data Types

// The standard Molt data types.
// Provide doc comments.
pub type MoltList = Vec<MoltValue>;
pub type MoltInt = i64;
pub type MoltFloat = f64;

/// The standard Molt value representation.  Variable values and list elements
/// are MoltValues.
///
/// TODO: Define other needed traits.  At least define Hash, which would hash on
/// the string_rep.

#[derive(Clone, Debug)]
pub struct MoltValue {
    string_rep: RefCell<Option<Rc<String>>>,
    data_rep: RefCell<Datum>,
}

// The data representation for MoltValues that define data
// TODO: flesh out comments.
#[derive(Clone, Debug)]
enum Datum {
    Int(MoltInt),
    Flt(MoltFloat),
    List(Rc<MoltList>),
    Other(Rc<MoltAny>),
    None,
}

impl Display for MoltValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // FIRST, if there's already a string, return it.
        let mut string_ref = self.string_rep.borrow_mut();

        if let Some(str) = &*string_ref {
            return write!(f, "{}", str);
        }

        // NEXT, if there's no string there must be data.  Convert the data to a string,
        // and save it for next time.
        let data_ref = self.data_rep.borrow();
        let new_string = match *data_ref {
            Datum::Int(int) => Rc::new(int.to_string()),
            Datum::Flt(flt) => Rc::new(flt.to_string()),
            _ => Rc::new("".to_string()),
        };

        *string_ref = Some(new_string.clone());

        return write!(f, "{}", new_string);
    }
}

impl MoltValue {
    /// Creates a new MoltValue from the given string.
    ///
    /// Note: takes a String rather than a &str because the whole point is to
    /// take ownership and create a reference-counted immutable string.  If
    /// the method took &str, then a newly created String might get cloned.
    pub fn from_string(str: String) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(Some(Rc::new(str))),
            data_rep: RefCell::new(Datum::None),
        }
    }

    /// Creates a new MoltValue whose data representation is a MoltInt.
    pub fn from_int(int: MoltInt) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Int(int)),
        }
    }

    /// Tries to return the MoltValue as an int.  This method will:
    ///
    /// * Return the data representation if it is already a MoltInt.
    /// * If there is no current string representation, produce one from the
    ///   current data representation.
    /// * Try to parse the string representation as an int.
    /// * On success, save the int as the new data representation
    /// * On failure, return an error.
    ///
    /// TODO: Need to return Molt-compatible Err's.
    pub fn to_int(&self) -> Result<MoltInt, String> {
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
            // TODO: Uses standard Rust integer parsing.  Need to use the
            // TCL algorithm; see Interp::get_int.
            if let Ok(int) = str.parse::<MoltInt>() {
                *data_ref = Datum::Int(int);
                return Ok(int);
            }
        }

        // NEXT, nothing worked.
        // TODO: Use the correct error message.
        Err("Not an integer".to_string())
    }

    /// Creates a new MoltValue whose data representation is a MoltFloat.
    pub fn from_float(flt: MoltFloat) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Flt(flt)),
        }
    }

    /// Tries to return the MoltValue as an float.  This method will:
    ///
    /// * Return the data representation if it is already a MoltFloat.
    /// * If there is no current string representation, produce one from the
    ///   current data representation.
    /// * Try to parse the string representation as a float.
    /// * On success, save the float as the new data representation
    /// * On failure, return an error.
    ///
    /// TODO: Need to return Molt-compatible Err's.
    pub fn to_float(&self) -> Result<MoltFloat, String> {
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
            // TODO: Currently uses the standard Rust parser.  That may
            // be OK, but I need to check.
            if let Ok(flt) = str.parse::<MoltFloat>() {
                *data_ref = Datum::Flt(flt);
                return Ok(flt);
            }
        }

        // NEXT, nothing worked.
        // TODO: need to use the right error message.
        Err("Not a float".to_string())
    }

    /// Creates a new MoltValue whose data representation is a MoltList.
    pub fn from_list(list: MoltList) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::List(Rc::new(list))),
        }
    }

    // Incomplete: should try to parse the string_rep, if any, as a list.  But I don't
    // have a list parser in this project.
    pub fn to_list(&self) -> Result<Rc<MoltList>, String> {
        let data_ref = self.data_rep.borrow_mut();

        if let Datum::List(list) = &*data_ref {
            Ok(list.clone())
        // } else if let Some(_str) = &self.string_rep {
        //     panic!("list string_rep not defined!");
        } else {
            Err("Not a list".to_string())
        }
    }

    /// Creates a new MoltValue containing the given value of a user type.
    /// The type must implement Display and FromStr, and the Display result
    /// must be compatible with the FromStr parsing (and with TCL syntax),
    /// i.e., if there's whitespace the value must be interpretable as a
    /// a TCL list.
    pub fn from_other<T: 'static>(value: T) -> MoltValue
    where
        T: Display + Debug,
    {
        MoltValue {
            string_rep: RefCell::new(None),
            // Use Rc<Rc<T>> === Rc<MoltAny>, so that Datum is known to be
            // clonable and the user's data is efficiently clonable and shareable.
            data_rep: RefCell::new(Datum::Other(Rc::new(Rc::new(value)))),
        }
    }

    /// Tries to interpret the MoltValue as a value of type `Rc<T>`.
    ///
    /// The value is returned as an `Rc<T>`, as this allows the client to
    /// use the value freely.
    ///
    /// This method returns `Option` rather than `Result` because it is up
    /// to the caller to provide a meaningful error message: the method
    /// doesn't know what to call the type.
    ///
    /// When adding an external type `MyType` to Molt, it is usual to define
    /// functions `from_my_type` and `to_my_type` that make use of `from_other`
    /// and `to_other`.
    pub fn to_other<T: 'static>(&self) -> Option<Rc<T>>
    where
        T: Display + Debug + FromStr,
    {
        let mut string_ref = self.string_rep.borrow_mut();
        let mut data_ref = self.data_rep.borrow_mut();

        // FIRST, if we have the desired type, return it.
        if let Datum::Other(other) = &*data_ref {
            // other is an &Rc<MoltAny>
            let result = (**other).as_any().downcast_ref::<Rc<T>>();

            if result.is_some() {
                // Let's be sure we're really getting what we wanted.
                let out: Rc<T> = result.unwrap().clone();
                return Some(out);
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
                return Some(out);
            }
        }

        // NEXT, we couldn't do it.
        None
    }
}

//-----------------------------------------------------------------------------
// The MoltAny Trait: a tool for handling external types.

trait MoltAny: Any + Display + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + Display + Debug> MoltAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

//-----------------------------------------------------------------------------
// Datum enum: a sum type for the different kinds of data_reps.

// TODO: needs to provide standard TCL list and float output.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(*val.to_string(), "abc".to_string());

        let val2 = val.clone();
        assert_eq!(*val.to_string(), *val2.to_string());
    }

    #[test]
    fn from_to_int() {
        let val = MoltValue::from_int(5);
        assert_eq!(*val.to_string(), "5".to_string());
        assert_eq!(val.to_int(), Ok(5));
        assert_eq!(val.to_float(), Ok(5.0));

        let val = MoltValue::from_string("7".to_string());
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.to_int(), Ok(7));
        assert_eq!(val.to_float(), Ok(7.0));

        // TODO: Note, 7.0 might not get converted to "7" long term.
        // In Standard TCL, its string_rep would be "7.0".  Need to address
        // MoltFloat formatting/parsing.
        let val = MoltValue::from_float(7.0);
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.to_int(), Ok(7));
        assert_eq!(val.to_float(), Ok(7.0));

        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
    }

    #[test]
    fn from_to_float() {
        let val = MoltValue::from_float(12.5);
        assert_eq!(*val.to_string(), "12.5".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_float(), Ok(12.5));

        let val = MoltValue::from_string("7.8".to_string());
        assert_eq!(*val.to_string(), "7.8".to_string());
        assert_eq!(val.to_int(), Err("Not an integer".to_string()));
        assert_eq!(val.to_float(), Ok(7.8));

        let val = MoltValue::from_int(5);
        assert_eq!(val.to_float(), Ok(5.0));

        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(val.to_float(), Err("Not a float".to_string()));
    }

    #[test]
    fn from_to_list() {
        let a = MoltValue::from_string("abc".to_string());
        let b = MoltValue::from_float(12.5);
        let listval = MoltValue::from_list(vec![a.clone(), b.clone()]);

        // Get it back as Rc<MoltList>
        let result = listval.to_list();

        assert!(result.is_ok());

        if let Ok(rclist) = result {
            assert_eq!(rclist.len(), 2);
            assert_eq!(rclist[0].to_string(), a.to_string());
            assert_eq!(rclist[1].to_string(), b.to_string());
        }
    }

    // TODO: Replace RGB with a simpler type defined here in the test module.
    use crate::rgb::RGB;

    #[test]
    fn from_to_rgb() {
        let rgb = RGB::new(1, 2, 3);
        let myval = MoltValue::from_other(rgb);

        // Get it back as Rc<RGB>
        let result = myval.to_other::<RGB>();
        assert!(result.is_some());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);

        let myval = MoltValue::from_string("#010203".to_string());
        let result = myval.to_other::<RGB>();
        assert!(result.is_some());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);
    }
}
