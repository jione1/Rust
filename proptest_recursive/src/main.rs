#[macro_use] extern crate proptest;

use std::collections::HashMap;
use proptest::test_runner::Config;
use proptest::prelude::*;

#[derive(Clone, Debug)]
enum Json{
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Map(HashMap<String, Json>),
}

fn arb_json() -> BoxedStrategy<Json>{
    let leaf = prop_oneof![
        Just(Json::Null),
        any::<bool>().prop_map(Json::Bool),
        any::<f64>().prop_map(Json::Number),
        ".*".prop_map(Json::String),
    ];
    leaf.prop_recursive(8, 256, 10, |inner| prop_oneof![
        prop::collection::vec(inner.clone(), 
        0..10).prop_map(Json::Array),
        prop::collection::hash_map(".*", inner,
        0..10).prop_map(Json::Map),]).boxed()
}

fn some_function(stuff: &[String], index: usize){
    let _ = &stuff[index];
}

// fn vec_and_index() -> BoxedStrategy<(Vec<String>, usize)>{
//     prop::collection::vec(".*", 1..100).prop_flat_map(|vec|{
//         let len = vec.len();
//         (Just(vec), 0..len)
//     }).boxed()
// }

fn add(a: i32, b: i32) -> i32{
    a + b
}

prop_compose!{
    fn vec_and_index()(vec in prop::collection::vec(".*", 1..100))
    (index in 0..vec.len(), vec in Just(vec)) -> (Vec<String>, usize){
        (vec, index)
    }
}

proptest!{
    // #[test]
    // fn test_some_function(ref stuff in prop::collection::vec(".*", 1..100), index in 0..100usize){
    //     prop_assume!(index < stuff.len());
    //     some_function(stuff, index);
    // }

    #[test]
    fn test_some_function((ref vec, index) in vec_and_index()){
        some_function(vec, index);
    }
}

proptest!{
    #![proptest_config(Config::with_cases(1000))]
    #[test]
    fn test_add(a in 0..1000i32, b in 0..1000i32){
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
    }
}

fn main() {
    println!("Hello, world!");
}
