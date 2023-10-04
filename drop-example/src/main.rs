struct CustomSmartPointer {
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // notice how variables are dropped in reverse 
    // order of their creation.
    println!("CustomSmartPointers created.");


    let c = CustomSmartPointer {
        data: String::from("some data"),
    };   
    println!("CustomSmartPointer created");
    drop(c);
    // c.drop() is not allowed, as it would cause a double-free.
    println!("CustomSmartPointer dropped before the end of main.");
}