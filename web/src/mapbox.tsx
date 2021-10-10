import React from "react";
import mapboxgl from "mapbox-gl";
import GeoJSON from "geojson";

import DATA from "./d.json";

// @ts-ignore
mapboxgl.accessToken = process.env.REACT_APP_MAPBOX_TOKEN;

type MapboxComponentState = {
  mapLoaded: boolean;
};

export class MapComponent extends React.Component<{}, MapboxComponentState> {
  map: mapboxgl.Map | null;
  // @ts-ignore
  mapContainer: HTMLElement;

  constructor(props: {}) {
    super(props);
    this.state = {
      mapLoaded: false,
    };

    this.map = null;
  }

  componentDidMount() {
    this.map = new mapboxgl.Map({
      container: this.mapContainer,
      style:
        "mapbox://styles/parkmycar/ckukemmoj2e2c17s633lovjgc?optimize=true",
      center: [-73.959, 40.735],
      bearing: 29,
      minZoom: 9,
      zoom: 13,
      hash: false,
      maxBounds: [
        [-74.8113, 40.1797],
        [-73.3584, 41.1247],
      ],
      maxPitch: 0,
    });

    this.map.on("load", () => {
      this.setState({ mapLoaded: true });
    });
  }

  componentDidUpdate() {
    if (this.state.mapLoaded) {
      this.renderLines();
    }
  }

  renderLines() {
    const layerId = "d";

    console.log("rendering!");

    // @ts-ignore
    this.map.addSource(layerId, {
      type: "geojson",
      // @ts-ignore
      data: DATA,
    });

    console.log(DATA);

    const layer = {
      id: layerId,
      type: "line",
      source: layerId,
      layout: {
        "line-join": "miter",
        "line-cap": "round",
      },
      paint: {
        "line-width": ["interpolate", ["linear"], ["zoom"], 8, 1, 13, 2, 14, 5],
        "line-color": "#FF6600",
      },
    };

    // @ts-ignore
    this.map.addLayer(layer);
    // @ts-ignore
    this.map.on("click", layerId, (e) => {
      console.log("clicked on layer!");
    });
  }

  componentWillUnmount() {
    this.map?.remove();
  }

  render() {
    const style = {
      position: "absolute" as "absolute",
      top: 0,
      bottom: 0,
      width: "100%",
    };

    // @ts-ignore
    return <div style={style} ref={(el) => (this.mapContainer = el)} />;
  }
}
