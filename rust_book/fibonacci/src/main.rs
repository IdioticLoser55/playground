fn main() {
    let result = fibonacci(0, 1, 10);
    println!("{result}");
}

// 0, 1, 1, 2, 3, 5, 8

fn fibonacci(n1: u32, n2: u32, limit: u32) -> u32 {
    if limit <= 2 {
        n1 + n2
    } else {
        fibonacci(n2, n1 + n2, limit - 1)
    }
}

