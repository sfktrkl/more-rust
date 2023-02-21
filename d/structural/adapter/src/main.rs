pub trait Target {
    fn get_request(&self) -> String;
}

pub struct Adaptee;
impl Adaptee {
    pub fn get_specific_request(&self) -> String {
        String::from("tseuqer cificeps a")
    }
}

pub struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    pub fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee: adaptee }
    }
}

impl Target for Adapter {
    fn get_request(&self) -> String {
        let request = self.adaptee.get_specific_request();
        request.chars().rev().collect::<String>()
    }
}

fn main() {
    let adaptee = Adaptee;
    println!("{}", adaptee.get_specific_request());

    let target = Adapter::new(adaptee);
    println!("{}", target.get_request());
}
