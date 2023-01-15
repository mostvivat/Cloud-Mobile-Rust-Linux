use std::io;

fn main() {
    println!("Enter a number:");

    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        },
    };

    
    print!("{} = ", number);
    let mut n = number;
    let mut first = true;
    for i in 2.. {
        while n % i == 0 {
            if first {
                first = false;
            } else {
                print!("*");
            }
            print!("{}", i);
            n /= i;
        }
        if n == 1 {
            break;
        }
    }
}