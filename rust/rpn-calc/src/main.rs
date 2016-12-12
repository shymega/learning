use std::io;

struct Stack {
    numbers: Vec<f64>,
}

impl Stack {
    fn new() -> Stack {
        Stack { numbers: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.numbers.is_empty()
    }

    fn push(&mut self, n: f64) {
        self.numbers.push(n);
    }

    fn result(&self) -> f64 {
        self.numbers
            .last()
            .expect("Stack empty.");
    }
}

fn compute(arg1: &str, oper1: f64, oper2: f64) {}

fn main() {
    let sum = compute("*", 2.0, 2.0);
    println!("{}", sum);

    println!("Hello, world!");
}
