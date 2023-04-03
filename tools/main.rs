fn main() {}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    pub trait A {
        fn doa(&self);
        fn doc(&mut self);
    }

    pub trait B {
        fn dob(&self);
    }

    pub struct A1 {
        val: i32,
    }

    impl A for A1 {
        fn doa(&self) {
            println!("do a");
        }
        fn doc(&mut self) {}
    }

    impl B for A1 {
        fn dob(&self) {
            println!("do b");
        }
    }

    pub trait C: A + B {}

    impl C for A1 {}

    pub struct Test {
        pub a: A1,
    }

    impl AsRef<dyn A> for Test {
        fn as_ref(&self) -> &(dyn A + 'static) {
            &self.a
        }
    }

    impl AsRef<dyn B> for Test {
        fn as_ref(&self) -> &(dyn B + 'static) {
            &self.a
        }
    }

    impl AsRef<dyn C> for Test {
        fn as_ref(&self) -> &(dyn C + 'static) {
            &self.a
        }
    }

    impl AsMut<dyn A> for Test {
        fn as_mut(&mut self) -> &mut (dyn A + 'static) {
            &mut self.a
        }
    }

    #[test]
    fn test() {
        let rc = Rc::new(RefCell::new(1));
        let b = rc.clone();
        let rc2 = Rc::new(RefCell::new(1));
        let pt1 = rc.as_ptr();
        let pt2 = b.as_ptr();
        let pt3 = rc2.as_ptr();
        println!("{}", pt1 == pt2);
        println!("{}", pt1 == pt3);
    }
}
