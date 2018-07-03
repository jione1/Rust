#[macro_use] extern crate proptest;

use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree};
use proptest::prelude::*;

fn main(){
    let mut runner = TestRunner::default();
    //let int_val = (0..100i32).new_value(&mut runner).unwrap();
    //let str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}".new_value(&mut runner).unwrap();

    for _ in 0..256{
        let mut val = (0..10000i32).new_value(&mut runner).unwrap();
        if some_function(val.current()){
            continue;
        }

        loop{
            if !some_function(val.current()){
                if !val.simplify(){
                    break;
                }
            }
            else{
                if !val.complicate(){
                    break;
                }
            }
        }

        println!("The minimal failing case is {}", val.current());
        assert_eq!(501, val.current());
        return;
    }
    panic!("Didn't find a failing test case");
    // let mut str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}".new_value(&mut runner).unwrap();
    // println!("str_val = {}", str_val.current());
    // while str_val.simplify() {
    //     println!("        = {}", str_val.current());
    // }
    
    // println!("int_val = {}, str_val = {}", int_val.current(), str_val.current());
}

fn some_function(v: i32) -> bool{
    // assert!(v <= 500);
    v <= 500
}

#[test]
fn some_function_doesnt_crash(){
    let mut runner = TestRunner::default();
    for _ in 0..256{
        let val = (0..10000i32).new_value(&mut runner).unwrap();
        some_function(val.current());
    }
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

#[derive(Clone, Debug)]
struct Order{
    id: String,
    item: String,
    quantity: u32,
}

fn do_stuff(order: &Order){
    let i: u32 = order.id.parse().unwrap();
    let s = i.to_string();
    assert_eq!(s, order.id);
}

// fn arb_order(max_quantity: u32) -> BoxedStrategy<Order>{
//     (any::<u32>().prop_map(|v|v.to_string()), "[a-z]*", 1..max_quantity)
//     .prop_map(|(id, item, quantity)| Order{id, item, quantity}).boxed()
// }

prop_compose!{
    fn arb_order_id()(id in any::<u32>()) -> String{
        id.to_string()
    }
}

prop_compose!{
    fn arb_order(max_quantity: u32)(id in arb_order_id(), item in "[a-z]*", quantity in 1..max_quantity) -> Order{
        Order{id, item, quantity}
    }
}

fn from(a: i32, b: i32) -> (i32, i32){
    let d = (a - b).abs();
    (a / d, b /d)
}

proptest!{
    #[test]
    fn test_add(a in 0..1000i32, b in 0..1000i32){
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
    }

    // #[test]
    // fn test_do_stuff(ref v in "[1-9][0-9]{0,8}"){
    //     do_stuff(v);
    // }

    // #[test]
    // fn test_do_stuff(ref v in any::<u32>().prop_map(|v| v.to_string())){
    //     do_stuff(v);
    // }

    // #[test]
    // fn test_do_stuff(
    //     ref order in (any::<u32>().prop_map(|v| v.to_string()), "[a-z]*", 1..1000u32).prop_map(|(id, item, quantity)| Order { id, item, quantity })) {
    //     do_stuff(order);
    // }

    #[test]
    fn test_do_stuff(ref order in arb_order(1000)){
        do_stuff(order);
    }

    #[test]
    fn some_test(v in (0..1000u32).prop_filter("Values must not divisible by 7 xor 11".to_owned(), |v| !((0 == v % 7) ^ (0 == v % 11)))){
        assert_eq!(0 == v % 7, 0 == v % 11);
    }

    #[test]
    fn test_frob(a in -1000..1000, b in -1000..1000){
        //if(a==b){return Err(TestCaseError::Reject(...))}
        prop_assume!(a != b);

        let (a2, b2) = from(a, b);
        assert!(a2.abs() <= a.abs());
        assert!(b2.abs() <= b.abs());
    }
}