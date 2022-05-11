
fn factorial(n: i32) -> i32 {
    if n == 1 { 
        return 1;
    }
    return n * factorial(n - 1);
}

fn factorial_tail_recursive(n: i32, acc: i32) -> i32 {
    if n == 1 {
        return acc;
    }
    return factorial_tail_recursive(n - 1, n * acc)
}

fn main() {
    println!("{}", factorial(4));
    println!("{}", factorial_tail_recursive(4, 1));
}
