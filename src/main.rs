use std::process::Command;
mod utils;
mod lib;
use lib::{read_points, Point};

fn main(){

    let output = Command::new("python3")
        .current_dir("image_process")
        .arg("image_process.py")
        .output()
        .expect("Failed to execute the python file");

    if !output.status.success() {
        eprintln!("Python Error: {}", String::from_utf8_lossy(&output.stderr));
        return ;
    }

    println!("Pyhton Result: {}", String::from_utf8_lossy(&output.stdout));

    let points_x = match read_points("image_process/points_x.txt") {
        Ok(p) => p,
        Err(e) => {
            eprint!("Failed to read point {}", e);
            return ;
        }
    };

    let poly_x = utils::make_the_polynomial(&points_x);

    let points_y = match read_points("image_process/points_y.txt") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read point {}", e);
            return;
        },
    };

    let poly_y = utils::make_the_polynomial(&points_y);

println!("\n=== PARAMETRIC POLYNOMIALS ===");
    println!("x(t) = {:?}", poly_x);
    println!("y(t) = {:?}", poly_y);
    println!("Range: t goes from 0 to {:.1}", (points_x.len() - 1) as f64);
}
