// demo unsafe rust

use std::slice;

fn main() {
    // Raw Pointers
    // -------------------------------------
    // We can create raw pointers outside of
    // an unsafe block. 
    let mut num = 5;
    
    let raw_immutable = &num as *const i32;
    let raw_mutable = &mut num as *mut i32;
    
    // We can even create a raw pointer to an 
    // arbitrary location in memory.
    // (Rarely is there any point to doing this.)
    let _address = 0x012345usize;
    let _r = _address as *const i32;
    
    // But dereferencing raw pointers
    // must be done inside an unsafe block.
    unsafe {
        println!("raw_immutable is: {}", *raw_immutable);
        println!("raw_mutable is: {}", *raw_mutable);
    }
    
    
    // Calling an Unsafe Function or Method
    // -------------------------------------
    // Here we are doing nothing unsafe,
    // but the unsafe designation requires that calls
    // happen inside an unsafe block.
    unsafe fn technically_dangerous() {}
    unsafe {
        technically_dangerous();
    }
    
    unsafe fn _actually_dangerous() {
        // the body of an unsafe function is effectively
        // an unsafe block -- no need for another inside it.
        let num = 5;
        let raw_immutable = &num as *const i32;
        println!("raw_immutable is: {}", *raw_immutable);
    }
    
    
    // Creating a safe abstraction over Unsafe Code
    // -------------------------------------
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // // This implementation of split_at_mut() fails to compile
    // // because the compiler can't see that we're borrowing from
    // // different parts of the slice. It only sees that we're
    // // borrowing twice from the same slice.
    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();

    //     assert!(mid <= len);

    //     (&mut values[..mid], &mut values[mid..])
    // }

    // This implementation uses an unsafe function tailor-made 
    // for this purpose.
    // Notice how this function is labelled safe, encapsulating 
    // the unsafe label.
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        // Note: This assertion...
        // - DOES demonstrate the unsafe code was used for a reason
        // - DOES NOT prove the unsafe code is actually safe.
        // A rigorous third party could find bugs in unsafe code
        // despite the presence of assertions like this.
        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    


}
