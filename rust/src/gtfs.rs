use compact_str::CompactStr;
use serde::Deserialize;

use std::{io, ops::Deref};

#[derive(Debug)]
pub struct GtfsStructure {
    pub shapes: Shapes,
}

impl GtfsStructure {
    pub fn from_reader<R: io::Read>(reader: R) -> anyhow::Result<Self> {
        let mut csv_reader = csv::Reader::from_reader(reader);
        let shape_records = csv_reader.deserialize().collect::<csv::Result<Vec<ShapeRecord>>>()?;
        let shapes = Shapes(shape_records);

        Ok(GtfsStructure { shapes })
    }
}

#[derive(Debug, Deserialize)]
pub struct Shapes(Vec<ShapeRecord>);

impl Deref for Shapes {
    type Target = Vec<ShapeRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct ShapeRecord {
    pub shape_id: CompactStr,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: usize,
    pub shape_dist_traveled: Option<f32>,
}
