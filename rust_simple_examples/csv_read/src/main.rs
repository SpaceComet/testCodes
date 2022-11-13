fn main() {
    // File Reader.
    // csv file from https://www.kaggle.com/datasets/harlfoxem/housesalesprediction
    let mut csv_file = csv::Reader::from_path("kc_house_data_sm.csv").expect("Error reading the file.");

    // Another way to declare the file reader.
    let mut _csv_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("kc_house_data_sm.csv")
        .expect("Error reading the file.");

    // Iterate through all the lines/rows in the file.
    for line in csv_file.records(){
        // get the entire line
        let entire_line = line.expect("Error reading line");

        // Get an specific column.
        let price: f64 = entire_line[2].parse().unwrap();
        
        println!("{:?}", entire_line);
        println!("Price: {:?}", price);
    }
}
