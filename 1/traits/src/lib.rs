pub mod animal {
    pub trait Speaks {
        fn name(&self) -> &str;
        fn speak(&self) {
            println!("Hello");
        }
    }

    pub struct Animal {
        name: String,
    }

    impl Animal {
        pub fn new(name: String) -> Self {
            Animal { name: name }
        }
    }

    impl Speaks for Animal {
        fn name(&self) -> &str {
            &self.name
        }
    }

    pub struct Dog {
        animal: Animal,
        sound: String,
    }

    impl Dog {
        pub fn new(name: String, sound: String) -> Self {
            Dog {
                animal: Animal::new(name),
                sound: sound,
            }
        }
    }

    impl Speaks for Dog {
        fn name(&self) -> &str {
            self.animal.name()
        }
        fn speak(&self) {
            println!("{}", self.sound);
        }
    }

    pub fn greet<T: Speaks>(animal: T) {
        println!("{}", animal.name());
        animal.speak();
    }
}
