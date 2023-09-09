use core::panic;

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];
    let mut _largest = number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    println!("The largest number is {}", largest_number(&number_list));
}

fn largest_number(list: &[i32]) -> &i32 {
    if list.is_empty() {
        panic!("Vector cannot be empty!")
    }
    let mut greatest = &list[0];
    for element in list {
        if element > greatest {
            greatest = element;
        }
    }
    greatest
}