use std::thread;
use std::sync::{Mutex, Arc};

struct Philospher {
    name: String,
    left: usize,
    right: usize,
}

impl Philospher {
    fn new(name: &str, left: usize, right: usize) -> Philospher {
        Philospher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);
        
        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]});

    
    let philosophers = vec![
        Philospher::new("Baruch Spinoza", 0, 1),
        Philospher::new("Gilles Deleuze", 1, 2),
        Philospher::new("Karl Marx", 2, 3),
        Philospher::new("Friedrich Nietzsche", 3, 4),
        Philospher::new("Michel Focucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
