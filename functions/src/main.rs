fn main() {
    another_function(5, 6);
    expression_example();
    gimme_five();
    run_me();
}

// functions use snake case
// in function signatures, you must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// statements do not return values
fn statement_example() {
    // this is a statement
    let y = 6;
}

fn expression_example() {
    let x = 5;

    // this is an expression
    // it evaluates to 4
    // expressions do not include ending semicolons
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// Functions with return values
fn five() -> i32 {
    5
}

fn gimme_five() {
    let x = five();

    println!("The value of x is: {}", x);
}

// another example
fn run_me() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}