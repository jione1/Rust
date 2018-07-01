extern crate rand;

//input & output library
use std::io;
//Less, Greatet and Equal => compare two values
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        //let : 변수 선언
        //mut : mutable variable (default : immutable)
        //crated a mutable variable that is currenetly bound to a new, empty instance of a String
        let mut guess = String::new();

        //use std::io;를 선언 안 했을 때 => std::io::stdin으로 사용
        //read_line : standard input
        //.expect() <= io::Result
        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Pleate type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}