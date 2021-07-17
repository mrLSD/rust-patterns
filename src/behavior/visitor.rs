#[cfg(test)]
mod test {
    use std::sync::Arc;

    pub trait Visitor {
        fn visit_component1(&self, component: &Component1);
        fn visit_component2(&self, component: &Component2);
    }

    pub trait Component {
        fn accept(&self, visitor: Arc<dyn Visitor>);
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
        fn accept(&self, visitor: Arc<dyn Visitor>) {
            visitor.visit_component1(&self);
        }
    }

    impl Component for Component2 {
        fn accept(&self, visitor: Arc<dyn Visitor>) {
            visitor.visit_component2(&self);
        }
    }

    pub struct Visitor1;

    impl Visitor for Visitor1 {
        fn visit_component1(&self, component: &Component1) {
            println!("{}", component.name())
        }
        fn visit_component2(&self, component: &Component2) {
            println!("{}", component.name())
        }
    }
    
    #[test]
    fn test_visitor() {
        let components: Vec<Box<dyn Component>> = vec![Box::new(Component1), Box::new(Component2)];
        let visitor = Arc::new(Visitor1);
        components
            .iter()
            .for_each(|el| el.accept(visitor.clone()));
    }
}
