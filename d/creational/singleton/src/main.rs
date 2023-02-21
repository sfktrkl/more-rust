use std::sync::{Mutex, Once};
use std::{mem::MaybeUninit, thread};

#[derive(Default)]
struct Singleton {
    value: Mutex<u8>,
}

impl Singleton {
    fn get_instance() -> &'static Singleton {
        static mut SINGLETON: MaybeUninit<Singleton> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton {
                    value: Mutex::new(0),
                };
                SINGLETON.write(singleton);
            });

            SINGLETON.assume_init_ref()
        }
    }
}

fn main() {
    let test = || {
        let singleton = Singleton::get_instance();
        let mut count = singleton.value.lock().unwrap();
        println!("Init value: {}", count);
        *count += 1;
        println!("End value: {}", count);
    };
    test();

    let handle = thread::spawn(move || {
        test();
    });
    handle.join().unwrap();

    test();
}
