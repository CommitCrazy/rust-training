use std::f64::consts::PI;

fn main() {
    let circle = Shape::Circle { radius: 5.0 };
    let rectangle = Shape::Rectangle { width: 4.0, height: 6.0 };
    let triangle = Shape::Triangle { base: 3.0, height: 8.0 };

    println!("Area of the circle: {:.2}", circle.area());
    println!("Area of the rectangle: {:.2}", rectangle.area());
    println!("Area of the triangle: {:.2}", triangle.area());
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius.powi(2),
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => base * height * 0.5,
        }
    }
}