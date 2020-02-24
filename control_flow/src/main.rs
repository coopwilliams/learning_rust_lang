fn main() {
    let number =3;

    if number < 5 { 
        println!("true!");
    } else {
        println!("false!");
    }

    // the rest is very similar to python

    // let's implement bubble sort

    let mut b = [123, 6, 8, 0, 0, 234, 7, 9, 0, 999];
    
    let result = loop {
    	let mut no_swaps = true;    
        for i in 0..b.len()-1 {
            if b[i] > b[i + 1] {
                let mut temp = b[i];
                b[i] = b[i + 1];
                b[i+1] = temp;
                no_swaps = false;
            }
        }
        if no_swaps {
            break b;
        };        
    };

    println!("Sorted list is: {:?}", &result);
}
