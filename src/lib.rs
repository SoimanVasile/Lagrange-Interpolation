use std::fs;


pub fn read_points(FILE_PATH: &str) -> Vec<Vec<f64>>{
    let mut points: Vec<Vec<f64>> = Vec::new();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    for line in contents.lines(){
        if line.trim().is_empty() { continue; }

        let row: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect();

        points.push(row);
    }

    points
}

pub fn create_vec_of_points(points:& Vec<Vec<f64>>, cardinal: usize) -> Vec<f64>{
    let mut vecx: Vec<f64> = Vec::new();

    for point in points{
        let x = point[cardinal];
        vecx.push(x);
    }
    
    vecx
}

