import { stitchVideo } from "./src/renderer/encoder.ts";
import { cleanup_frames, RenderJobDirectory } from "./src/renderer/executor.ts";

const dirs = new RenderJobDirectory("c518b455-8d34-429d-bac5-7765c6cdfd4e", "./output")
const video = await stitchVideo(dirs);
await cleanup_frames({ job: video, })