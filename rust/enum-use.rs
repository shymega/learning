#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Solider,
}


fn main() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money :-("),
        Poor => println!("The poor have no money."),
    }

    match work {
        Civilian => println!("Civilians work."),
        Solider => println!("Soliders follow orders."),
    }
}
