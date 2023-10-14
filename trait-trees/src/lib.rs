// demonstrate proposed designs for trait trees
// that satisfy (not necessarily well) the requirement:

// Requirement: The API client is expected to 
// provide a single WidgetImpl enum representing 
// all possible widgets, and implement the 
// Widget trait for WidgetImpl.

// This example proves that, while Option 1
// is the only good choice, all the options
// technically satisfy the requirement.

// Option 1: children must be Self
pub trait Widget: Sized {
    fn render(&self) -> Vec<Self>;
}

// Option 2: children are a trait parameter
pub trait Widget2<Children> {
    fn render(&self) -> Vec<Children>;
}

// Option 3: children are an associated type
pub trait Widget3 {
    type Children: Widget3;
    fn render(&self) -> Vec<Self::Children>;
}

// Option 4: children are a reference trait object
pub trait Widget4 {
    fn render(&self) -> Vec<&dyn Widget4>;
}

// Option 5: children are a boxed trait object
pub trait Widget5 {
    fn render(&self) -> Vec<Box<dyn Widget5>>;
}