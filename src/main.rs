fn main() {
    let _x = 2.0;

    let y: f32 = 3.0;

    let x = 5;
    
    let x = x +1;

    let x = x * 2;

    println!("The value of x,y is: {},{}", x, y);


    let c = 'z';

    println!("z is {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, _z) = tup;

    println!("The value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{},{},{}", five_hundred, six_point_four, one);

    let _a = [1, 2, 3, 4, 5];

    let _months = ["january", "february"];

    let _a:[i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3;5];

    let _first = _a[0];

    let _second = _a[1];

    another_function();

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is : {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another function.");
}
