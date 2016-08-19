fn READ(sexp: &str) -> &str {
    return sexp;
}

fn main() {
    println!("{}", READ("(read \"test\")"));
}
