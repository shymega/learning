fn main() {
    let nums = (1..100).collect::<Vec<i32>>();

    for num in nums.iter() {
        println!("{}", num);
    }
    
    let nums_2 = vec![1, 2, 3];

    for num in nums_2.iter() {
        println!("{}", num);
    }
}
