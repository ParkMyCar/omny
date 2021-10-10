use gtfs_geojson::features;


use rust::gtfs;
use std::{
    fs,
    path::PathBuf,
};

fn main() {
    let path = PathBuf::from(std::env::args().nth(1).expect("no path given!"));
    let file = fs::File::open(&path).expect("failed to open csv!");
    let gtfs_record = gtfs::GtfsStructure::from_reader(&file).expect("failed to parse csv!");

    println!("num records: {}", gtfs_record.shapes.len());
}
