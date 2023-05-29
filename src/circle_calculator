// Calculate the circumference, and area of a circle.

const PI: f64 = std::f64::consts::PI;

fn main() {
    let radius: f64 = 3.0;

    let perimeter = calc_circle_perimeter(radius);
    let area = calc_area_of_circle(radius);

    println!(
        "The perimeter of a circle with radius of {}, is {:.2}",
        radius, perimeter
    );

    println!(
        "The area of a circle with radius of {}, is {:.2}",
        radius,
        area
    );
}

fn calc_circle_perimeter(r: f64) -> f64 {
    2.0 * PI * r
}

fn calc_area_of_circle(r: f64) -> f64 {
    PI * r.powi(2)
}
