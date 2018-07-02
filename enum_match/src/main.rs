#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    //if와 유사해 보이나 if는 boolean형 조건이 필요
    match coin {
        Coin::Penny => {
            println!("Lucky");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}!", state);
            25
        },
    }
}