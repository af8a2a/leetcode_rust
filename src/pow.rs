pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut x=x;
    let mut n=n;
    if n < 0 {
        x=1.0/x;
        n=n.abs();
    }

    if n == 0 {
        1.0
    } else if n % 2 == 0 {
        my_pow(x * x, n / 2)
    } else {
        my_pow(x * x, (n - 1) / 2) * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(my_pow(2.0, 10), 1024.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(my_pow(2.1, 3), 9.261000000000001);
    }
}
