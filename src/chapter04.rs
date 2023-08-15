fn main() {
    let mut s = String::from("hello");
    s.push_str("string");
    change(&s);
    let r1 = &s;
}

fn change(some_string: &String) -> usize {
    some_string.len()
}