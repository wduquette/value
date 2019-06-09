// Because this is experimental.
#![allow(dead_code)]

// Initial attempt at MyValue struct in March 2019.
// How not to do it; this is a complete mess.
mod value1;

// A better attempt at MyValue struct, based on the realization that in Rust
// "immutable string" is spelled Rc<String>.  Beginning of a step-by-step
// approach.  string_rep and data_rep, with the data_rep allowing
// ints, floats, and MyList = Vec<MyValue>.  No user types, no interior mutability.
mod value2;

// First attempt at adding user types, e.g., Datum::Other.  No interior mutability;
// not very successful.
mod value3;

// Added interior mutability for string_rep, data_rep.  Supports only Datum::Int
// and Datum::Flt, but it works.
mod value4;

// An experimental version of the MyAny trait: Any + other required trait bounds.
mod my_any;

// A candidate user data type, supporting std::fmt::Display, std::fmt::Debug,
// std::str::FromStr, and Clone.
mod rgb;

// First attempt at integrating MyAny into MyValue.  A real mess, as I attempted
// to do much at once.  Things that needed to be done first:
//
// * Integrate MyList/Datum::List back in, so I see how to do a non-Copy type
//   without the "Any" complexity.
// * data_rep is defined as Option<Datum>, which is silly.  If I add Datum::None,
//   I can remove the Option<T>, and make it all simpler.
mod value5;

// Second attempt at integrating MyAny.  Just trying to support putting a
// user type in and getting it out again, with no shimmer logic.  The solution
// looks plausible, but doesn't work.
mod value6;

// Copied Datum out into a separate module, so I can experiment with
// MyAny and Datum::Other without the MyValue complexity.  With this I can
// put a user type in and get it out again, but there's more to do.
// Oddness: the user type value is saved as `Datum::Other(Rc<MyAny>)`; but
// when I get the user value back I need to get it as `Rc<T>` where T is the
// user type.  I can't seem to convert the `Rc<MyAny>` back to an `Rc<T>`
// so effectively I'm storing `Rc<Rc<T>>` as `Rc<MyAny>` so I can extract
// the `MyAny` as `Rc<T>`.
mod datum;

// Extended datum.rs to allow returning Result<Rc<T>, String> from a function.
mod datum2;

// Revised the MyValue::to_other method accordingly.  It now returns
// Rc<T> successfully.  Also, added the full shimmer logic for everything
// but Datum::List (since I don't have a string_rep for that at the moment).
mod value7;

// Preparing for use.
mod value8;

// Extracting references rather than Rc's
mod datum3;

// Trying to return immutable borrows rather than Rc's.
mod value9;
