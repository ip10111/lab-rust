// Rectangle calculator

fn main() {
    let rect_w: f64 = 4.0;
    let rect_h: f64 = 5.0;

    println!(
        "Perimeter of a rectangle with width {}, and height {} is {}",
        rect_w,
        rect_h,
        calc_rectangle_perimeter(rect_w, rect_h)
    );

    println!(
        "Area of a rectangle with width {}, and width {} is {}",
        rect_w,
        rect_h,
        calc_area_of_rectangle(rect_w, rect_h)
    );

    println!(
        "Diagonal of a rectangle with width {}, and width {} is {}",
        rect_w,
        rect_h,
        calc_diagonal_of_rectangle(rect_w, rect_h)
    );
}

fn calc_rectangle_perimeter(w: f64, h: f64) -> f64 {
    (w + h) * 2.0
}

fn calc_area_of_rectangle(w: f64, h: f64) -> f64 {
    w * h
}

fn calc_diagonal_of_rectangle(w: f64, h: f64) -> f64 {
    (w.powi(2) + h.powf(2.0)).sqrt()
}
