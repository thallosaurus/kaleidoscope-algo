import { BlenderRenderProject, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const project: BlenderRenderProject = {
  job: randomKaleidoscope({
    type: "still",
    frame: 0
  }, {
    width: 1920 * 2,
    height: 1080 * 2
  }),
  outputPath: "./output",
  type: "blender",
  cleanup: true
}

const output = await renderBlenderProject(project);