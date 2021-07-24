#[cfg(test)]
mod test {
    pub trait Visitor {
        fn visit_component1(&self);
        fn visit_component2(&self);
    }

    pub trait Component {
        fn accept(&self, visitor: Box<dyn Visitor>);
    }

    pub struct Component1;
    pub struct Component2;

    impl Component1 {
        pub fn name(&self) -> String {
            "Component1".to_string()
        }
    }

    impl Component2 {
        pub fn name(&self) -> String {
            "Component2".to_string()
        }
    }

    impl Component for Component1 {
        fn accept(&self, visitor: Box<dyn Visitor>) {
            visitor.visit_component1();
        }
    }

    impl Component for Component2 {
        fn accept(&self, visitor: Box<dyn Visitor>) {
            visitor.visit_component2();
        }
    }

    pub struct Visitor1;

    impl Visitor for Visitor1 {
        fn visit_component1(&self) {
            todo!()
        }

        fn visit_component2(&self) {
            todo!()
        }
    }

    #[test]
    fn test_visitor() {}
}
