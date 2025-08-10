struct MyStruct {
    n: i32
}

struct HasDroppables {
    x: MyStruct,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self {
            n
        }
    }    
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.n);
    }
}

fn move_me(x: MyStruct) {
    // Do nothing
}

fn main() {
    let x = MyStruct::new(1);
    {
        let y = MyStruct::new(2);
    }
    
    move_me(x);
    println!("Back from function");

    let has_drop = HasDroppables { x: MyStruct::new(4) };

    println!("Ending the main function");
}

// RAII -> Resource Acquisition Is Initialization