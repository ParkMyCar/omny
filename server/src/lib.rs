#[macro_use]
extern crate tracing;

use geojson::Feature;
use gtfs_structures::Gtfs;
use itertools::Itertools;
use rgb::RGB8;
use serde::Serialize;
use std::{ops::Deref, path::Path, sync::Arc};

pub mod static_data;

#[derive(Clone, Debug, Serialize)]
pub struct RouteProperties {
    route_id: String,
    short_name: String,
    long_name: String,
    desc: Option<String>,
    color: Option<String>,
}

impl RouteProperties {
    fn from_gtfs(route: &gtfs_structures::Route) -> Self {
        RouteProperties {
            route_id: route.id.clone(),
            short_name: route.short_name.clone(),
            long_name: route.long_name.clone(),
            desc: route.desc.clone(),
            color: route.route_color.map(|rgb| rgb_as_hex(&rgb)),
        }
    }

    fn into_json(self) -> serde_json::Map<String, serde_json::value::Value> {
        let mut properties = serde_json::Map::with_capacity(5);

        properties.insert("route_id".to_string(), self.route_id.into());
        properties.insert("short_name".to_string(), self.short_name.into());
        properties.insert("long_name".to_string(), self.long_name.into());

        if let Some(desc) = self.desc {
            properties.insert("desc".to_string(), desc.into());
        }
        if let Some(color) = self.color {
            properties.insert("color".to_string(), color.into());
        }

        properties
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Route(Feature);

impl Deref for Route {
    type Target = Feature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct OMNYGeoJson {
    pub routes: Vec<Route>,
    // gtfs: Arc<Gtfs>,
}

impl OMNYGeoJson {
    pub fn from_gtfs<P: AsRef<Path>>(
        subway: P,
        path: P,
        lirr: P,
        metro_north: P,
        nj_transit: P,
    ) -> anyhow::Result<Self> {
        let subway_gtfs = Gtfs::from_path(subway).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let path_gtfs = Gtfs::from_path(path).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let lirr_gtfs = Gtfs::from_path(lirr).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let metro_north_gtfs = Gtfs::from_path(metro_north).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let nj_transit_gtfs = Gtfs::from_path(nj_transit).map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let subway_shapes = OMNYGeoJson::trip_shapes(&subway_gtfs);
        let path_shapes = OMNYGeoJson::trip_shapes(&path_gtfs);
        let lirr_shapes = OMNYGeoJson::trip_shapes(&lirr_gtfs);
        let metro_north_shapes = OMNYGeoJson::trip_shapes(&metro_north_gtfs);
        let nj_transit_shapes = OMNYGeoJson::trip_shapes(&nj_transit_gtfs);


        let routes = subway_shapes
            .into_iter()
            .map(|feature| Route(feature))
            .chain(path_shapes.into_iter().map(|feature| Route(feature)))
            .chain(lirr_shapes.into_iter().map(|feature| Route(feature)))
            .chain(metro_north_shapes.into_iter().map(|feature| Route(feature)))
            .chain(nj_transit_shapes.into_iter().map(|feature| Route(feature)))
            .collect::<Vec<Route>>();

        println!("num routes: {}", routes.len());

        Ok(OMNYGeoJson {
            routes,
            // gtfs: Arc::new(gtfs),
        })
    }

    fn trip_shapes(gtfs: &Gtfs) -> Vec<Feature> {
        gtfs.trips
            .values()
            // filter to trips with a `shape_id`
            .filter_map(|trip| trip.shape_id.as_ref().map(|shape_id| (trip, shape_id)))
            // dedupe by `shape_id`
            .unique_by(|(_trip, shape_id)| shape_id.clone())
            // map to `Feature`
            .filter_map(|(trip, shape_id)| OMNYGeoJson::feature_from_shape(&gtfs, &shape_id, trip))
            .collect()
    }

    fn feature_from_shape(
        gtfs: &Gtfs,
        shape_id: &str,
        trip: &gtfs_structures::Trip,
    ) -> Option<Feature> {
        gtfs.shapes
            .get(shape_id)
            // map a GTFS shape to a list of coordinates
            .map(|shapes| {
                shapes
                    .iter()
                    .map(|shape| vec![shape.longitude, shape.latitude])
                    .collect::<geojson::LineStringType>()
            })
            // "strongly type" these coordinates for GeoJSON
            .map(|coords| geojson::Geometry::new(geojson::Value::LineString(coords)))
            // get properties for the route associated with this trip
            .map(|geom| {
                let properties = gtfs
                    .routes
                    .get(&trip.route_id)
                    .map(|route| RouteProperties::from_gtfs(&route).into_json());
                Feature {
                    bbox: None,
                    geometry: Some(geom),
                    id: None,
                    properties,
                    foreign_members: None,
                }
            })
    }
}

fn rgb_as_hex(rgb: &RGB8) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

#[cfg(test)]
mod tests {
    use super::rgb_as_hex;
    use rgb::RGB8;

    #[test]
    fn rgb_to_hex() {
        let white = RGB8 {
            r: 255,
            g: 255,
            b: 255,
        };
        assert_eq!(rgb_as_hex(&white), "#FFFFFF");

        let black = RGB8 { r: 0, g: 0, b: 0 };
        assert_eq!(rgb_as_hex(&black), "#000000");

        let orange = RGB8 {
            r: 255,
            g: 99,
            b: 25,
        };
        assert_eq!(rgb_as_hex(&orange), "#FF6319");
    }
}
