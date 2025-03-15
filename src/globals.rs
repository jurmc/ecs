use std::collections::HashMap;
use std::any::Any;

pub struct Globals {
    globals: HashMap<String, Box<dyn Any>>,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            globals: HashMap::new(),
        }
    }

    pub fn add<T: Any>(&mut self, name: &str, c: T) {
        self.globals.insert(String::from(name), Box::new(c));
    }

    pub fn get<T: 'static>(&mut self, name: &str) -> Option<&T> {
        let retval = self.globals.get(name)?;
        retval.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        let retval = self.globals.get_mut(name)?;
        retval.downcast_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_globals() {
        let mut g = Globals::new();

        let name1 = "first";
        let name2 = "second";

        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2];
        let v1_expected = v1.clone();
        let v2_expected = v2.clone();

        g.add(name1, v1);
        g.add(name2, v2);

        assert_eq!(Some(&v1_expected), g.get::<Vec<u32>>(name1));
        assert_eq!(Some(&v2_expected), g.get::<Vec<u32>>(name2));

        let v = g.get_mut::<Vec<u32>>(name1).unwrap();
        v.pop();

        assert_eq!(Some(&v2_expected), g.get::<Vec<u32>>(name1));
    }

    #[test]
    fn test_globals_exceptions() {
        let mut g: Globals = Globals::new();

        let name = "non-existing";
        assert_eq!(None, g.get::<Vec<u32>>(name));

        let name = "non-existing";
        assert_eq!(None, g.get_mut::<Vec<u32>>(name));
    }
}
