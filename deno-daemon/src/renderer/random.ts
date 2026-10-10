import { randomUUID } from "node:crypto";
import { Kaleidoscope, type TKaleidoscope } from "./arguments.ts";

const between = (min: number, max: number) =>
  min + Math.random() * (max - min);

const integerBetween = (min: number, max: number) =>
  Math.floor(between(min, max + 1));

function randomTexture(): TKaleidoscope["texture"] {
  switch (integerBetween(0, 5)) {
    case 0:
      return {
        type: "gabor",
        gabor_scale: between(0.00, 20),
        gabor_frequency: between(0, 20),
        gabor_anisotropy: between(0, 1),
        gabor_orientation_range: between(0, 360),
      };
    case 1:
      return {
        type: "magic",
        magic_depth: integerBetween(1, 10),
        magic_scale: between(0, 5),
        magic_dist: between(0, 5),
      };
    case 2:
      return {
        type: "noise",
        noise_scale: between(1, 15),
        noise_detail: between(0, 5),
        noise_roughness: between(0, 1),
        noise_lacunarity: between(0, 10),
        noise_distortion: between(0, 10),
      };
    // unoise, voronoi, wave und textured analog ergänzen
    default:
      return {
        type: "wave",
        wave_scale: between(0.20, 5),
        wave_distortion: between(-10, 10),
        wave_detail: between(0, 5),
        wave_detail_roughness: between(0, 1),
        wave_phase_offset: between(0, 50),
      };
  }
}

export function randomKaleidoscope(): TKaleidoscope {
  return Kaleidoscope.parse({
    id: randomUUID(),
    texture: randomTexture(),
    polar: {
      repetition: integerBetween(0, 20),
      scaling: between(0.1, 3),
      rotation: between(0, 360),
      pingpong: between(0, 1),
    },
    composite: {
      lens_distortion: between(-1, -0.5),
      lens_dispersion: between(-1, -0.5),
      hue: between(0, 1),
      saturation: between(1, 2),
    },
    frames: {
      type: "animated",
      start: 1,
      end: 50
      //frame: integerBetween(1, 100),
    },
  });
}