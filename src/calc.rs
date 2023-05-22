fn main() {
    let a = 5;
    let b = 2;
    
    let sum = calc(a, b);
    
    println!("The sum of {} plus {} is {}", a, b, sum);
}

fn calc(x:i32, y:i32) -> i32 {
    return x+y;
}
