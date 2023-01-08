use traits::animal::*;

pub fn greet<T: Speaks>(animal: T) {
    println!("{}", animal.name());
    animal.speak();
}

fn main() {
    let animal = Animal::new(String::from("Animal"));
    greet(animal);

    let dog = Dog::new(String::from("Dog"), String::from("Woof"));
    greet(dog);
}
