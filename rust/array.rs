fn give_array() -> [i32; 5] {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    return xs;
}

fn main() {
    let array = give_array();
    println!("The first element is: {}", array[0]);
}
