import React from "react";

export type LineGEOJson = {
  geometry: any;
  properties: {
    color: string;
    desc: string;
    long_name: string;
    route_id: string;
    short_name: string;
  };
};

export type Lines = LineGEOJson[];

export async function fetchGeoJson(): Promise<Lines> {
  let json = await window
    .fetch("http://localhost:3030/geojson", {
      method: "GET",
      headers: { "content-type": "application/json;charset=UTF-8" },
    })
    .then((response) => response.json())
    .then((json) => json);

  return json;
}
