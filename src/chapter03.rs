

fn main() {
    let x: i32 = 2147483635;

    let x = x + 10;


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", months[1]);
    let a = [1;200009];
    println!("{}", a[3000000]);
    let a: u32 = {};
}