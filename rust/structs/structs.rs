struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("Point co-ords: ({}, {})", point.x, point.y);
}
