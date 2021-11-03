use nalgebra::Vector2;
pub struct Points {
    points: Vec<Vector2<f32>>,
}
/// Finds convex hull, requires at least one point to generate hull
pub fn hull(points: &Points) {
    let min_x =
        points.points.iter().fold(
            (Vector2::new(0.0, 0.0)),
            |acc, x| if x.x < acc.x { *x } else { acc },
        );
    let max_x =
        points.points.iter().fold(
            (Vector2::new(0.0, 0.0)),
            |acc, x| if x.x > acc.x { *x } else { acc },
        );
    println!("min x: {}", min_x);
    println!("max x: {}", max_x);
}
fn main() {
    let points = Points {
        points: vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(0.5, 0.5),
            Vector2::new(1.0, 0.0),
        ],
    };
    hull(&points);
    println!("Hello, world!");
}
