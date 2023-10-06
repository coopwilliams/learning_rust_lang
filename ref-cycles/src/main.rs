// demonstrate reference cycles

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Note: this implementation of List is different
// from what's in `interior-mutability` crate.
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// Node for use in a tree
#[derive(Debug)]
struct Node {
    value: i32,
    // child has a non-owning reference to parent
    parent: RefCell<Weak<Node>>, 
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // This illustrates how we can't rely on Rust
    // to catch reference cycles.

    // ----------------
    // Let's make a tree where nodes are child-aware and parent-aware.
    // In this example, Weak<T> is used to prevent memory leaks
    // that would otherwise occur due to reference cycles. There is
    // no cycle here, because the child's Weak<T> reference to the
    // parent has no bearing on whether the parent's memory 
    // is deallocated.
    let leaf = Rc::new(Node {
        value: 3,
        // 
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // at this point the leaf has a weak reference to its parent
        
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // here the parent falls out of scope.
    // The leaf's reference automatically goes back to None.
    // This is due to how Drop() is implemented in Weak<T>.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}