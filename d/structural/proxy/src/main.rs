pub trait HasOperation {
    fn operation(&self);
}

pub struct Service;
impl HasOperation for Service {
    fn operation(&self) {
        println!("Service: Handle operation.")
    }
}

pub struct Proxy {
    service: Service,
}

impl Proxy {
    pub fn new(service: Service) -> Self {
        Proxy { service: service }
    }

    pub fn check_access(&self) -> bool {
        println!("Proxy: Checking access.");
        true
    }
}

impl HasOperation for Proxy {
    fn operation(&self) {
        if self.check_access() {
            self.service.operation();
        }
    }
}

fn main() {
    let service = Service;
    service.operation();

    let proxy = Proxy::new(service);
    proxy.operation();
}
