fn main() {
    println!("Hello, world!");
}
use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::any::Any;

pub type MoltStringFunc = fn(value: &Any) -> String;
pub type MoltCloneFunc = fn(value: &Any) -> Box<dyn Any>;

struct MoltType {
    name: &'static str,
    to_string: MoltStringFunc,
    clone: MoltCloneFunc,
}

//-------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MoltPair {
    f1: i32,
    f2: i32,
}

const MOLT_PAIR: MoltType = MoltType {
    name: "MOLT_PAIR",
    to_string: MoltPair::any_to_string,
    clone: MoltPair::any_clone,
};

impl MoltPair {
    pub fn new(f1: i32, f2: i32) -> Self {
        Self { f1, f2 }
    }

    pub fn any_to_string(value: &Any) -> String {
        match value.downcast_ref::<MoltPair>() {
            Some(mp) => {
                format!("({},{})", mp.f1, mp.f2)
            }
            None => {
                panic!("not a MoltPair");
            }
        }
    }

    pub fn any_clone(value: &Any) -> Box<dyn Any> {
        match value.downcast_ref::<MoltPair>() {
            Some(mv) => {
                Box::new(mv.clone())
            }
            None => {
                panic!("not a MoltPair");
            }
        }
    }
}


// TODO: Implement debug for MoltType, and derive it for this.
struct IntRep {
    type_def: &'static MoltType,
    value: Box<dyn Any>,
}

impl IntRep {
    fn to_string(&self) -> String {
        (self.type_def.to_string)(&self.value)
    }
}

impl Clone for IntRep {
    fn clone(&self) -> Self {
        IntRep {
            type_def: self.type_def,
            value: (self.type_def.clone)(&self.value),
        }
    }
}


// TODO: Implement debug for MoltType, and derive it for this.
#[derive(Clone)]
struct OuterValue {
    inner: RefCell<InnerValue>,
}
// TODO: Implement debug for MoltType, and derive it for this.
#[derive(Clone)]
struct InnerValue {
    str_rep: Option<String>,
    int_rep: Option<IntRep>,
}

impl OuterValue {
    pub fn to_string(&self) -> String {
        let mut inner = self.inner.borrow_mut();

        if inner.str_rep.is_some() {
            inner.str_rep.as_ref().unwrap().clone()
        } else if inner.int_rep.is_some() {
            let string = inner.int_rep.as_ref().unwrap().to_string();
            inner.str_rep = Some(string.clone());
            string
        } else {
            String::new()
        }
    }
}

type MoltValue = Rc<OuterValue>;

// TODO: See if we can make this generic for any MoltType.
fn get_value(mv: &MoltValue) -> Result<MoltPair, String> {
    let inner = mv.inner.borrow();

    if let Some(int_rep) = &inner.int_rep {
        match int_rep.value.downcast_ref::<MoltPair>() {
            Some(mv) => {
                return Ok(mv.clone());
            }
            None => {
                return Err("Could not convert.".into());
            }
        }
    }

    if let Some(str_rep) = &inner.str_rep {
        // TODO: Parse as pair, if possible.
        return Ok(MoltPair::new(1,2));
    }

    Err("Conversion failed".into())
}
