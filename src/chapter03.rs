use std::string;


fn main() {
    let x: i32 = 2147483635;

    let x = x + 10;

    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    let _months: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", months[1]);
    let x: i32 = function(&"hi".to_string());
    if x == 2 {
        
    } else if x != 3 {
        println!("asdf");
    }
}

fn function(x:&String) -> i32{
    println!("Successfully passed a string: {x}!");
    if true {2} else {4};
    loop{
        if elem == 2 {break 7;}
        break 3;
    }
}