// Hide dead code warnings
#![allow(dead_code)]

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info {
        name: String,
        height: i32,
    },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is an Engineer!"),
        Person::Scientist => println!("is an Scientist!"),
        Person::Height(i) => println!("is a height of {}.", i),
        Person::Weight(i) => println!("is a weight of {}.", i),
        Person::Info { name, height } => {
            println!("{} is {} tall.", name, height);
        }
    }
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
