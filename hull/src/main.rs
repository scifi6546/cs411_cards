use nalgebra::Vector2;
use rand::prelude::*;
pub struct Points {
    points: Vec<Vector2<f32>>,
}
/// NOTE TO FUTURE ME READ THIS
///https://steamcdn-a.akamaihd.net/apps/valve/2014/DirkGregorius_ImplementingQuickHull.pdf
impl Points {
    /// Builds random inside of unit square
    fn rand(number_points: usize) -> Self {
        let mut rng = rand::thread_rng();
        Points {
            points: (0..number_points)
                .map(|_| Vector2::new(rng.gen(), rng.gen()))
                .collect(),
        }
    }
}
/// Finds convex hull, requires at least one point to generate hull
pub fn hull(mut points: Points) -> Vec<Vector2<f32>> {
    let min_x = points
        .points
        .iter()
        .fold(Vector2::new(f32::MAX, f32::MAX), |acc, x| {
            if x.x < acc.x {
                *x
            } else {
                acc
            }
        });
    let max_x = points
        .points
        .iter()
        .fold(Vector2::new(f32::MIN, f32::MIN), |acc, x| {
            if x.x > acc.x {
                *x
            } else {
                acc
            }
        });
    println!("min x: {}", min_x);
    println!("max x: {}", max_x);
    let (max_distance_point, max_dist) = points
        .points
        .iter()
        .map(|p| {
            let a = p - min_x;
            let line = max_x - min_x;
            (
                *p,
                a.norm_squared() - (a.dot(&line).powf(2.0) / line.norm_squared()),
            )
        })
        .fold((Vector2::new(0.0, 0.0), 0.0), |acc, x| {
            if x.1 > acc.1 {
                x
            } else {
                acc
            }
        });
    let mut hull = vec![max_x, min_x];
    loop {
        if points.points.len() == 0 {
            break;
        }
        let (max_distance_point, max_dist) = points
            .points
            .iter()
            .map(|p| {
                let a = p - min_x;
                let line = max_x - min_x;
                (
                    *p,
                    a.norm_squared() - (a.dot(&line).powf(2.0) / line.norm_squared()),
                )
            })
            .fold((Vector2::new(0.0, 0.0), 0.0), |acc, x| {
                if x.1 > acc.1 {
                    x
                } else {
                    acc
                }
            });
        hull.push(max_distance_point);
        let filtered = points
            .points
            .iter()
            .filter(|p| **p != max_distance_point)
            .copied();
        points.points = filtered.collect();
    }
    println!(
        "max distance point: {},distance: {}",
        max_distance_point, max_dist
    );
    let distances_squared = points.points.iter().map(|p| {
        let a = p - min_x;
        let line = max_x - min_x;
        a.norm_squared() - (a.dot(&line).powf(2.0) / line.norm_squared())
    });
    for d in distances_squared {
        println!("distance squared: {}", d);
    }
    let outside_triangle = points
        .points
        .iter()
        .filter(|x| in_triangle([min_x, max_x, max_distance_point], **x));
    for out in outside_triangle {
        println!("outside triangle: {}", out);
    }
    todo!("should never get to here, need to clean up")
}
fn sign(p1: Vector2<f32>, p2: Vector2<f32>, p3: Vector2<f32>) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}
/// finds out of is in triangle
/// https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
fn in_triangle(triangle: [Vector2<f32>; 3], point: Vector2<f32>) -> bool {
    let d1 = sign(point, triangle[0], triangle[1]);
    let d2 = sign(point, triangle[1], triangle[2]);
    let d3 = sign(point, triangle[2], triangle[0]);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}
fn main() {
    println!(
        "norm_squared: {}",
        Vector2::new(1.0f32, 0.0f32).norm_squared()
    );
    println!(
        "norm_squared: {}",
        Vector2::new(1.0f32, 1.0f32).norm_squared()
    );
    let points = Points::rand(10);
    hull(points);
    println!("Hello, world!");
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        let triangle = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.5, 1.0),
        ];
        assert_eq!(in_triangle(triangle, Vector2::new(0.5, 0.25)), true);
        assert_eq!(in_triangle(triangle, Vector2::new(1.5, 0.25)), false);
    }
}
