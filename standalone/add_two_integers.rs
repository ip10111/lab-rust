// Add two integers and print the result.

fn main() {
    let a = 5;
    let b = 7;

    let sum = add(a, b);

    println!("{a} + {b} = {}", sum);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
