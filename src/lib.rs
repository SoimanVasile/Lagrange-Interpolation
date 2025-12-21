use std::fs;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64,
}
pub fn read_points(FILE_PATH: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;

    let mut points = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        let line = line.trim();
        if line.is_empty() { continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            eprintln!("Skipping line {}: Not enough data", i+1);
        }

        let x = parts[0].parse::<f64>()?;
        let y = parts[1].parse::<f64>()?;

        points.push(Point{x, y});

    }

    Ok(points)
}
