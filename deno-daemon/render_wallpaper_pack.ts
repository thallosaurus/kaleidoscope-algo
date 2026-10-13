import { BlenderRenderProject, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

for (let i = 0; i < 20; i++) {
    const project: BlenderRenderProject = {
        job: randomKaleidoscope({
            type: "still",
            frame: 0,
            //cleanup: true
        }, {
            width: 1920 * 2,
            height: 1080 * 2
        }),
        outputPath: "./output",
        type: "blender",
    }

    const output = await renderBlenderProject(project);
    console.log(output)
}
console.log("finished")