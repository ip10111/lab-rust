// Simple calculator

fn main() {
    let x = 4_f64; // using notation
    let y = 2 as f64;
    let op = "divide";

    let result = calculate(x, y, op);

    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }
}

fn calculate(a: f64, b: f64, operator: &str) -> Result<f64, &'static str> {
    let result = match operator {
        "add" => Ok(a + b),
        "subtract" => Ok(a - b),
        "multiply" => Ok(a * b),
        "divide" => {
            if b == 0.0 {
                Err("Error: Division by zero")
            } else {
                Ok(a / b)
            }
        }
        "modulus" => Ok(a % b),
        _ => Err("Unknown operator"),
    };

    result
}
