struct Car {
    capacity: i32,
    brand: String,
    production_date: String,
    electric: bool,
}

impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Car: {} was produced on {} and has capacity {})", self.brand, self.production_date, self.capacity)
    }
}
fn main(){
    
    let car1: Car = Car {capacity: 4, brand: String::from("Audi"), production_date: String::from("2023-08-17"), electric: false};
    let car2: Car = Car {capacity: 3, ..car1};
    println!("{car1}");
}