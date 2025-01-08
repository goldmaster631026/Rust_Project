use std::io;
use rand::Rng;
use std::cmp::Ordering;

// struct User {
//     active : bool,
//     username: String,
//     email : String,
//     sign_in_count : u64,
// }

fn main() {

    // let user1 = User {
    //     email : String::from ("Someone@example.com");
    //     username : String::from ("someoneuwernam123");
    //     active : true;
    //     sign_in_count : 1;
    // }

    // println!("Hello, world!");
    // println! ("Guess the number");


    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
    // user1.email = String::from("yukiamano221@gmail.com");
    // println!("{user1.email}")

    loop {
        println! ("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read linke");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("To small!"),
            Ordering::Greater => println! ("Too big!"),
            Ordering::Equal =>{
                println! ("You win!");
                break;
            }
        }
    }    



}
