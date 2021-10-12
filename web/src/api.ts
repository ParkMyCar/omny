import React from "react";

export type LineGEOJson = {
  geometry: {
    type: string;
    coordinates: number[][];
  };
  properties: {
    color: string;
    offset: string;
    desc: string;
    long_name: string;
    route_id: string;
    short_name: string;
  };
};

export type Lines = LineGEOJson[];

export async function fetchGeoJson(): Promise<Lines> {
  let json = await window
    .fetch("http://192.168.1.203:3030/geojson", {
      method: "GET",
      headers: { "content-type": "application/json;charset=UTF-8" },
    })
    .then((response) => response.json())
    .then((json) => json);

  return json;
}
