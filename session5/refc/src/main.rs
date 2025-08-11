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

fn main() {
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

// Rc -> reference count is "like" a garbage collector
/*
 * using rc is when you represent a finite source
 * when you clone, it's actually doens't create a clone at all, it just creates a reference that increments da counter that you're pointing to the original
 * borrow check is "happy" since the Rc is the onwer of your data and the Rc will count what is using it and dispose of it when is no longer in use
 * Rc by default is not compatible with multi-thread and not include synchronization protection
 */

