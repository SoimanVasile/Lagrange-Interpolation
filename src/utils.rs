use crate::Point;

fn add_polys(poly_a: &[f64], poly_b: &[f64]) -> Vec<f64>{
    // Adds 2 polynomials
    let max_len = poly_a.len().max(poly_b.len());
    let mut result = vec![0.0; max_len];

    for (i, &val) in poly_a.iter().enumerate() { result[i] += val; }
    for (i, &val) in poly_b.iter().enumerate() { result[i] += val; }

    result
}

fn multiply_poly_by_linear(poly: &[f64], root: f64) -> Vec<f64> {
    //Multiplys a polynomial to a linear equation of form x-root=0

    let mut result = vec![0.0; poly.len() + 1];

    for (i, &val) in poly.iter().enumerate(){
        result[i+1] += val;
        result[i] -= val * root;
    }

    result
}


pub fn lagrange_term(target_ind: usize, points: &[Point]) -> Vec<f64>{
    let xj = points[target_ind].x;
    let yj = points[target_ind].y;

    let mut numerator_poly = vec![1.0];
    let mut denominator = 1.0;

    for (i, &point) in points.iter().enumerate(){
        if i == target_ind {continue; }

        numerator_poly = multiply_poly_by_linear(&numerator_poly, point.x);

        denominator *= xj-point.x;
    }

    let scalar = yj/denominator;

    numerator_poly.iter()
        .map(|c| c * scalar)
        .collect()
}

pub fn make_the_polynomial(points: &[Point]) -> Vec<f64> {

    let mut result = vec![0.0; points.len()];

    for (i, point) in points.iter().enumerate(){
        let lj = lagrange_term(i, &points);
        result = add_polys(&result, &lj)
    }
    
    result
}

pub fn evaluate_poly(coeffs: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    for &c in coeffs.iter().rev() {
        result = result * x + c;
    }
    result
}
