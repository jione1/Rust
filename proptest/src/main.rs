#[macro_use] extern crate proptest;
use proptest::prelude::*;

//YYYY-MM-DD
fn parse_date(s: &str) -> Option<(u32, u32, u32)>{
    //날짜 형식이 10글자가아니면 None return
    if 10 != s.len(){
        return None;
    }
    
    //ASCII가 아니면 무시
    if !s.is_ascii(){
        return None;
    }

    if "-" != &s[4..5] || "-" != &s[7..8]{
        return None;
    }

    let year = &s[0..4];
    let month = &s[5..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(|y| month.parse::<u32>()
    .ok().and_then(
        |m| day.parse::<u32>().ok().map(|d| (y, m, d))
    ))
}

#[test]
fn test_parse_date(){
    assert_eq!(None, parse_date("2018-07-02"));
    assert_eq!(None, parse_date("2018-07-020"));
    assert_eq!(None, parse_date("2018-07020"));
    assert_eq!(Some((2018, 07, 02)), parse_date("2018-07-02"));
}

#[test]
fn test_unicode_gibberish() {
    assert_eq!(None, parse_date("aAௗ0㌀0"));
}

#[test]
fn test_october_first() {
    assert_eq!(Some((0, 10, 1)), parse_date("0000-10-01"));
}

proptest!{
    #[test]
    fn doesnt_crash(ref s in "\\PC*"){
        parse_date(s);
    }

    #[test]
    fn parse_all_valid_dates(ref s in "[0-9]{4}-[0-9]{2}-[0-9]{2}"){
        parse_date(s).unwrap();
    }
 
    #[test]
    fn parse_date_back_to_original(y in 0u32..10000, m in 1u32..13, d in 1u32..32){
        let (y2, m2, d2) = parse_date(&format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();
        println!("y = {}, m = {}, d = {}", y, m, d);
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }

    //항상 Test 성공
    #[test]
    fn i64_abs_is_never_negative(a in any::<i64>()){
        assert!(a.abs() >= 0);
    }
}

fn main(){

}