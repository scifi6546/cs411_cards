use nalgebra::Vector2;
use rand::prelude::*;
use tokio::{
    fs::File,
    io::{self, AsyncWriteExt},
};
fn make_writable(data: &[Vector2<f32>]) -> [Vec<f32>; 2] {
    let mut x = vec![];
    let mut y = vec![];
    for v in data.iter() {
        x.push(v.x);
        y.push(v.y);
    }
    return [x, y];
}
pub struct Points {
    points: Vec<Vector2<f32>>,
}
/// gets min along with index of min
fn find_min(points: &[Vector2<f32>]) -> (usize, Vector2<f32>) {
    points.iter().cloned().enumerate().fold(
        (usize::MAX, Vector2::new(f32::MAX, f32::MAX)),
        |acc, x| {
            if x.1.x < acc.1.x {
                x
            } else {
                acc
            }
        },
    )
}
/// gets max along with index of max
fn find_max(points: &[Vector2<f32>]) -> (usize, Vector2<f32>) {
    points.iter().cloned().enumerate().fold(
        (usize::MAX, Vector2::new(f32::MIN, f32::MIN)),
        |acc, x| {
            if x.1.x > acc.1.x {
                x
            } else {
                acc
            }
        },
    )
}
/// splits into two datasets along line with first being above line and second being below line
fn split(
    points: &[Vector2<f32>],
    line_start: Vector2<f32>,
    line_end: Vector2<f32>,
) -> (Vec<Vector2<f32>>, Vec<Vector2<f32>>) {
    let mut upper = vec![];
    let mut lower = vec![];
    for p in points.iter().copied() {
        if is_above(p, line_start, line_end) {
            upper.push(p)
        } else {
            lower.push(p)
        }
    }
    return (upper, lower);
}
fn is_above(point: Vector2<f32>, line_start: Vector2<f32>, line_end: Vector2<f32>) -> bool {
    let line = line_end - line_start;
    let point = point - line_start;
    let slope = line.y / line.x;
    let y = point.x * slope;
    y < point.y
}
/// Calculates connvex hull using quick hull
pub fn psudo_hull(points: &mut Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {
    let (min_index, min) = find_min(points);
    let (max_index, max) = find_max(points);

    points.swap_remove(min_index);
    points.swap_remove(max_index);
    let (upper, lower) = split(points, max, min);
    let mut upper_hull = hull_inner(upper, max, min);
    let mut lower_hull = hull_inner(lower, max, min);
    let mut hull = vec![max, min];
    hull.append(&mut upper_hull);
    hull.append(&mut lower_hull);
    return hull;
    /*
       let (min,max) = find_min_max;
       points.remove(min)
       points.remove(max)
       hull.push([min,max])
       let upper = find_upper(max,min,points)
       let lower  = find_lower(max,min,points)
         let upper = hull_half(find_upper(points),point,line_start,line_end);
            let lower = hull_half(find_lower(points),point,line_start,line_end);
            let mut hull = vec![point];
            hull.append(&mut upper);
            hull.append(&mut lower);
            return hull




       fn hull_half(points: &Vec<Vector2<f32>>,line_start,line_end){
            let point = find_furthest(line_start,line_end,points);
            let triangle  = [point,line_start,line_remove]
            remove_inside_triangle(triangle,points);
            let upper = hull_half(find_upper(points),point,line_start,line_end);
            let lower = hull_half(find_lower(points),point,line_start,line_end);
            let mut hull = vec![point];
            hull.append(&mut upper);
            hull.append(&mut lower);
            return hull
       }
    */
}
/// finds firhtest point from line and returns index in array
fn find_furthest(
    points: &[Vector2<f32>],
    line_start: Vector2<f32>,
    line_end: Vector2<f32>,
) -> (usize, Vector2<f32>) {
    let (index, cord, dist_squared) = points
        .iter()
        .cloned()
        .enumerate()
        .map(|p| {
            let a = p.1 - line_start;
            let line = line_end - line_start;
            (
                p.0,
                p.1,
                a.norm_squared() - (a.dot(&line).powf(2.0) / line.norm_squared()),
            )
        })
        .fold((0, Vector2::new(0.0, 0.0), 0.0), |acc, x| {
            if x.2 > acc.2 {
                x
            } else {
                acc
            }
        });
    (index, cord)
}
/// removes all points lying inside of triangle
fn remove_triangle(points: &mut Vec<Vector2<f32>>, triangle: [Vector2<f32>; 3]) {
    let new_points = points
        .iter()
        .copied()
        .filter(|point| !is_in_triangle(*point, triangle))
        .collect();
    *points = new_points;
}

/// finds out of is in triangle
/// https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
fn is_in_triangle(point: Vector2<f32>, triangle: [Vector2<f32>; 3]) -> bool {
    let d1 = sign(point, triangle[0], triangle[1]);
    let d2 = sign(point, triangle[1], triangle[2]);
    let d3 = sign(point, triangle[2], triangle[0]);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}
fn sign(p1: Vector2<f32>, p2: Vector2<f32>, p3: Vector2<f32>) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}
fn hull_inner(
    mut points: Vec<Vector2<f32>>,
    line_start: Vector2<f32>,
    line_end: Vector2<f32>,
) -> Vec<Vector2<f32>> {
    if points.len() == 0 {
        return vec![];
    }
    let (furthest_index, furthest) = find_furthest(&points, line_start, line_end);
    points.swap_remove(furthest_index);
    let triangle = [line_start, line_end, furthest];
    remove_triangle(&mut points, triangle);
    let (upper, lower) = split(&points, line_start, furthest);
    let mut upper = hull_inner(upper, line_start, furthest);
    let mut lower = hull_inner(lower, furthest, line_end);
    let mut hull = vec![furthest];
    hull.append(&mut upper);
    hull.append(&mut lower);
    return hull;
}
fn rand_points(n: usize) -> Vec<Vector2<f32>> {
    let mut rng = thread_rng();

    (0..n).map(|_| Vector2::new(rng.gen(), rng.gen())).collect()
}
/// NOTE TO FUTURE ME READ THIS
///https://steamcdn-a.akamaihd.net/apps/valve/2014/DirkGregorius_ImplementingQuickHull.pdf
#[tokio::main]
async fn main() -> io::Result<()> {
    let mut points = rand_points(1000);
    {
        let mut f = File::create("points.json").await?;
        let mut points_data =
            serde_json::to_string_pretty(&make_writable(&points)).expect("failed to parse");
        f.write_all(points_data.as_bytes()).await?;
    }

    let hull = psudo_hull(&mut points);
    {
        let mut f = File::create("hull.json").await?;
        let mut hull_data =
            serde_json::to_string_pretty(&make_writable(&hull)).expect("failed to parse");
        f.write_all(hull_data.as_bytes()).await?;
    }
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn min() {
        let points = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.5, 1.0),
        ];
        let min = find_min(&points);
        assert_eq!(min, (0, Vector2::new(0.0, 0.0)));
    }
    #[test]
    fn max() {
        let points = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.5, 1.0),
        ];
        let min = find_max(&points);
        assert_eq!(min, (1, Vector2::new(1.0, 0.0)));
    }
    #[test]
    fn basic() {
        let triangle = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.5, 1.0),
        ];
        assert_eq!(is_in_triangle(Vector2::new(0.5, 0.25), triangle), true);
        assert_eq!(is_in_triangle(Vector2::new(1.5, 0.25), triangle), false);
        assert_eq!(is_in_triangle(Vector2::new(0.5, 1.25), triangle), false);
    }
}
