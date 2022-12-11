use ndarray::{Array1, arr1, stack, Axis};
use rayon::prelude::*;
use std::time::Instant;

mod reader;

fn scalar_calculation(x: &Vec<f64>) -> (u128, f64) {
    let theta = [1.0, 2.0];
    let now = Instant::now();
    let mut y = 0.0;

    for x_i in x {
        y += theta[0] + theta[1] * x_i;
    }

    let elapsed = now.elapsed().as_micros();

    (elapsed, y)
}

fn scalar_rayon_calculation(x: &Vec<f64>) -> (u128, f64) {
    let theta = [1.0, 2.0];
    let now = Instant::now();
    let y: f64 = x.par_iter().map(|xi| theta[0] + theta[1] * xi).sum();
    let elapsed = now.elapsed().as_micros();

    (elapsed, y)
}

fn vectorized_calculation(x: &Array1<f64>) -> (u128, f64) {
    let theta = arr1(&[1.0, 2.0]);
    let x = stack!(Axis(1), Array1::<f64>::ones(x.len()), x.clone());
    
    let now = Instant::now();
    let y = x.dot(&theta).sum(); 
    let elapsed = now.elapsed().as_micros();

    (elapsed, y)
}
fn main() {
    let (arr, arr_nd) = reader::read_file("../../shared_data/kc_house_data.csv");
    
    let n_tests = 100000.0;
    let mut sc_total: u128 = 0;
    let mut src_total: u128 = 0;
    let mut vc_total: u128 = 0;

    println!("Running {} tests with an array of {} elements...", n_tests, arr.len());

    for _n in 0..(n_tests as i64) {
        let sc_result = scalar_calculation(&arr);
        let src_result = scalar_rayon_calculation(&arr);
        let vc_result = vectorized_calculation(&arr_nd);

        sc_total += sc_result.0;
        src_total += src_result.0;
        vc_total += vc_result.0;
        
        assert_eq!(23345871629.0, sc_result.1);
        assert_eq!(23345871629.0, src_result.1);
        assert_eq!(23345871629.0, vc_result.1);
    }

    println!("--- Scalar ---");
    println!("Elapsed (μs): {}\n", (sc_total as f64)/n_tests);
    
    println!("--- Scalar Rayon ---");
    println!("Elapsed (μs): {}\n", (src_total as f64)/n_tests);

    println!("--- Vectorized ndarray ---");
    println!("Elapsed (μs): {:.2?}\n", (vc_total as f64)/n_tests);

}
