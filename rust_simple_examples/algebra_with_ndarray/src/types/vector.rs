use ndarray::arr1;

pub fn print_vector(){
    // Create a vector/1d Matrix.
    let v = arr1(&[1, 2, 3]);

    println!("--------");
    println!("{}", v);
    println!("--------\n");
}