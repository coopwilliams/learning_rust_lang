pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // here, `Box<dyn Draw>` is a trait object,
    // a stand-in for any type inside a Box that
    // implements the Draw trait.

    // Generaly, we create a trait object by 
    // 1. specifying some sort of pointer, such as 
    //     a & reference or a Box<T> smart pointer, 
    // 2. then the dyn keyword, 
    // 3. and then specifying the relevant trait.
    
    // If we had used generics here instead,
    // the screen would only be able to 
    // hold one type of component.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code draw the button
    }
}

