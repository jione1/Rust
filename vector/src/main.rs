enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let v3 = vec![100, 300, 400];
    for i in &v3{
        println!("{}", i);
    }

    let mut v4 = vec![10, 11, 12];
    for i in &mut v4{
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),  
    ];
}