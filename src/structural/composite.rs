//! # Composite pattern
//!
//! Structural pattern:
//! https://refactoring.guru/design-patterns/composite
//!

// The base Component class declares common operations for both simple and
// complex objects of a composition.
trait Component {
    // Optionally, the base Component can declare an interface for setting and
    // accessing a parent of the component in a tree structure. It can also
    // provide some default implementation for these methods.
    fn set_parent(&mut self, _parent: &Self) {}
    fn get_parent(&self) -> &Self {
        self
    }

    // In some cases, it would be beneficial to define the child-management
    // operations right in the base Component class. This way, you won't need to
    // expose any concrete component classes to the client code, even during the
    // object tree assembly. The downside is that these methods will be empty
    // for the leaf-level components.
    fn add(&mut self, _parent: &Self) {}
    fn remove(&mut self, _parent: &Self) {}

    // You can provide a method that lets the client code figure out whether a
    // component can bear children.
    fn is_composite(&self) -> bool {
        false
    }

    // The base Component may implement some default behavior or leave it to
    // concrete classes
    fn operation(&self) -> String;
}

// The Leaf class represents the end objects of a composition. A leaf can't have
// any children.
//
// Usually, it's the Leaf objects that do the actual work, whereas Composite
// objects only delegate to their sub-components.
struct Leaf;

impl Component for Leaf {
    fn remove(&mut self, parent: &Self) {
        let _ = parent;
    }

    fn operation(&self) -> String {
        "Lead".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_leaf() {}
}
