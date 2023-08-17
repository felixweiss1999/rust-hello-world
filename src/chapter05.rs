#[derive(Debug)]
struct Car {
    capacity: i32,
    brand: String,
    production_date: String,
    electric: bool,
}

impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Car: {} was produced on {} and has capacity {} and is {}electric)", self.brand, self.production_date, self.capacity, if self.electric {""} else {"not "})
    }
}

impl Car {
    fn talk_to_me(&self) {
        println!("Hello, I'm your car I guess :)");
    }
}

impl Car {
    fn hello(&self) {
        println!("Hello");
    }
    fn hihi() {
        println!("hihi");
    }
}
fn main(){
    
    let car1: Car = Car {capacity: 4, brand: String::from("Audi"), production_date: String::from("2023-08-17"), electric: true};
    let car2: Car = Car {capacity: 3, ..car1};
    println!("{car2}");
    println!("{:#?}", car2);
    dbg!(&car2);
    car2.talk_to_me();
    car2.hello();
    Car::hello(&car2);
    Car::hihi();
}