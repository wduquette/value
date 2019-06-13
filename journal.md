# MoltValue thoughts

## Things to remember to do

*   Determine: is it possible to convert an Rc<Any> to an Rc<T>?
    *   Or, can we make Datum::Other(_) contain something else that will work?
    *   Or, do we need the "Wrapper" newtype?
*   Figure out what the API for defining/using a user type should look like.
    *   You'd want to wrap from_other and to_other.
    *   Probably individual functions, but could be defined on the user_type struct.
*   Integration
    *   See the "TODO's" in value8.rs: spots where existing Molt code needs to link up.
    *   Use Molt's get_int() logic when converting string_rep to int.
        *   Probably the only place this code needs to exist, other than in `expr` (since
            there are some special details there).
        *   And note that MoltValue will replace the `get_*` methods/functions.
    *   Use Molt's float formatting (which is not yet written) when converting
        Datum::Flt() to string.
        *   Again, probably the only place this code needs to exist.
    *   Use Molt's list parsing and formatting for Datum::List.
    *   The MoltValue functions return errors as Result<_,String>; they should be
        Result<_,ResultCode>.
    *   And ResultCode should use `Error(MoltValue)` and `Return(MoltValue)` instead of String.

## 2019-06-12
*   Tried returning Ref<String> instead of Rc<String>, and immediately ran into
    runtime errors.  Better leave the Ref's purely inside.
    *   Per https://users.rust-lang.org/t/returning-immutable-references-with-option-refcell/29130
    *   If it could safely look like &str, that would be one thing; but it's
        clearly different.
    *   And Rc<String> is not obviously worse than Ref<String>; and it doesn't
        involve a runtime borrow.
*   Added MoltValue::as_string, which returns an Rc<String>; it's more efficient
    (or should be) than the std::fmt::Display to_string(), since it doesn't
    involve creating a new string most of the time.
*   Chose to use "as_" syntax, e.g., `as_string`, `as_int`, `as_list` instead of
    "to_" syntax.  It really is returning the value as that given type; but the
    type is a reference counted copy of what's in the MoltValue.  "as_" is only
    supposed to be used when the conversion is basically free; but it *is*
    basically free after the first time.  And it keeps `as_string` distinct
    from `to_string`.
*   Tried to make MyAny require Clone, and use just Datum::Other(MyAny).  Not going
    to fly, at least not obviously.

## 2019-06-08
*   Looked into defining MoltInt and MoltFloat as newtypes.
    *   Doable but tricky.
    *   There are MANY numeric traits; I'm not sure what the full list would be.
    *   Any implementation that I would do would be sufficiently complicated that
        I'm not sure I'd have faith in it, and I certainly wouldn't know where the
        holes were.
    *   All I'm really wanting to provide is special parsing for ints and special
        output for floats.  It's probably easier to provide the individual functions
        rather than trying to use FromStr and Display in this case.
    *   The same applies to MoltList.
    *   The conversion to and from strings should be localized in MoltValue for all
        of these types; once it's there, Rust code should just work with MoltValues, and it
        should all just work.
*   Do we want to define Eq or PartialEq on MoltValue? NO.
    *   It would have to be based on the string_rep.
    *   We compare in expressions and in some commands like "string equals".
    *   Rust code would normally extract the content in the desired form and use
        normal Rust comparisons.
    *   So, NO.  We don't.
*   Are there operators we'd like to define on MoltValue? E.g., "+" for concatenation? NO.
    *   TCL programmers would use TCL commands.
    *   Rust programmers would extract values and do things in Rust.
    *   You don't know what "+" should mean until you extract the data in the desired form.
*   See datum3.rs for an example of how to return the string_rep as an immutable borrow.
    This is much better than returning an Rc<String>.  Look into how to do this for all
    the types.
    *   Hmmm.  Easy for simple string fields, but harder for more complicated cases.
        *   Question at users.rust-lang.org:
        https://users.rust-lang.org/t/returning-immutable-references-with-option-refcell/29130
    *   And got an answer!  It turns out to be fairly straightforward.

## 2019-06-07
*   Continue working with Datum to handle Datum::Other properly.
    *   Specific things I need to do for Data::Other
        *   See if we can support Datum::Other in a way that doesn't
            involve nested Rc<>'s.
    *   Verified that we can return Rc<T> successfully, when just pass a &Datum
        to a function.
    *   Verified that we can return Rc<T> successfully in a MyValue method.
    *   Implemented std::fmt::Display for Datum.
    *   Extended MyValue::to_int and MyValue::to_float to do the whole shimmer logic:
        *   Return the data_rep if it is available and the right type.
        *   Otherwise, produce the string_rep if it doesn't exist.
        *   Then attempt to parse the string_rep for the desired type.
        *   On success, update the data_rep and return the value.
        *   On failure, return an error.
    *   Verified that MyValue::to_string() can convert Datum::Other to string
        when needed.
    *   Verified that we can do the whole shimmer dance on to_other.
