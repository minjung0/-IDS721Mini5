use std::io;

fn main() {
    println!("Find a happy number!");
    
    loop {
        println!("Please input your number.");
        println!("If you want to exit, please input '0'.");

        let mut happy = String::new();

        io::stdin()
            .read_line(&mut happy)
            .expect("Failed to read line");

        let happy: i32 = match happy.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if happy == 0 {
            break;
        }
        
        let ishappy = is_happy(happy);

        match ishappy {
            true => println!("This is a happy number."),
            _ => println!("This is not a happy number."),
        }
    }
}

pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    loop {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            sum += d * d;
            n /= 10;
        }
        match sum {
            1 => return true,
            4 => return false,
            _ => n = sum,
        }
    }
}