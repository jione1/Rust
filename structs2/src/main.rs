//직사각형의 면적 계산하기
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area_impl(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle{width: 30, height :50};

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("The area of the rectangle is {} square pixels.", rect1.area_impl());
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}