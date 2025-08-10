use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout
}

impl <T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout
            }
        }
    }

    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl <T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory from SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn main() {
    let mut my_num = SmartPointer::<i32>::new();
    my_num.set(12);
    println!("my_num = {}", my_num.get());


    let my_num = Box::new(12); // does samething of the "SmartPointer"
}

// anything representing a Resource in Rust is also using a Drop trait, it is fundamental
/*
 * Resource -> in Rust its a value that's managed by ownership and borrowing rules to ensure memory safety.
 * It's anything that a program needs to acquire, use, and eventually release, like memory, files, or network connections.
 * Rust's RAII (Resource Acquisition Is Initialization) (also known as OBRM - Ownership Based Resource Management)
 *  ensures these resources are automatically managed, preventing leaks and dangling pointers
*/

