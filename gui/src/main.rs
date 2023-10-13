use gui::{Button, Draw, Screen};


// This SelectBox struct illustrates how
// a library user could implement the Draw trait.
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box
    }
}

fn main() {
    // Now the user can create a Screen instance
    // having both library and custom components.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    // // Using trait objects can hurt DevEx for API clients.
    // // If we wanted to break out the components first,
    // // The compiler needs to know that the vector has
    // // the type specified in the Screen struct (Box<dyn Draw>).
    // // There's two ways to do this.
    // let components = vec![
    //     Box::new(SelectBox { /* .. */ }) as Box<dyn Draw>,
    //     Box::new(Button { /* .. */ }),
    // ];
    // // or
    // let components: Vec<Box<dyn Draw>> = vec![
    //     Box::new(SelectBox { /* .. */ }),
    //     Box::new(Button { /* .. */ }),
    // ];

    // Using trait objects also hurts performance, because
    // the compiler emits code that uses dynamic dispatch.

    screen.run();
}