*   As value8.rs, began cleaning up the code to produce MoltValue.
    *   Removed MyWrapper, as we didn't seem to need it.
    *   Renamed "My*" to "Molt*".
    *   Replaced MoltValue::to_string() with "impl Display for MoltValue".
    *   Don't need to implement FromStr: you don't parse a string to get a
        MoltValue; you just provide a string.

## 2019-06-05
*   Added RGB, as a type that supports FromStr and std::fmt::Display.
*   Implemented MyAny that supports std::fmt::Display
    *   See my_any.rs.
    *   Define a struct that implements std::fmt::Display, e.g., MyType.
    *   Wrapper(MyType) can be saved as a `dyn MyAny`.
    *   The `dyn MyAny` support to_string().
    *   And can be downcast to Wrapper<MyType> again.  Woohoo!
*   Extended it to show how to write `get_mytype` functions.
*   Next step: see if I can integrate this into MyValue!
*   Not as easily as I thought.
*   Sudden realization: Using Option<Datum> is goofy.  Just add Datum::None, and
    lose the Option. (Done)
*   Before integrating MyAny fully, I think I need to put Datum::List back in.
    *   I'm worried that my RefCell code is only working because I can `Copy`
        ints and floats.
    *   Nope, using MyList directly works just fine, because I'm returning a clone
        of an Rc<MyList>.
*   The problem is that I'm getting an Rc<MyAny> out of the Datum, and what I
    want to return is an Rc<T>.
    *   Can I save an Rc<T> as a MyAny, and downcast to get that?
*   I have code that looks like it should work.
    *   If I have a Datum::Other(), it works.
    *   If I have a &Datum::Other(), it doesn't.
    *   I dunno why.
*   Pulled the Datum code into a separate module; and there it seems to be
    working.  (?)
    *   More tomorrow.

## 2019-06-03
*   Implementing string conversion in Rust:
    *   To implement conversion of a type into a string, it should implement the
        ToString trait; and the way to implement the ToString trait is to
        implement the fmt::Display trait which also makes it work well with
        `print!()`.
        *   See https://doc.rust-lang.org/rust-by-example/conversion/string.html#to-and-from-strings

```Rust
impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: format list as a string
        write!(f, "{}", my_string)
    }
}
```

    *   To implement parsing, implement the `FromStr` trait.
        *   See https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html#implement-the-fromstr-trait-for-a-custom-struct
        *   You're basically implementing a `from_str()` function that returns a `Result`.
            The error type is up to the trait.  
        *   Then you can use the str::parse<T>("...") call.
    *   So, to be used as an "Other" within the MoltValue, the type needs to implement
        ToString and FromStr.

## 2019-06-02

*   Got a response from a person on users.rust-lang.org:
    *   https://users.rust-lang.org/t/enums-any-and-partialeq/28844/5
    *   It shows how to create a struct that implements Any plus others.
    *   Need to learn how to do this to define structs that can do Any and also
        be be converted to and from strings.
        *   Which means I need to go read up on how to do a struct that can be
            converted to and from a string.
        *   It looks like it should get the job done.
*   value4.rs: Add RefCell, allowing interior mutability.
    *   But no "Any" option, or MyList; want to keep it simpler as I implement the conversion
        logic.
    *   Added the RefCells.
        *   to_string() now saves the computed string_rep.
        *   to_int() and to_float() now saves the computed data_rep.
        *   The data_rep can shimmer if the same string can be interpreted as two different
            types, but the string_rep is fixed once set.
*   Next Steps:
    *   Figure out how to generalize the to/from string processing.  We ought to be able to
        define this on the Datum and require it of all types used within the Datum cases.
    *   Then, figure out how to use the forum answer given above to define a wrapper such
        that we can wrap any type that can be converted to and from a string, and also
        access it.

## 2019-06-01

*   value3.rs: Add `Datum::Other(Rc<Any>)`.
*   I can support any input type by adding `Datum::Other(Rc<Any>)`.
    *   It probably needs to be `Datum::Other({SomeTypeToken},Rc<Any>)`,
        where {SomeTypeToken} is a value used to look up the type details.
