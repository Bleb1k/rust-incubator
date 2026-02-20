#[derive(Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Polyline {
    points: Vec<Point>,
}

fn main() {
    let polyline = Polyline {
        points: (0..200).into_iter().map(|_| Point::default()).collect(),
    };
}
