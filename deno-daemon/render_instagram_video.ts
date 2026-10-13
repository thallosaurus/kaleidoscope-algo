import { postInstagram } from "./src/api/instagram.ts";
import { BlenderRenderProject, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";


const project: BlenderRenderProject = {
  job: randomKaleidoscope({
    end: 300,
    start: 1,
    type: "animated",
    cleanup: true
  }),
  outputPath: "./output",
  type: "blender",
}

const tags = [
  "honk", "digitalgoose", "kaleidoscope", "blender", "render", "digital"
]

const start = performance.now();
const output = await renderBlenderProject(project);
const elapsed = performance.now() - start;
console.log(`render took ${elapsed.toFixed(1)} ms`);

switch (output.type) {
  case "still":
    break
  case "video":
    console.log("upload to instagram")
    await postInstagram(project, { caption: "🪿\n" + tags.map(v => "#" + v).join(" ") });
    console.log("upload finished")
    break;
}