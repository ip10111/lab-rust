// Check if a number is prime.

fn main() {
    let number = 23;
    let mut info: String = format!("{number} is not a prime number");

    if is_prime(number) {
        info = format!("{number} is a prime number");
    }

    println!("{}", info);
}

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
