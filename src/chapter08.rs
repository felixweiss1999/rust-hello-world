fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    vec.push(3);
    println!("{}", vec[0]);
    let does_not_exist = &_v[100];
    let does_not_exist = _v.get(100);
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    } // interestingly, the mutable reference prevents modification of the whole vector during the for loop, because you can't have immutable and mutable references to the same object within the same scope (if they really overlap in scope)!


}