fn main() {
    println!("Hello, world!");
    fibonacci(2);
}

fn fibonacci(mut n:u64) -> u64{
    assert!(n > 0);
    let mut x = 1;
    let mut y = 1;
    while n > 2{
        let t = y;
        y = x + y;
        x = t;
        n = n - 1;
    }
    y
}

#[test]
fn test_fibonaccci(){
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
}