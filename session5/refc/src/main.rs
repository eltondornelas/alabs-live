fn arc_example_3() {
    use std::sync::{Arc, Mutex};

    struct SharedData {
        data: Mutex<String>,
    }

    impl SharedData {
        fn new(s: &str) -> Self {
            Self {
                data: Mutex::new(s.to_string()),
            }
        }
    }

    let my_shared = Arc::new(SharedData::new("Hello")); // Mutex now is inside the struct
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.data.lock().unwrap();
    println!("{data}");
}

fn arc_example_2() {
    use std::sync::{Arc, Mutex};

    struct SharedData(String);

    let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string()))); // Mutex gives synchronization protection
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.lock().unwrap();
            data.0.push_str(&format!(" {i}"));
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let data = my_shared.lock().unwrap();
    println!("{}", data.0);
}

fn arc_example_1() {
    #[derive(Debug)]
    struct Droppable(i32);

    impl Droppable {
        fn new(n: i32) -> Self {
            println!("Constructing {n}");
            Self(n)
        }
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.0);
        }
    }

    fn move_me(x: Arc<Droppable>) {
        println!("Moved {}", x.0);
    }

    let my_shared = Arc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());

    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_clone = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            println!("{my_clone:?}");
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("{my_shared:?}");
    println!("Application exit");
}

fn rc_example() {
    use std::rc::Rc;

    #[derive(Debug)]
    struct Droppable(i32);

    impl Droppable {
        fn new(n: i32) -> Self {
            println!("Constructing {n}");
            Self(n)
        }
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.0);
        }
    }

    fn move_me(x: Rc<Droppable>) {
        println!("Moved {}", x.0);
    }

    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());

    println!("{my_shared:?}");
    println!("Application exit");
}

use std::{cell::RefCell, sync::Arc};

struct MyData {
    data: RefCell<String>,
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string()),
        }
    }
}

fn move_data(data: Arc<MyData>) {
    let mut data = data.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    // rc_example();
    // arc_example_1();
    // arc_example_2();
    // arc_example_3();

    let shared_data = Arc::new(MyData::new());
    move_data(shared_data.clone());
    let data = shared_data.data.borrow();
    println!("{data}");
}

// RefCell
/* RefCell -> single item that can be threated as one unit in memory, can e a struct, single number...
 * unlike Mutex it doesn't lock, it borrows with the borrow function
 * it doesn't protect at compile level, only runtimes, means you still protected but changing where the protection happens
 * RefCell is really fast but should be used carefully, if no need to change data, stay with Arc
 */

// Arc -> atomic reference counting
/*
 * Arc is slightly slower but can be sent across threads
 * Arc is not Sync, it's Send, means it's only read.
 * OBS: you can make a type to be Sync and Send using unsafe but is not recommended *
 */

// Rc -> reference count is "like" a garbage collector
/*
 * using rc is when you represent a finite source
 * when you clone, it's actually doens't create a clone at all, it just creates a reference that increments da counter that you're pointing to the original
 * borrow check is "happy" since the Rc is the onwer of your data and the Rc will count what is using it and dispose of it when is no longer in use
 * Rc by default is not compatible with multi-thread and not include synchronization protection
 */