*   Problem: `Rc<Any>`` supports Clone; but it doesn't support PartialEq.  
    *   It isn't obvious (if it's even possible) to tell Rust that
        the `Any` is only going to contain values of types that
        implement certain traits like `PartialEq`.  `Any` really means "any".
        *   Got a question in at users.rust-lang.org.
    *   But this might not be a real problem.
        *   We'll be comparing MoltValues for equality either as numbers or
            as strings, not as MoltValue objects.
        *   If we use MoltValues as `HashMap` keys, as we will when I
            implement dicts and arrays, I'll need to hash on the `string_rep`,
            not on the object as a whole.
        *   Which means I need to find out how that's done, so I can:
            *   Make sure that there *is* a string representation
            *   And then hash on it.
*   Added code to parse string_reps into i64 and f64 if possible, and indicated where to do
    that for list and Any.
*   I've decided that I do need interior mutability to fill in the string_rep or data_rep.
*   Next Steps:
    *   `string_rep: RefCell<Option<Rc<String>>>`
    *   `data_rep: RefCell<Option<Datum>>`
    *   Update the i64 and f64 code to do string-to-data and data-to-string translations
        automatically.
        *   data-to-string only if there is no string_rep
        *   string-to-data if the requested data_rep isn't available.
    *   Determine how to register types with the relevant data-to-string and string-to-data
        conversions.
    *   NOTE: I could, possibly, always add the string_rep when given a data_rep, so that I
        don't ever need to compute it on the fly.  But since I need to compute the data_rep
        and save it, there's no real advantage to not using interior mutability for both.

## 2019-05-30

*   A MoltValue may or may not have an immutable clonable string. string.
    *   An immutable clonable string is spelled Rc<String>; and since we might not
        have it, it's Option<Rc<String>>.
*   I'm going to try avoiding the use of internal mutation for shimmering.
    *   (String, None) to (String, Data):
        *   If you have a MoltValue with just a string, you create a new MoltValue with the
            same string and a new data_rep.
    *   (None, Data) to (String, Data):
        *   If you have a MoltValue with just a data_rep, you create a new MoltValue with the
            same data_rep and the string.
    *   (None, Data1) to (String, Data1) to (String, Data2)
        *   If you need to shimmer from one data_rep to another, you do it in two steps.
*   value2.rs shows the beginnings of what I want.
    *   Will convert binary types to string on demand, and make them available as
        binary.
        *   Needs some work on the "to_*" methods, see comments.
    *   Better than what I've currently got, but without a better way to fill in the
        second slot might just as well be an enum.
    *   Doesn't support registered types.
    *   Supports three datums, i64, f64, and Vec<MyValue>
    *   Reference-counts strings and list content.
    *   Not sure how best to fill in the empty slot and keep it.

## 2019-03-22

*   Reference-counting
    *   We want to be able to share MoltValues across data structures, safely.
    *   So we need to use `Rc<T>`.
    *   But we want to pass `&[&MoltValue]` to methods, not
        `&[&Rc<MoltValue>]`.
    *   So `MoltValue` must be a type alias:
```
type MoltValue = Rc<MoltValueSomething>;
```
    *   Or a newtype? `struct MoltValue(Rc<Something>)`?
        *   But I think this would break Deref.
*   Immutability
    *   A MoltValue is meant to be immutable.  If you want to modify the
        data, you copy and modify that, leaving previous references alone.
    *   But, it's immutable from the TCL point of view.  Internally you can
        have shimmering.
        *   list to string to dict to list to dict
    *   Conversions between non-string types are via the string representation.
    *   Once the string_rep is established, it doesn't change.
    *   But you can start with the int_rep, then acquire the string_rep;
        and given the string_rep you can go through a series of int_reps.
    *   This means that the internals need to be contained in a RefCell.
        *   The MoltValue needs to be an immutable reference to (sometimes)
            mutable data.
*   The Null problem
    *   The string_rep can be present or absent.  Once present it never
        changes.
        *   So, `Option<String>``.
    *   The int_rep can be anything, and it can always change, and it might
        be nothing.
        *   So, `Any`.  (Any can be set to `()` when there is none.)
*   Operations on int_reps:
    *   The interpreter needs to be able to produce a string_rep from the
        int_rep, and to clone the int_rep (which might not be a normal Rust
        `clone` since it would handle MoltValues specially.  Or maybe it would;
        cloning `Rc<_>` increments the refcount, so cloning a `Vec<Rc<_>>`
        should do the right thing.)
    *   But with an Any you don't know what it is, so you don't know what
        traits it implements.  And you can only convert it to concrete types.
    *   So the inner struct needs a `MoltType`:
        *   A trait object
        *   Set to the appropriate struct for the int_rep.
        *   Takes `Any` and produces `String`, panicking if the int_rep can't
            be down_cast to the the right type.
        *   Takes `Any` and clones it, producing `Any`, panicking if the
            int_rep can't be down_cast to the right type.
        *   When setting the int_rep, you need to set the `MoltType` field
            as well.
    *   But the `MoltType` is also optional.
        *   So maybe `MoltIntRep` is a struct with a `MoltType` and an `Any`,
            and `MoltInnerValue` has an `Option<MoltIntRep>`; and we never
            set `Any` to `()`.
