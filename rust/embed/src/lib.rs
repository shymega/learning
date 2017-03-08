#[no_mangle]
pub extern "C" fn process() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
