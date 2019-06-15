
//! The MoltValue Type
//! 
//! The [`MoltValue`] struct is the standard representation of a data value
//! in the Molt language.  It represents a single immutable data value; the 
//! data is reference-counted, so instances can be cloned efficiently.  The
//! data value can be any TCL data value: a number, a list, or any 
//! arbitrary type (that meets certain requirements).
//! 
//! [`MoltValue`]: struct.MoltValue.html

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;
use std::any::TypeId;

//-----------------------------------------------------------------------------
// Public Data Types

// The standard Molt data types.  These are already defined in the Molt code base;
// we'll need to update them there, but we don't need doc comments here.
pub type MoltList = Vec<MoltValue>;
pub type MoltInt = i64;
pub type MoltFloat = f64;

/// The `MoltValue` struct is the standard representation of a data value
/// in the Molt language.  It represents a single immutable data value; the 
/// data is reference-counted, so instances can be cloned efficiently.  The
/// data value can be any TCL data value: a number, a list, or any 
/// arbitrary type (that meets certain requirements).
/// 
/// In TCL "everything is a string"; thus, every `MoltValue` has a string
/// representation, or _string rep_.  But for efficiency with numbers, lists,
/// and user-defined binary data structures, the MoltValue also caches a
/// data representation, or _data rep_.
/// 
/// A `MoltValue` can have just a string rep, just a data rep, or both.
/// Like the `Tcl_Obj` in standard TCL, the `MoltValue` is like a stork: it
/// can stand one leg, the other leg, or both legs.
/// 
/// A client can ask the `MoltValue` for its string, which is always available
/// and will be computed from the data rep if it doesn't already exist.  (Once 
/// computed, the string rep never changes.)  A client can also ask
/// the `MoltValue` for any other type it desires.  If the requested data rep
/// is already available, it will be returned; otherwise, the `MoltValue` will
/// attempt to parse it from the string_rep.  The last successful conversion is 
/// cached for later.
/// 
/// For example, consider the following sequence: 
/// 
/// * A computation yields a `MoltValue` containing the integer 5. The data rep is
///   a `MoltInt`, and the string rep is undefined.
/// 
/// * The client asks for the string, and the string rep "5" is computed.
/// 
/// * The client asks for the value's integer value.  It's available and is returned.
/// 
/// * The client asks for the value's value as a MoltList.  This is possible, because
///   the string "5" can be interpreted as a list of one element, the 
///   string "5".  A new data rep is computed and saved, replacing the previous one.
/// 
/// With this scheme, long series of computations can be carried 
/// out efficiently using only the the data rep, incurring the parsing cost at most 
/// once, while preserving TCL's "everything is a string" semantics. 
/// 
/// Converting from one data rep to another is expensive, as it involves parsing
/// the string value.  Performance suffers when code switches rapidly from one data 
/// rep to another, e.g., in a tight loop.  The effect, which is known as "shimmering",
/// can usually be avoided with a little care.  
/// 
/// `MoltValue` handles strings, integers, floating-point values, and lists as
/// special cases, since they are part of the language and are so frequently used.
/// In addition, a `MoltValue` can also contain any Rust struct that meets
/// certain requirements.
/// 
/// # External Types
/// 
/// Any struct that implements the `std::fmt::Display`, `std::fmt::Debug`, 
/// and `std::str::FromStr` traits can be saved in a `MoltValue`.  The struct's
/// `Display` and `FromStr` trait implementations are used to do the string 
/// rep/data rep conversions.  In particular:
/// 
/// * The `Display` implementation is responsible for producing the value's string rep.
/// 
/// * The `FromStr` implementation is responsible for producing the value's data rep from
///   a string, and so must be able to parse the `Display` implementation's
///   output.
/// 
/// * The string rep should be chosen so as to fit in well with TCL syntax, lest
///   confusion, quoting hell, and comedy should ensue.  (You'll know it when you
///   see it.)
/// 
/// ## Example
/// 
/// For example, the following code shows how to define an external type implementing
/// a simple enum.
/// 
/// ```
/// ```
#[derive(Clone, Debug)]
pub struct MoltValue {
    string_rep: RefCell<Option<Rc<String>>>,
    data_rep: RefCell<Datum>,
}

