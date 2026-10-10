import { randomUUID } from "node:crypto";
import { Kaleidoscope, TKaleidoscope } from "./src/renderer/arguments.ts";
import { renderBlenderProject } from "./src/renderer/executor.ts";
import z from "zod";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const job = Kaleidoscope.parse({
  "id": randomUUID(),
  "composite": {
    "hue": 0.4,
    "lens_dispersion": -0.6,
    "lens_distortion": -0.6,
    "saturation": 1.1
  },
  "frames": {
    "type": "animated",
    "start": 1,
    "end": 100
  },
  "polar": {
    "pingpong": 0.5,
    "repetition": 4,
    "rotation": 5,
    "scaling": 1.0
  },
  "texture": {
    "type": "noise",
    "noise_detail": 0.5,
    "noise_distortion": 0.1,
    "noise_lacunarity": 1.0,
    "noise_roughness": 0.1,
    "noise_scale": 3.0,
  }
});

const random = randomKaleidoscope();
console.log(random);

renderBlenderProject(random, "./output")