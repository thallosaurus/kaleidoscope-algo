import { renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

const random = randomKaleidoscope({
  end: 300,
  start: 1,
  type: "animated"
});
console.log(random);

const random_still = randomKaleidoscope({
  type: "still",
  frame: 0
})

await renderBlenderProject({
  job: random_still,
  outputPath: "./output",
  type: "blender"
})