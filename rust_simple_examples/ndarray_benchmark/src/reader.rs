use ndarray::{Array, Array1};

pub fn read_file(file_path: &str) -> (Vec<f64>, Array1<f64>){

    // File Reader.
    let mut csv_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .expect("Error reading the file.");

    // Iterate through all the lines/rows in the file.
    let x_arr: Vec<f64> = csv_file.records().map(|line| {
        let entire_line = line.expect("Error reading line");
        let x: f64 = entire_line[2].parse().unwrap();

        x
    }).collect();

    let x_arr_nd = Array::from_vec(x_arr.clone());

    (x_arr, x_arr_nd)
}