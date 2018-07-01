fn main() {
    println!("Hello, world!");
    function_1();
    function_2(3);
    function_3(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is:{}", y);

    println!("The value of five() is {}", five());
    println!("The value of plus_one() is {}", plus_one(5));
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn function_1(){
    println!("Anotehr fuction.");
}

fn function_2(x: i32){
    println!("The value of x is:{}", x);
}

fn function_3(x: i32, y: i32){
    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);
}