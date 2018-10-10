fn main() {
    println!("Hello, world!");
    fibonacci(2);
    gcd(2,4);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn fibonacci(mut n: u64) -> u64 {
    assert!(n > 0);
    let mut x = 1;
    let mut y = 1;
    while n > 2 {
        let t = y;
        y = x + y;
        x = t;
        n = n - 1;
    }
    y
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
fn test_fibonaccci() {
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
}
