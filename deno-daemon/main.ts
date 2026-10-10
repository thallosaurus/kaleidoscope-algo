import { renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const random = randomKaleidoscope({
  end: 300,
  start: 1,
  type: "animated"
});
console.log(random);

await renderBlenderProject({
  job: random,
  outputPath: "./output",
  type: "blender"
})