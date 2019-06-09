use std::any::Any;

trait MyAny: Any + std::fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + std::fmt::Display> MyAny for T {
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

struct Wrapper<T: ?Sized + std::fmt::Display>(T);

impl<T: 'static + std::fmt::Display> std::fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgb::RGB;

    fn get_rgb(value: &dyn MyAny) -> Option<&RGB> {
        let myval = value.as_any().downcast_ref::<Wrapper<RGB>>();
        match myval {
            Some(Wrapper(rgb)) => Some(rgb),
            _ => None,
        }
    }

    #[test]
    fn using_wrapper() {
        let x: Wrapper<RGB> = Wrapper(RGB::new(1, 2, 3));
        let a: &dyn MyAny = &x;
        assert_eq!(a.to_string(), "#010203".to_string());

        let myvar = a.as_any().downcast_ref::<Wrapper<RGB>>().unwrap();
        assert_eq!(myvar.to_string(), "#010203".to_string());

        assert_eq!(myvar.0.to_string(), "#010203".to_string());

        let a: &dyn MyAny = &x;
        let rgb = get_rgb(a).unwrap();
        assert_eq!(rgb, &RGB::new(1, 2, 3));
    }
}
