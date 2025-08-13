fn trait_example_with_polymorphism() {
    use std::any::Any;

    trait Animal: std::fmt::Debug {
        fn speak(&self);
    }

    #[derive(Debug)]
    struct Cat;

    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow");
        }
    }

    #[derive(Debug)]
    struct Dog;

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof");
        }
    }

    fn speak_twice(animal: &impl Animal) {
        animal.speak();
        animal.speak();
        println!("{animal:?}")
    }

    fn make_animal() -> impl Animal {
        Cat
    }

    trait DowncastableAnimal {
        fn speak(&self) {
            println!("No idea")
        }
        fn as_any(&self) -> &dyn Any;
    }

    struct Tortoise;

    impl DowncastableAnimal for Tortoise {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
    speak_twice(&cat);

    make_animal().speak();

    // let animals: Vec<impl Animal> = vec![Cat, Dog]; // traits aren't sized; needs Box to work
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)]; // example of being dinamic trying to force OOP; not recommended
    // dynamic polimorphed vector; collection of pointers of Dogs and Cats in the heap;
    // being dynamic will make significantly slower (its runtime) than knowing the types as in compile time
    // it's possibel to sendo a Box contained a dynamic trait into over channels, but needs to support being Send
    // Rust does not support Inheritance, it forces you to Compose; don't overuse Traits

    animals.iter().for_each(|animal| animal.speak());

    /* // example of how to know what kind of Animal would be in that trait (needs to use Any for the downcast)
    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals.iter() {
        if let Some(t) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("I'm a tortoise");
        }
    }
    */
}

fn main() {
    trait_example_with_polymorphism();

    use std::ops::Add;

    struct Point {
        x: f32,
        y: f32,
    }

    impl Add for Point {
        type Output = Point;
        // the output not necessarily needs to be what you working on

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
        
}
