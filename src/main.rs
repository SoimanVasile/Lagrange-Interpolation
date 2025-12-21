mod utils;
mod lib;
use lib::{read_points, Point};

const FILE_PATH: &str = "points.txt";

fn main(){
    let points = match read_points(FILE_PATH) {
        Ok(p) => p,
        Err(e) => {
            eprint!("Failed to read point {}", e);
            return ;
        }
    };

    println!("Points: {:?}", points);

    let polynom = utils::make_the_polynomial(&points);

    println!("{:?}", polynom);
}
