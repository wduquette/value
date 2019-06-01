# MoltValue thoughts

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
