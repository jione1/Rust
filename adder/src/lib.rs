pub fn add_two(a: i32) -> i32{
    a + 2
}

pub fn greeting(name: &str) -> String{
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        //메크로 함수를 이용해 2+2가 4인지 확인
        //cargo test command 이용
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another(){
    //     panic!("Make this test fail");
    // }

    #[test]
    fn it_adds_two(){
        //left와 rigth비교
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol123"),
            "Greeting did not contain name, value was '{}'", result
        );
    }
}
