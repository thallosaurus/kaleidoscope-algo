import { joinGlobs } from "jsr:@std/path@^1.1.4/join-globs";
import { BlenderRenderProject, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const job = randomKaleidoscope({
    type: "animated",
    "start": 1,
    "end": 100
  });

  job.texture = {
    "type": "textured",
    "file_path": new URL("assets/momo_head_lighter.png", import.meta.url).pathname
  };

const project: BlenderRenderProject = {
  job: job,
  outputPath: "./output",
  type: "blender",
  cleanup: true
}

const output = await renderBlenderProject(project);