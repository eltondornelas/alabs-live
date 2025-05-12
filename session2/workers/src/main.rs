use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;
// Box is a smart pointer, can never be null, it points to an area on the heap, when the Box goes out of scope it automatically delete it and they are not unsafe operations

enum Command {
    Run(Job),
    Quit,
}

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    // let (tx, rc) = mpsc::channel::<Job>();
    let (tx, rc) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rc.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
        /*
               while let Ok(job) = rc.recv() {
                   // // a pointer to a function somewhere in memory; as function pointer you just need to call the variable function
                   job();
               }
        */
    });

    /*
       let job =  || println!("Hello from a closure");
       let job2 = || {
           for i in 0..10 {
               println!("{i}");
           }
       };
       // both are function pointers cointaining the closure

       tx.send(Box::new(job)).unwrap();
       tx.send(Box::new(job2)).unwrap();
       tx.send(Box::new(hi_there)).unwrap();
       tx.send(Box::new(|| println!("I'm in a box"))).unwrap();
    */

    tx.send(Command::Run(Box::new(|| println!("I'm in a box"))))
        .unwrap();
    tx.send(Command::Run(Box::new(hi_there))).unwrap();
    tx.send(Command::Quit).unwrap();

    handle.join().unwrap();
    // not terminate until the thread is done (in this case is never); program never end
}
