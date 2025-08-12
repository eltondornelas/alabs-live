fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    // ' -> means lifetime and is static (stick around forever)
    i
}

struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

struct CatFeeder<'a> {
    // usually avoid creating structs that keeps references, 
    // except in a parent-child relationship where you pointer to the children, 
    // because the parent will be responsible for it going away
    // if need to use references, try Rc or Arc.
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    // let n = 12;
    // borrow(&n, &n);

    let mut cats = vec![
        Cat("Frodo".to_string()), 
        Cat("Bilbo".to_string())
    ];

    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat });
    }

    feeders.iter_mut().for_each(|f| f.feed());
}
