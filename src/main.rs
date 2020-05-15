fn main() {
    for i in 1..u64::max_value() {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2  {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let max = ((num as f64).sqrt() as i64 + 1) as u64;
    for i in (3u64..max).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
