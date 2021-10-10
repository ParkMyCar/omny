use omny::OMNYGeoJson;
use std::path::PathBuf;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    let path = PathBuf::from(std::env::args().nth(1).expect("no path given!"));
    let omny_geojson = OMNYGeoJson::from_gtfs(&path).expect("failed to make OMNY GeoJSON");

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

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
