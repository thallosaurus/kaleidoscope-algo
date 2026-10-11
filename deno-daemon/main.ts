import { BlenderRenderProject, cleanup_frames, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const project: BlenderRenderProject = {
  job: randomKaleidoscope({
    end: 300,
    start: 1,
    type: "animated"
  }),
  outputPath: "./output",
  type: "blender"
}

const output = await renderBlenderProject(project);

switch (output.type) {
  case "still":
    break
  case "video":
    console.log("upload to instagram")
    //await cleanup_frames(project);
    break;
}