# MoltValue thoughts

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
