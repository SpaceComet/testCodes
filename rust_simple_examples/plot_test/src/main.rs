use plotters::prelude::*;
mod csv_func;

fn main() {
    
    let house = csv_func::read_file("../../shared_data/kc_house_data_sm_mod.csv").expect("test failed");
    let price = house.0;
    let sqft = house.1;

    // Create a new vector with the tuples (price, sqft)
    let price_sqft_iter: Vec<(f64, i32)> = price.iter().cloned().zip(sqft.iter().cloned()).collect();

    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new("out/test1.png", (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("House Sales in King County", ("sans-serif", 40.0)) // Caption
        .build_cartesian_2d(0.0..6000.0, 0..10000) // Cartesian range and type
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(
        price_sqft_iter.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
    ).unwrap();
}