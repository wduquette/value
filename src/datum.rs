use std::fmt::Debug;
use std::fmt::Display;
use std::any::Any;
use std::rc::Rc;

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

#[derive(Clone,Debug)]
enum Datum {
    Other(Rc<MyAny>),
    None
}

    fn to_other<T: 'static>(datum: &Datum) -> Rc<T>
        where T: Display + Debug
    {
        if let Datum::Other(other) = datum {
            // other is an &Rc<MyAny>
            // Here I cannot successfully downcast
            let result = (**other).as_any().downcast_ref::<Rc<T>>();
            println!("datum to_other: result={:?}", result);

            let out: Rc<T> = result.unwrap().clone();
            // it worked!
            return out;
        } else {
            panic!("failure in to_other");
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        // FIRST, create a Datum containing an Rc<MyAny>, where the
        // MyAny -> Rc<String>.
        //
        // The Rc<Rc<String>> is regrettable, but it allows the Datum to
        // be cloned, and then allows the content (Rc<String>) to be efficiently
        // cloned and returned from a function.
        let str = String::from("xyz");
        let datum = Datum::Other(Rc::new(Rc::new(str)));

        if let Datum::Other(other) = datum {
            // other is an Rc<MyAny>.
            // I can successfully downcast the MyAny to an Rc<String> again.
            let result = (*other).as_any().downcast_ref::<Rc<String>>();
            let out: Rc<String> = result.unwrap().clone();
            assert_eq!(*out, "xyz");
        } else {
            panic!("failed!");
        }

        // NEXT, try to do it in a function.
        let str = String::from("abc");
        let datum = Datum::Other(Rc::new(Rc::new(str)));

        let out = to_other::<String>(&datum);
        assert_eq!(*out, "abc");
    }

}
