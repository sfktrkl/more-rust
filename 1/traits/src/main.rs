fn main() {
    use traits::animal::*;

    let animal = Animal::new(String::from("Animal"));
    println!("{}", animal.name());
    animal.speak();

    let dog = Dog::new(String::from("Dog"), String::from("Woof"));
    println!("{}", dog.name());
    dog.speak();
}
