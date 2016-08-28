fn main() {
    let num = 5;
    let plus_num = |x: i32| x + num;

    let result = plus_num(5);

    println!("The result is: {}", result);
}
