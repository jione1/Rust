enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}
/* struct 사용
struct IpAddr{
    kind: IpAddrKind,
    address String,
}

let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr{
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
*/

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

enum Option<T>{
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a String");

let absent_number: Option<i32> = None;