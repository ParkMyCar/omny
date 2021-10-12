mod urls;
mod worker;

pub use worker::StaticDataWorker;

#[derive(Copy, Clone, Debug)]
pub enum GtfsDataSource {
    MtaBronxBus,
    MtaBrooklynBus,
    MtaManhattanBus,
    MtaQueensBus,
    MtaStatenIslandBus,
    MtaSubway,
    Path,
    MtaLirr,
    MtaMetroNorth,
    NjTransitRail,
}

impl GtfsDataSource {
    pub const fn url(&self) -> &'static str {
        use GtfsDataSource::*;

        match self {
            MtaBronxBus => urls::MTA_BRONX_BUS_GTFS,
            MtaBrooklynBus => urls::MTA_BROOKLYN_BUS_GTFS,
            MtaManhattanBus => urls::MTA_MANHATTAN_BUS_GTFS,
            MtaQueensBus => urls::MTA_QUEENS_BUS_GTFS,
            MtaStatenIslandBus => urls::MTA_STATEN_ISLAND_GTFS,
            MtaSubway => urls::MTA_SUBWAY_GTFS,
            Path => urls::PATH_GTFS,
            MtaLirr => urls::MTA_LIRR_GTFS,
            MtaMetroNorth => urls::MTA_METRO_NORTH_GTFS,
            NjTransitRail => urls::NJ_TRANSIT_RAIL_GTFS,
        }
    }

    pub const fn filename(&self) -> &'static str {
        use GtfsDataSource::*;

        match self {
            MtaBronxBus => "mta_bronx_bus",
            MtaBrooklynBus => "mta_brooklyn_bus",
            MtaManhattanBus => "mta_manhattan_bus",
            MtaQueensBus => "mta_queens_bus",
            MtaStatenIslandBus => "mta_staten_island_bus",
            MtaSubway => "mta_subway",
            Path => "path",
            MtaLirr => "mta_lirr",
            MtaMetroNorth => "mta_metro_north",
            NjTransitRail => "nj_transit_rail",
        }
    }

    pub const fn all() -> [GtfsDataSource; 10] {
        [
            GtfsDataSource::MtaBronxBus,
            GtfsDataSource::MtaBrooklynBus,
            GtfsDataSource::MtaManhattanBus,
            GtfsDataSource::MtaQueensBus,
            GtfsDataSource::MtaStatenIslandBus,
            GtfsDataSource::MtaSubway,
            GtfsDataSource::Path,
            GtfsDataSource::MtaLirr,
            GtfsDataSource::MtaMetroNorth,
            GtfsDataSource::NjTransitRail,
        ]
    }
}
