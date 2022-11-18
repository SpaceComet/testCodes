use ndarray::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub fn print_matrix(){
    // Create a 2d matrix. (3x2)
    let _a = arr2(&[[1, 2],
                                                     [3, 4],
                                                     [5, 6]]);
    
    // Another way to create it.
    let a = arr2(&[[1, 2], [3, 4], [5, 6]]);

    // Another way to create a 1d, 2d or 3d matrix.
    let b = array![[1, 2], [3, 4], [5, 6]];

    println!("--------");
    println!("{}", a);
    println!("{:?}", a.dim());
    println!("{:?}", a.len());
    println!("--");
    println!("{}", b);
    println!("{:?}", b.dim());
    println!("{:?}", b.len());
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

pub fn print_matrix_identity(){
    let i_matrix = Array::<f64, _>::eye(5);
    //let i_matrix_a = Array2::<f64>::eye(5);

    println!("--------");
    println!("{}", i_matrix);
    println!("--------\n");
}

pub fn print_matrix_addition(){
    let ma = array![[1, 2], [3, 4], [50, 60]];

    // Create a 3x2 matrix full of 2
    let mb = Array::from_elem((3, 2), 2);

    println!("--------");
    println!("{}", ma+mb);
    println!("--------\n");
}

pub fn print_matrix_scalar_mul_div(){
    let scalar = 2;
    let ma = array![[10, 2], [30, 4], [50, 6]];

    println!("--------");
    println!("{}", scalar*&ma);
    println!("{}", ma/scalar);
    println!("--------\n");
}

pub fn print_matrix_dot_product(){
    let v = arr1(&[1, 2]);
    let ma = array![[10, 2], [30, 4], [50, 6]];

    println!("--------");
    println!("{}", &ma.dot(&v));
    
    // the next is not valid and it will panic
    //println!("{}", &v.dot(&ma));
    println!("--------\n");
}