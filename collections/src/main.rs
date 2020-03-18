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


}