impl Display for MoltValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl MoltValue {
    /// Creates a new `MoltValue` from the given string.
    ///
    /// **Note:** this method takes a `String` rather than a `&str` because the 
    /// intent is for the `MoltValue` to take ownership and create a reference-counted 
    /// immutable string from the input.  If the method took `&str` instead, it
    /// would have to clone its input in order to save it, which would very often
    /// result in cloning newly-created strings.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn from_string(str: String) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(Some(Rc::new(str))),
            data_rep: RefCell::new(Datum::None),
        }
    }
    /// Returns the value's string representation as a reference-counted
    /// string. 
    /// 
    /// **Note**: this is the standard way of retrieving a `MoltValue`'s 
    /// string rep, as unlike `to_string` it doesn't create a new `String`.
    /// 
    /// # Example
    /// 
    /// TODO
    fn as_string(&self) -> Rc<String> {
        // FIRST, if there's already a string, return it.
        let mut string_ref = self.string_rep.borrow_mut();

        if let Some(str) = &*string_ref {
            return Rc::clone(str);
        }

        // NEXT, if there's no string there must be data.  Convert the data to a string,
        // and save it for next time.
        let data_ref = self.data_rep.borrow();
        let new_string = Rc::new((*data_ref).to_string());

        *string_ref = Some(new_string.clone());

        new_string
    }

    /// Creates a new `MoltValue` whose data representation is a `MoltInt`.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn from_int(int: MoltInt) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Int(int)),
        }
    }

    /// Tries to return the `MoltValue` as a `MoltInt`, parsing the 
    /// value's string representation if necessary.
    ///
    /// TODO: Need to return Molt-compatible Err's.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn as_int(&self) -> Result<MoltInt, String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let mut string_ref = self.string_rep.borrow_mut();

        // FIRST, if we have an integer then just return it.
        if let Datum::Int(int) = *data_ref {
            return dbg!(Ok(int));
        }

        // NEXT, if we don't have a string_rep, get one from the current
        // data_rep.
        if (*string_ref).is_none() {
            *string_ref = Some(Rc::new(data_ref.to_string()));
        }

        // NEXT, Try to parse the string_rep as an integer
        if let Some(str) = &*string_ref {
            // TODO: Uses standard Rust integer parsing.  Need to use the
            // TCL algorithm; see Interp::get_int.
            if let Ok(int) = str.parse::<MoltInt>() {
                *data_ref = Datum::Int(int);
                return Ok(int);
            }
        }

        // NEXT, we can't interpret it as an integer; return an error.
        // TODO: Use the correct error message.
        Err("Not an integer".to_string())
    }

    /// Creates a new `MoltValue` whose data representation is a `MoltFloat`.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn from_float(flt: MoltFloat) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Flt(flt)),
        }
    }

    /// Tries to return the `MoltValue` as a `MoltFloat`, parsing the 
    /// value's string representation if necessary.
    ///
    /// TODO: Need to return Molt-compatible Err's.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn as_float(&self) -> Result<MoltFloat, String> {
        let mut data_ref = self.data_rep.borrow_mut();
        let mut string_ref = self.string_rep.borrow_mut();

        // FIRST, if we have a float then just return it.
        if let Datum::Flt(flt) = *data_ref {
            return Ok(flt);
        }

        // NEXT, if we don't have a string_rep, get one from the current 
        // data rep.
        if (*string_ref).is_none() {
            *string_ref = Some(Rc::new(data_ref.to_string()));
        }

        // NEXT, Try to parse the string_rep as a float
        if let Some(str) = &*string_ref {
            // TODO: Currently uses the standard Rust parser.  That may
            // be OK, but I need to check.
            if let Ok(flt) = str.parse::<MoltFloat>() {
                *data_ref = Datum::Flt(flt);
                return Ok(flt);
            }
        }

        // NEXT, we can't interpret it as an integer; return an error.
        // TODO: need to use the right error message.
        Err("Not a float".to_string())
    }

    /// Creates a new `MoltValue` whose data representation is a `MoltList`.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn from_list(list: MoltList) -> MoltValue {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::List(Rc::new(list))),
        }
    }

    /// Tries to return the `MoltValue` as a `MoltList`, parsing the 
    /// value's string representation if necessary.
    ///
    /// TODO: Need to return Molt-compatible Err's.
    /// TODO: Need to add list parsing.
    /// 
    /// # Example
    /// 
    /// TODO
    pub fn as_list(&self) -> Result<Rc<MoltList>, String> {
        let data_ref = self.data_rep.borrow_mut();

        if let Datum::List(list) = &*data_ref {
            Ok(list.clone())
        // } else if let Some(_str) = &self.string_rep {
        //     panic!("list string_rep not defined!");
        } else {
            Err("Not a list".to_string())
        }
    }

    /// Creates a new `MoltValue` containing the given value of some user type. 
    /// 
    /// The type must implement `Display`, `Debug`, and `FromStr`, and the 
    /// `Display` output must be compatible with the `FromStr` parser (and with 
    /// TCL syntax).  The value will be reference counted.
    pub fn from_other<T: 'static>(value: T) -> MoltValue
    where
        T: Display + Debug,
    {
        MoltValue {
            string_rep: RefCell::new(None),
            data_rep: RefCell::new(Datum::Other(Rc::new(value))),
        }
    }

    /// Tries to interpret the `MoltValue` as a value of type `T`.
    ///
    /// The value is returned as an `Rc<T>`, as this allows the client to
    /// use the value freely.
    ///
    /// This method returns `Option` rather than `Result` because it is up
    /// to the caller to provide a meaningful error message.  It is normal
    /// for externally defined types to wrap this function in a function
    /// that does so.
    ///
    /// # Example
    /// 
    /// TODO
    pub fn as_other<T: 'static>(&self) -> Option<Rc<T>>
    where
        T: Display + Debug + FromStr,
    {
        let mut string_ref = self.string_rep.borrow_mut();
        let mut data_ref = self.data_rep.borrow_mut();

        // FIRST, if we have the desired type, return it.
        if let Datum::Other(other) = &*data_ref {
            // other is an &Rc<MoltAny>
            let result = other.clone().downcast::<T>();

            if result.is_ok() {
                // Let's be sure we're really getting what we wanted.
                let out: Rc<T> = result.unwrap();
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

    /// Tries to interpret the `MoltValue` as a value of type `T`, returning
    /// a copy.
    ///
    /// This method returns `Option` rather than `Result` because it is up
    /// to the caller to provide a meaningful error message.  It is normal
    /// for externally defined types to wrap this function in a function
    /// that does so.
    ///
    /// # Example
    /// 
    /// TODO
    pub fn as_copy<T: 'static>(&self) -> Option<T>
    where
        T: Display + Debug + FromStr + Copy,
    {
        let mut string_ref = self.string_rep.borrow_mut();
        let mut data_ref = self.data_rep.borrow_mut();

        // FIRST, if we have the desired type, return it.
        if let Datum::Other(other) = &*data_ref {
            // other is an &Rc<MoltAny>
            let result = other.clone().downcast::<T>();

            if result.is_ok() {
                // Let's be sure we're really getting what we wanted.
                let out: Rc<T> = result.unwrap();
                return Some(*out);
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
                return Some(*out);
            }
        }

        // NEXT, we couldn't do it.
        None
    } 
}

//-----------------------------------------------------------------------------
// The MoltAny Trait: a tool for handling external types.

/// This trait allows us to except "other" types, and still compute their
/// string rep on demand.
trait MoltAny: Any + Display + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl dyn MoltAny {
    /// Is this value a value of the desired type?
    pub fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }
    
    /// Downcast an `Rc<MoltAny>` to an `Rc<T>`
    fn downcast<T: 'static>(self: Rc<Self>) -> Result<Rc<T>, Rc<Self>>{
        if self.is::<T>() {
            unsafe {
                Ok(Rc::from_raw(Rc::into_raw(self) as _))
            }
        } else {
            Err(self)
        }
    }
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

// The data representation for MoltValues.
#[derive(Clone, Debug)]
enum Datum {
    /// A Molt integer
    Int(MoltInt),

    /// A Molt float
    Flt(MoltFloat),

    /// A Molt List
    List(Rc<MoltList>),

    /// An external data type
    Other(Rc<dyn MoltAny>),

    /// The MoltValue has no data rep at present.
    None,
}

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
    fn to_string() {
        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(*val.to_string(), "abc".to_string());

        let val2 = val.clone();
        assert_eq!(*val.to_string(), *val2.to_string());
    }

    fn as_string() {
        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(*val.as_string(), "abc".to_string());

        let val2 = val.clone();
        assert_eq!(*val.as_string(), *val2.to_string());
    }

    #[test]
    fn from_as_int() {
        let val = MoltValue::from_int(5);
        assert_eq!(*val.to_string(), "5".to_string());
        assert_eq!(val.as_int(), Ok(5));
        assert_eq!(val.as_float(), Ok(5.0));

        let val = MoltValue::from_string("7".to_string());
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.as_int(), Ok(7));
        assert_eq!(val.as_float(), Ok(7.0));

        // TODO: Note, 7.0 might not get converted to "7" long term.
        // In Standard TCL, its string_rep would be "7.0".  Need to address
        // MoltFloat formatting/parsing.
        let val = MoltValue::from_float(7.0);
        assert_eq!(*val.to_string(), "7".to_string());
        assert_eq!(val.as_int(), Ok(7));
        assert_eq!(val.as_float(), Ok(7.0));

        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(val.as_int(), Err("Not an integer".to_string()));
    }

    #[test]
    fn from_as_float() {
        let val = MoltValue::from_float(12.5);
        assert_eq!(*val.to_string(), "12.5".to_string());
        assert_eq!(val.as_int(), Err("Not an integer".to_string()));
        assert_eq!(val.as_float(), Ok(12.5));

        let val = MoltValue::from_string("7.8".to_string());
        assert_eq!(*val.to_string(), "7.8".to_string());
        assert_eq!(val.as_int(), Err("Not an integer".to_string()));
        assert_eq!(val.as_float(), Ok(7.8));

        let val = MoltValue::from_int(5);
        assert_eq!(val.as_float(), Ok(5.0));

        let val = MoltValue::from_string("abc".to_string());
        assert_eq!(val.as_float(), Err("Not a float".to_string()));
    }

    #[test]
    fn from_as_list() {
        let a = MoltValue::from_string("abc".to_string());
        let b = MoltValue::from_float(12.5);
        let listval = MoltValue::from_list(vec![a.clone(), b.clone()]);

        // Get it back as Rc<MoltList>
        let result = listval.as_list();

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
        let result = myval.as_other::<RGB>();
        assert!(result.is_some());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);

        let myval = MoltValue::from_string("#010203".to_string());
        let result = myval.as_other::<RGB>();
        assert!(result.is_some());

        let rgb2 = result.unwrap();
        assert_eq!(rgb, *rgb2);
    }
}
