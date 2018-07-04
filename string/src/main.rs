fn main() {
    // let mut s = String::new();
    let data = "initial contents";
    println!("{}", data);

    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let mut s2 = String::from("foo");
    s2.push_str("bar");
    println!("{}", s2);

    let mut l = String::from("lo");
    l.push('l');
    println!("{}", l);
}
