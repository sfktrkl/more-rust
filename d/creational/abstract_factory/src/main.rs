use rand::{thread_rng, Rng};

pub trait ProductA {
    fn operation_a(&self);
}

pub struct FirstProductA;
impl ProductA for FirstProductA {
    fn operation_a(&self) {
        println!("First Product A");
    }
}

pub struct SecondProductA;
impl ProductA for SecondProductA {
    fn operation_a(&self) {
        println!("Second Product A");
    }
}

pub trait ProductB {
    fn operation_b(&self);
}

pub struct FirstProductB;
impl ProductB for FirstProductB {
    fn operation_b(&self) {
        println!("First Product B");
    }
}

pub struct SecondProductB;
impl ProductB for SecondProductB {
    fn operation_b(&self) {
        println!("Second Product B");
    }
}

pub trait Factory {
    fn create_product_a(&self) -> Box<dyn ProductA>;
    fn create_product_b(&self) -> Box<dyn ProductB>;
}

pub struct FirstProductFactory;
impl Factory for FirstProductFactory {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(FirstProductA)
    }

    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(FirstProductB)
    }
}

pub struct SecondProductFactory;
impl Factory for SecondProductFactory {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(SecondProductA)
    }
    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(SecondProductB)
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

    let product_a = factory.create_product_a();
    product_a.operation_a();

    let product_b = factory.create_product_b();
    product_b.operation_b();
}
