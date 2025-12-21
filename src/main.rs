use std::fs;
const FILE_PATH: &str = "points.txt";

fn lj(xj: &f64,yj: &f64, list_x: &Vec<f64>) -> Vec<f64>{

    let mut prod: f64 = 1 as f64;
    for xi in list_x{
        if xi != xj{
            prod *= xj-xi;
        }
    }

    let mut list_lj:Vec<f64> = Vec::new();
    list_lj.push(1 as f64);
    for xi in list_x{
        if xi != xj{
            let copy = xi*(-1 as f64);
            let mut list_mul_copy = list_lj.clone();
            for x in &mut list_mul_copy{
                *x = *x*copy;
            }
            for ind in 1..list_lj.len(){
                list_lj[ind] += list_mul_copy[ind-1];
            }
            list_lj.push(list_mul_copy[list_mul_copy.len()-1]);
        }
    }
    let constanta = yj/prod;
    for ind in 0..list_lj.len(){
        list_lj[ind] *= constanta;
    }
    list_lj
}

fn sum_of_lists(list: &Vec<f64>, list2: &Vec<f64>) -> Vec<f64>{
    let mut list1 = list.clone();
    for ind in 0..list1.len(){
        list1[ind] += list2[ind];
    }
    list1.clone()
}

fn read_points() -> Vec<Vec<f64>>{
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
fn create_vec_of_points(points:& Vec<Vec<f64>>, cardinal: usize) -> Vec<f64>{
    let mut vecx: Vec<f64> = Vec::new();

    for point in points{
        let x = point[cardinal];
        vecx.push(x);
    }
    
    vecx
}

fn main() {
    let vec_points: Vec<Vec<f64>> = read_points();
    let vecx = create_vec_of_points(&vec_points, 0);
    let vecy = create_vec_of_points(&vec_points, 1);

    println!("{:?}\n{:?}", vecx,vecy);

    let mut polynom: Vec<f64> = lj(&vecx[0], &vecy[0], &vecx);
    for ind in 1..vec_points.len(){
        let xj = vecx[ind];
        let yj = vecy[ind];

        let result = lj(&xj, &yj, &vecx);
        polynom = sum_of_lists(&polynom, &result);
    }
    println!("{:?}", polynom);
}
