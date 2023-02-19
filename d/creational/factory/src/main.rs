use rand::{thread_rng, Rng};

pub trait Product {
    fn operation(&self);
}

pub struct FirstProduct;
impl Product for FirstProduct {
    fn operation(&self) {
        print!("First Product");
    }
}

pub struct SecondProduct;
impl Product for SecondProduct {
    fn operation(&self) {
        print!("Second Product");
    }
}

pub trait Factory {
    fn factory_method(&self) -> Box<dyn Product>;

    fn run(&self) {
        let product = self.factory_method();
        print!("Created: ");
        product.operation();
        println!("");
    }
}

pub struct FirstProductFactory;
impl Factory for FirstProductFactory {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(FirstProduct)
    }
}

pub struct SecondProductFactory;
impl Factory for SecondProductFactory {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(SecondProduct)
    }
}

fn initialize() -> Box<dyn Factory> {
    let rng = thread_rng().gen_range(0..10);
    if rng % 2 == 0 {
        Box::new(FirstProductFactory)
    } else {
        Box::new(SecondProductFactory)
    }
}

fn main() {
    let factory = initialize();
    factory.run();
}
