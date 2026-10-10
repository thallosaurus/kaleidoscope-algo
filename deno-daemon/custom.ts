import { randomUUID } from "node:crypto";
import { renderBlenderProject } from "./src/renderer/executor.ts";
import { Kaleidoscope } from "./src/renderer/arguments.ts";

const job = Kaleidoscope.parse({
    "texture": {
        "type": "voronoi",
        "voronoi_scale": 2.015333898408686,
        "voronoi_detail": 2.3095101842315446,
        "voronoi_randomize": 0.6973992870808493
    },
    "polar": {
        "repetition": 1,
        "scaling": 0.4559221168181481,
        "rotation": 83.3370078488869,
        "pingpong": 0.9556921824348349
    },
    "composite": {
        "lens_distortion": -0.6234152351278881,
        "lens_dispersion": -0.6484810448366629,
        "hue": 0.1004369103959174,
        "saturation": 1.7572992821554414
    },
    "frames": {
        "type": "animated",
        "start": 1,
        "end": 300
    },
    "resolution": {
        "width": 1080,
        "height": 1920
    },
    "id": randomUUID()
})


await renderBlenderProject({
    job,
    outputPath: "./output",
    type: "blender"
})