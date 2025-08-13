fn cycle_problem_example() {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping node with value {}", self.value);
        }
    }

    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(tail.clone())),
    });

    // Let's break things creating chaos (turning list into a circle)
    *tail.next.borrow_mut() = Some(head.clone());

    println!("head: {head:?}");
    // even if comment this print, can see that nothing gets dropped, memory never gets free
    // since head holds a reference to the tail and the tail holds a reference to the head
}

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<NextNode>,
}

#[derive(Debug)]
enum NextNode {
    None,
    Strong(Rc<Node>), // strong reference/pointer
    Weak(Weak<Node>), // creating a weak link
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    // cycle_problem_example();
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(NextNode::None),
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(NextNode::Strong(tail.clone())),
    });

    *tail.next.borrow_mut() = NextNode::Weak(Rc::downgrade(&head));
    // if need to use the weak pointer, needs to check if it's valid before you can use it

    println!("head: {head:?}");
}

// in modern computers LinkedLists are slower than Vectors
// using Rc with linked lists might bring a problem
