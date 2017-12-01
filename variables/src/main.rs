fn main() {
    // immutable by default
    let y = 4;
    println!("The value of y is {}", y);
    // this will throw an error at compile time
    // y = 5

    // mutable
    let mut x =5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    shadowing();
}

// an example of shadowing
fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);
}