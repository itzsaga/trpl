use std::io;

fn main() {
    loop {
        println!("Enter 1 to convert from Celcius or 2 to convert from Farenheit:");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            to_farenheit();
            break;
        } else if input == 2 {
            to_celcius();
            break;
        } else {
            println!("Please enter 1 or 2!");
        }
    }
}

fn to_celcius() {
    println!("How many degrees farenheit?");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    
    let celcius = (input - 32.0) / 1.8;

    println!("{} fareneheit in celcius is {}.", input, celcius);
}

fn to_farenheit() {
    println!("How many degrees celcius?");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let farenheit = input * 1.8 + 32.0;

    println!("{} celcius in farenheit is {}.", input, farenheit);
}
