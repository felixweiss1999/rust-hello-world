fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    vec.push(3);
    println!("{}", vec[0]);
    //let does_not_exist = &_v[100]; would panic!
    let does_not_exist = _v.get(100);
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    } // interestingly, the mutable reference prevents modification of the whole vector during the for loop, because you can't have immutable and mutable references to the same object within the same scope (if they really overlap in scope)!

    // enums for vectors that effectively store values of different types.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // string magic:
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // Strings don't support index access: even if accessed using slices, program will panic at runtime if not clean unicode boundaries!
    //this is how to properly iterate over utf 8 characters in a string!
    for c in "Зд".chars() {
        println!("{c}");
    }
    // or use bytes() method for individual bytes! (under the hood, String is a wrapper for a vec<i8>!)
    for b in "Зд".bytes() {
        println!("{b}");
    }
    
    // hash maps:
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // here, copied() gets a copy of the reference (essentially gets rid of reference), and unwrap_or() is a useful function to handle Option<> data types (if Null, null it lol)

    //iterating over hash maps (essentially dictionaries!)

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    //default insert behavior is overwrite, since only one entry allowed per key. with following trick, you can check whether entry exists before changing
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


}