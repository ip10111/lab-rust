// Check if a number is prime.

fn main() {
    let number = 23;
    let mut info: String = format!("{number} is not a prime number");

    if is_prime(number) {
        info = format!("{number} is a prime number");
    }

    println!("{}", info);
}

// Learning version.
fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }

    for i in 2..x - 1 {
        if x % i == 0 {
            return false;
        }
    }

    true
}

// Optimized version.
// set x to u32, since negative numbers are not considered prime.
fn is_prime_opt(x: u32) -> bool {
    if x < 2 {
        return false;
    }

    // Special case.
    if x == 2 {
        return true;
    }

    // Handling all even number.
    if x % 2 == 0 {
        return false;
    }

    // Set the limit as the square root of x.
    // because if x is not prime and has a divisor greater than its square root, then it must also have a corresponding divisor smaller than its square root.
    let limit = (x as f64).sqrt() as u32;

    // Looping while skip all even number.
    for i in (3..=limit).step_by(2) {
        if x % i == 0 {
            return false;
        }
    }

    true
}
