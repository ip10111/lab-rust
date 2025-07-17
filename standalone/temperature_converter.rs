// Convert temperature between celsius, Fahrenheit, and Kelvin

fn main() {
    let celsius = 30_f64;
    let fahrenheit = 30.0;
    let kelvin = 30 as f64;

    // celsius
    println!(
        "{celsius} celsius is equal to {} fahrenheit",
        celsius_to_fahrenheit(celsius)
    );
    println!(
        "{celsius} celsius is equal to {} kelvin",
        celsius_to_kelvin(celsius)
    );

    // fahrenheit
    println!(
        "{fahrenheit} fahrenheit is equal to {} celsius",
        fahrenheit_to_celsius(fahrenheit)
    );
    println!(
        "{fahrenheit} fahrenheit is equal to {} kelvin",
        fahrenheit_to_kelvin(fahrenheit)
    );

    // kelvin

    println!(
        "{kelvin} kelvin is equal to {} celsius",
        kelvin_to_celsius(kelvin)
    );
    println!(
        "{kelvin} kelvin is equal to {} fahrenheit",
        kelvin_to_fahrenheit(kelvin)
    );
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * (9.0 / 5.0)) + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * (5.0 / 9.0)
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin * (9.0 / 5.0)) - 459.67
}
