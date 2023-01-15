use std::io;

fn main() {
    //receive input from user integer
    let mut n = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        },
    };
    //calculate sum
    let sum = n * (n + 1) / 2;
    println!("sum: {}", sum);
}
