use std::collections::HashMap;

fn main() {
    // creates empty vector, specifying the type to store
    let v: Vec<i32> = Vec::new();

    // More commonly we just insert values and the type is inferred
    // But how does it know it's not a u8? 
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("vector is {:?}", v);
    println!("vector part 3 is {:?}", &v[2]); // panic on bad index
    println!("yes, vector part 3 is {:?}", v.get(2)); // handle out-of-bounds 

    for i in &mut v {
        *i += 50;
    }

    //new string
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string(); // you can also use this on the literal

    // the above is equivalent to
    let mut s = String::from("initial contents");

    let s2 = " and more";
    s.push_str(s2); // append data to string w/out taking ownership

    println!("s2 is {}", s2);

    //append just one char (Must use single quote ' ' not double " ")
    s.push('!');

    // Concatenate strings
    let t1 = String::from("Hey");
    let t2 = String::from(", what's up?");
    let t3 = t1 + &t2; // here t1 is moved and falls out of scope
    // the first thing to be added is always a string
    // the rest are borrows or raw data

    println!("t3 is {}", t3);

    // We can concatanate a lot more with the format! macro
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");

    let f = format!("{}-{}-{}", f1, f2, f3);

    println!("f is {}", f);

    // try concatenating data
    let d1 = "Hey";
    let d2 = ", how's it going in there?";
    let d3 = d1.to_string() + &d2; // first thing has to be sent to string
    
    println!("{}", d3);

    // using ranges to create string slices is dangerous
    // because out of boundary errors can cause panic at runtime
    let slice = &d2[0..4]; // &f2 here would compile but panic
    println!("{}", slice);

    //iterating over strings two ways
    for c in "coop".chars() {
        println!("{}", c);
    };
    for c in "coop".bytes() {
        println!("{}", c);
    };

    let guess: u32 = "42".parse().expect("Not a number!");
    let guess: u8 = "42".parse().expect("Not a number!");

    let x = 0;
    let mut x_ref = &x;
    println!("x now is: {}", x);
    println!("x_ref is: {}", *x_ref);
    x_ref = &1;
    println!("x_ref is: {}", *x_ref);
    println!("x now is: {}", x);
    
    let mut v3: Vec<i32> = vec![1,2,3];
    dup_in_place(&mut v3);
    println!("v_num is: {:?}", v3);
    
    implement_counter();

    // test shrink to fit function
    let mut shrink_me = vec![1, 2, 0, 5];
    remove_zeros(&mut shrink_me);
    println!("{:?}", shrink_me);
}


fn dup_in_place(v: &mut Vec<i32>) {
    // duplicate/repeat a vector in place

    for n_ref in 0..v.len() {
        v.push(v[n_ref]);
    }
}

fn implement_counter() {
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}

fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0 .. v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}