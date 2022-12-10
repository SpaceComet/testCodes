use ndarray::{Array1, arr1, stack, Axis};
use rayon::prelude::*;
use std::time::Instant;

mod reader;

fn scalar_calculation(x: Vec<f64>){
    let theta = [1.0, 2.0];
    let now = Instant::now();
    let mut y = 0.0;

    for x_i in x {
        y += theta[0] + theta[1] * x_i;
    }

    let elapsed = now.elapsed();
    println!("--- Scalar ---");
    println!("Result: {:.2?}", y);
    println!("Elapsed: {:.2?}\n", elapsed);
}

fn scalar_rayon_calculation(x: Vec<f64>){
    let theta = [1.0, 2.0];
    let now = Instant::now();
    let y: f64 = x.par_iter().map(|xi| theta[0] + theta[1] * xi).sum();
    let elapsed = now.elapsed();
    
    println!("--- Scalar Rayon ---");
    println!("Result: {:.2?}", y);
    println!("Elapsed: {:.2?}\n", elapsed);
}

fn vectorized_calculation(x: Array1<f64>){
    let theta = arr1(&[1.0, 2.0]);
    let x = stack!(Axis(1), Array1::<f64>::ones(x.len()), x);
    let now = Instant::now();
    let y = x.dot(&theta).sum(); 
    let elapsed = now.elapsed();

    println!("--- Vectorized ndarray ---");
    println!("Result: {:.2?}", y);
    println!("Elapsed: {:.2?}\n", elapsed);
}
fn main() {
    let (arr, arr_nd) = reader::read_file("../../shared_data/kc_house_data_sm_mod.csv");
    
    scalar_calculation(arr.clone());
    scalar_rayon_calculation(arr);
    vectorized_calculation(arr_nd);
}
