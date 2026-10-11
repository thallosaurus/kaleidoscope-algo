import { stitchVideo, stitchHdrVideo } from "./src/renderer/encoder.ts";
import { RenderJobDirectory } from "./src/renderer/executor.ts";

const dirs = new RenderJobDirectory("c518b455-8d34-429d-bac5-7765c6cdfd4e", "./output")
const video = await stitchHdrVideo(dirs);
console.log(video);
