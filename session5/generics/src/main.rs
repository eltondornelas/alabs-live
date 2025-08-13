fn generics_example_1() {
    // fn just_print_it<T: ToString>(x: T) {  // samething using "where"
    fn just_print_it<T, U>(x: T, y: U)
    where
        T: ToString + std::fmt::Debug,
        U: ToString,
    {
        // by compile time it will create "n" types of just_print_it, which increases slightly your binary
        println!("{}", x.to_string());
    }
    just_print_it("Hello", 5);
    just_print_it(5, 12);
}

fn generics_example_2() {
    struct Degrees(f32);
    struct Radians(f32);

    impl From<Radians> for Degrees {
        fn from(rad: Radians) -> Self {
            Degrees(rad.0 * 180.0 / std::f32::consts::PI)
        }
    }

    impl From<Degrees> for Radians {
        fn from(deg: Degrees) -> Self {
            Radians(deg.0 * std::f32::consts::PI / 180.0)
        }
    }

    fn sin<T: Into<Radians>>(angle: T) -> f32 {
        let angle: Radians = angle.into();
        angle.0.sin()
    }

    let behind_you = Degrees(180.0);
    let behind_you_radians = Radians::from(behind_you);
    let behind_you_radians2: Radians = Degrees(180.0).into();
}

fn main() {
    // generics_example_1();
    // generics_example_2();

    use std::collections::HashMap;

    #[derive(Debug)]
    struct HashMapBucket<K, V> {
        map: HashMap<K, Vec<V>>,
    }

    impl<K, V> HashMapBucket<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        fn new() -> Self {
            HashMapBucket {
                map: HashMap::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            let values = self.map.entry(key).or_insert(Vec::new());
            values.push(value);
        }
    }

    let mut bucket = HashMapBucket::new();
    bucket.insert("hello", 1);
    bucket.insert("hello", 2);
    bucket.insert("goodbye", 3);
    println!("{bucket:?}")
}
