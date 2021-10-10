import React from "react";
import mapboxgl from "mapbox-gl";
import { fetchGeoJson, Lines } from "./api";

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
      // @ts-ignore
      fetchGeoJson().then((lines) => {
        this.renderLines(lines);
      });
    }
  }

  renderLines(lines: Lines) {
    for (const line of lines) {
      const layerId = line.properties.route_id;

      if (this.map?.getSource(layerId)) {
        // @ts-ignore
        this.map.getSource(layerId).setData(line);
      } else {
        this.map?.addSource(layerId, {
          type: "geojson",
          // @ts-ignore
          data: line,
        });
      }

      if (!this.map?.getLayer(layerId)) {
        const layer = {
          id: layerId,
          type: "line",
          source: layerId,
          layout: {
            "line-join": "miter",
            "line-cap": "round",
          },
          paint: {
            "line-width": [
              "interpolate",
              ["linear"],
              ["zoom"],
              8,
              1,
              13,
              2,
              14,
              5,
            ],
            "line-offset": [
              "interpolate",
              ["linear"],
              ["zoom"],
              8,
              ["get", "offset"],
              13,
              ["*", ["get", "offset"], 1.5],
              14,
              ["*", ["get", "offset"], 3],
            ],
          },
        };
        if (line.properties.color) {
          // @ts-ignore
          layer.paint["line-color"] = line.properties.color;
        }
        // @ts-ignore
        this.map.addLayer(layer);
      }

      // @ts-ignore
      this.map.on("click", layerId, (e) => {
        console.log("clicked on layer! id: " + layerId);
      });
    }
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
