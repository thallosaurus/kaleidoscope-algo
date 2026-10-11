import { postInstagram } from "./src/api/instagram.ts";
import { BlenderRenderProject, RenderJobDirectory } from "./src/renderer/executor.ts";

const id = Deno.args.pop();
if (!id) {
    throw new Error("missing project id")
} else {

    const tags = [
        "honk", "digitalgoose", "kaleidoscope", "blender", "render", "digital"
    ]

    console.log(id)
    const dirs = new RenderJobDirectory(id, "./output");

    const job = JSON.parse(Deno.readTextFileSync(dirs.parametersPath));
    const project: BlenderRenderProject = {
        job,
        outputPath: "./output",
        type: "blender",
        cleanup: true
    }

    console.log("upload to instagram")
    postInstagram(project, { caption: tags.map(v => "#" + v).join(" ") });
    console.log("upload finished")
}