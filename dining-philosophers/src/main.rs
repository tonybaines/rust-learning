use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    // 'usize' is the type to index vectors
    left: usize, // left-hand fork
    right: usize, // right-hand fork
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    #[allow(deprecated)]
    fn eat(&self, table: &Table) {
        // Locks are released when _left/_right go out of scope
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating", self.name);

        thread::sleep_ms(1000); // deprecated
//        thread::sleep(Duration::new(1,0)); somehow blocks concurrency!

        println!("{} is done eating", self.name);
    }
}

fn main() {
    // Atomic Reference Count (Arc)
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0), // left-handed philosopher! Prevents deadlock apparently
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone(); // increments the Arc ref count.  Decremented when table out-of-scope

        thread::spawn(move || {
            p.eat(&table)
        })
    }).collect();

    for h in handles {
        h.join().unwrap()
    }
}