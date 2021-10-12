use omny::{
    static_data::{GtfsDataSource, StaticDataWorker},
    OMNYGeoJson,
};
use std::path::PathBuf;
use warp::{http::Method, Filter};

const FILES_PATH: &str = "/Users/parkertimmerman/.omny";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // update all GTFS data
    let worker = StaticDataWorker::new(PathBuf::from(FILES_PATH));
    worker
        .update_all()
        .await
        .expect("failed to update some GTFS data?");

    // get the path for the subway GTFS data
    let mta_subway_gtfs = worker
        .path_for(GtfsDataSource::MtaSubway)
        .await
        .expect("resources don't exist?");
    let path_gtfs = worker
        .path_for(GtfsDataSource::Path)
        .await
        .expect("PATH resources don't exist");
    let lirr_gtfs = worker
        .path_for(GtfsDataSource::MtaLirr)
        .await
        .expect("LIRR resources don't exist");
    let metro_north_gtfs = worker
        .path_for(GtfsDataSource::MtaMetroNorth)
        .await
        .expect("Metro North resources don't exist");
    let nj_transit_rail = worker
        .path_for(GtfsDataSource::NjTransitRail)
        .await
        .expect("NJ Transit resources don't exist");

    let omny_geojson = OMNYGeoJson::from_gtfs(
        &mta_subway_gtfs,
        &path_gtfs,
        &lirr_gtfs,
        &metro_north_gtfs,
        &nj_transit_rail,
    )
    .expect("failed to make OMNY GeoJSON");

    let geojson = warp::path("geojson")
        .and(warp::path::end())
        .map(move || warp::reply::json(&omny_geojson.routes));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ]);

    let routes = warp::get().and(geojson).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
