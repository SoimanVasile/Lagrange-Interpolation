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

fn sum_of_lists(list: &mut Vec<f64>, list2: &Vec<f64>) -> Vec<f64>{
    let mut list1 = list.clone();
    for ind in 0..list1.len(){
        list1[ind] += list2[ind];
    }
    list1.clone()
}

fn main() {
    let vecx = vec![1.2,2.1,3.2];
    let vecy = vec![2.4,3.0,4.1];

    let mut polynom: Vec<f64> = lj(&vecx[0], &vecy[0], &vecx);
    for ind in 1..vecx.len(){
        let xj = vecx[ind];
        let yj = vecy[ind];

        let result = lj(&xj, &yj, &vecx);
        polynom = sum_of_lists(&mut polynom, &result);
    }
    println!("{:?}", polynom);
}
