fn main() {}

#[cfg(test)]
mod test {
    use std::{rc::Rc, cell::RefCell};

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
        let mut a = Test { a: A1 { val: 2 } };
        let a1: &dyn A = a.as_ref();
        a1.doa();
        let a2: &dyn B = a.as_ref();
        a2.dob();

        let a3 = a.as_mut();
        a3.doc();
        let rc = Rc::new(RefCell::new(Test { a: A1 { val: 1 } }));
        {
            let rc1 = rc.clone();
            let mut rc2 = rc1.borrow_mut();
            rc2.a.val = 2;
        }
        let val = rc.as_ref().borrow();
        println!("{:?} ", val.a.val);
    }
}
