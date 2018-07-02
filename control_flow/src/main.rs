fn main() {
    let number = 6;

    if number % 4 == 0{
        println!("number is divisible by 4");
    }
    else if number % 3 == 0{
        println!("number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition {
        5
    }
    else {
        6
    };

    println!("The value of number is:{}", num);

    let mut while_num = 3;
    while while_num != 0{
        println!("{}!", while_num);

        while_num = while_num - 1;
    }
    println!("LIFTOFF!!!");
}
