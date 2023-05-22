fn main() {
    let a = 5;
    let b = 2;
    
    let sum = sum(a, b);
    let multi = multiply(a, b);
    
    println!("Sum of {} plus {} is {}", a, b, sum);
    println!("{} multiply {} is {}", a, b, multi);
}

fn sum(x:i32, y:i32) -> i32 {
    return x+y;
}

fn multiply(x:i32, y:i32) -> i32 {
    return x*y;
}
