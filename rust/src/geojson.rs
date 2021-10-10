use crate::gtfs;
use geo_types::{
    Coordinate,
    MultiLineString,
};

impl From<gtfs::Shapes> for geojson::Feature {
    fn from(shapes: gtfs::Shapes) -> Self {
        // shapes.into_iter().map(|record| Coordinate { x: record.shape_pt_lat, y: record.shape_pt_lon })

        todo!()
    }
}
