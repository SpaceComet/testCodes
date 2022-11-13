use ndarray::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub fn print_matrix(){
    // Create a 2d matrix. (2x3)
    let _a = arr2(&[[1, 2],
                                                     [3, 4],
                                                     [5, 6]]);
    
    // Another way to create it.
    let a = arr2(&[[1, 2], [3, 4], [5, 6]]);

    println!("--------");
    println!("{}", a);
    println!("--------\n");
}

pub fn print_matrix_ones(){
    // Create a 2x3 matrix of ones.
    // Specify both the element type and dimensionality:
    let ones1 = Array::<f64, Ix2>::ones((2, 3));
    let ones2 = Array2::<f64>::ones((2, 3));

    // Create a 3x3 matrix of ones.
    // Specify just the element type and infer the dimensionality:
    let ones3 = Array::<f64, _>::ones((3, 3));

    println!("--------");
    println!("{}", ones1);
    println!("--------\n");

    println!("--------");
    println!("{}", ones2);
    println!("--------\n");

    println!("--------");
    println!("{}", ones3);
    println!("--------\n");
}

pub fn print_matrix_zeros(){
    // Create a 3x3 matrix of zeros.
    // Specify just the element type and infer the dimensionality:
    let zeros3 = Array::<f64, _>::zeros((3, 3));

    println!("--------");
    println!("{}", zeros3);
    println!("--------\n");
}

pub fn print_matrix_rand(){
    // create a 2x5 matrix of random numbers from 0 to 10
    let r = Array::random((2, 5), Uniform::new(0., 10.));

    println!("--------");
    println!("{:1.3}", r);
    println!("--------\n");
}