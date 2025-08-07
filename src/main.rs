use std::io;
use rand::Rng;

fn main() {
    println!("Password Generator.");
    let password = gen_password();
    println!("Generated Password is - {}", password);
}
fn gen_password() -> String {
    println!(" Choose length yours password (8 or 16 chars)");
    let mut length_choice = String::new();
    io::stdin()
    .read_line(&mut length_choice)
    .expect("failed");
    let length_choice = match length_choice.trim().parse() {
        Ok(8) => 8,
        Ok(16) => 16,
        Ok(_) => {
            println!("Password will be 8 or 16!");
            return String::new();
        }
        Err(_) => {
            println!("Please, write the number");
            return String::new();
        }
    };
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    let password:String = (0..length_choice)
    .map(|_| {
        
        let random_index = rng.gen_range(0..chars.len());
        chars[random_index]
    })
    .collect();

    password
}