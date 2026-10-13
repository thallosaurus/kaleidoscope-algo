import { BlenderRenderProject, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const project: BlenderRenderProject = {
  job: randomKaleidoscope({
    type: "still",
    frame: 0
  }),
  outputPath: "./output",
  type: "blender"
}

const output = await renderBlenderProject(project);