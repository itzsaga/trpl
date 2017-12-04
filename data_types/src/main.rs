fn main() {
    println!("So many types!");
}

fn scalar_types() {
    // Floating-Point Types
    // f64
    let x = 2.0;

    // f32
    let y: f32 = 3.0;

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;

    // with explicit type annotation
    let f: bool = true;

    // Character Type
    // char type is specified with single quotes
    // strings use double quotes
    // char type represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn compound_types() {
    // Tuples
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    // will print 6.4
    println!("The value of y is: {}", y);

    // accessing directly with a period
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    // all elements must be the same type
    // fixed length
    // once declared cannot grow or shrink in size
    let a = [1, 2, 3, 4, 5];

    // accessing elements
    let first = a[0];
    let second = a[1];
}