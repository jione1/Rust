//구조체
//struct 이름{
//     필드 이름: 필드 유형;    
// }

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Tuple Structs
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1's username is {}", user1.username);

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("aaaa"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        ..user1
    };

    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        // 이름이 같으면 반복하지 않아도 괜찮다.
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
