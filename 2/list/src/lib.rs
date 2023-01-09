pub mod list {
    use std::alloc::{dealloc, realloc, Layout};
    use std::ptr::{self};

    pub struct List<T> {
        count: usize,
        items: *mut T,
    }

    impl<T: std::cmp::PartialEq> List<T> {
        pub fn new() -> Self {
            Self {
                count: 0,
                items: ptr::null_mut(),
            }
        }

        pub fn get(&self, index: usize) -> Option<T> {
            if index < self.count {
                unsafe { Some(ptr::read(self.items.add(index))) }
            } else {
                None
            }
        }

        pub fn insert(&mut self, index: usize, value: T) {
            if index > self.count {
                return;
            }

            self.items = unsafe {
                let new_layout = Layout::array::<T>(self.count + 1).unwrap();
                let old_layout = Layout::array::<T>(self.count).unwrap();
                let old_ptr = self.items as *mut u8;
                realloc(old_ptr, old_layout, new_layout.size()) as *mut T
            };

            unsafe {
                ptr::copy(
                    self.items.add(index),
                    self.items.add(index + 1),
                    self.count - index,
                );
                ptr::write(self.items.add(index), value);
                self.count += 1;
            }
        }

        pub fn remove(&mut self, index: usize) {
            if index > self.count {
                return;
            }

            unsafe {
                ptr::copy(
                    self.items.add(index + 1),
                    self.items.add(index),
                    self.count - 1 - index,
                );
            }

            self.items = unsafe {
                let new_layout = Layout::array::<T>(self.count - 1).unwrap();
                let old_layout = Layout::array::<T>(self.count).unwrap();
                let old_ptr = self.items as *mut u8;
                realloc(old_ptr, old_layout, new_layout.size()) as *mut T
            };

            self.count -= 1;
        }

        pub fn search(&self, value: T) -> usize {
            let mut index = 0;
            while index < self.count {
                if self.get(index).unwrap() == value {
                    return index;
                }
                index += 1;
            }
            return usize::MAX;
        }

        pub fn count(&self) -> usize {
            self.count
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            unsafe {
                dealloc(
                    self.items as *mut u8,
                    Layout::array::<T>(self.count).unwrap(),
                )
            };
        }
    }
}
