fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x); //12

    //Numeric Operations
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);

    let div = 56.7 / 32.2;
    println!("Div: {}", div);

    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    //Tuple type
    //원소가 모두 다른 타입이어도 된다.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of x is {}, {}, {}", five_hundred, six_point_four, one);
    
    //Array Type
    //원소가 모두 같은 타입
    //길이가 고정되어있다.
    let array = [1, 2, 3, 4, 5];
    println!("index 1 is {}", array[1]);
}
