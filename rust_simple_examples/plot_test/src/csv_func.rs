pub fn read_file(path: &str) -> Result<(Vec<f64>, Vec<i32>), Box<dyn std::error::Error>> {
    let mut house_price_list: Vec<f64> = Vec::new();
    let mut house_sqft_list: Vec<i32> = Vec::new();

    let mut csv_file = csv::Reader::from_path(path)?;
    //println!("{}", path);

    for line in csv_file.records(){
        // Get entire line
        let entire_line = line?;
        house_price_list.push(entire_line[2].parse::<f64>().unwrap() / 1000.0);
        house_sqft_list.push(entire_line[5].parse().unwrap());
    }

    Ok((house_price_list, house_sqft_list))
}