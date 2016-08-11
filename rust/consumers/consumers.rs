fn main() {
    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
    for i in &one_to_one_hundred {
        println!("`i` equals.. {}!", i);
    }
}
