fn factorial(n: usize) -> usize {
    if n<=1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    println!("{}",factorial(5));
}
