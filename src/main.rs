const FILE_PATH: &str = "points.txt";
use polynomialInterpolation::{create_vec_of_points, read_points};
mod utils;

fn main() {
    let vec_points: Vec<Vec<f64>> = read_points(FILE_PATH);
    let vecx = create_vec_of_points(&vec_points, 0);
    let vecy = create_vec_of_points(&vec_points, 1);

    println!("{:?}\n{:?}", vecx,vecy);

    let polynom = utils::make_the_polynomial(&vecx, &vecy);

    println!("{:?}", polynom);
}
