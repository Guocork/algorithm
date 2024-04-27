/**
 *  这里展示递归
 */

fn factorial(n: usize) -> usize {
    if n<=1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// 斐波那契数列
// f(n) = f(n-1) + f(n-2)
// f(0) = 0    f(1) = 1
fn fibonacci(n: usize) -> usize {
    return if n <= 2 { 1 } else { fibonacci(n - 1) + fibonacci(n - 2 ) };
}

fn main() {
    println!("{}",factorial(5));
    println!("{}",fibonacci(5))
}
