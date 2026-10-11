import { createInstagramClient } from "./src/api/instagram.ts";
import { BlenderRenderProject, cleanup_frames, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

async function postInstagram(project: BlenderRenderProject, post: InstagramPost) {
  const base = "lenna.duckdns.org";
  const videoUrl = new URL(`https://${base}/${project.job.id}/video.hdr.mp4`);
  console.log("posting", videoUrl);

  const accessToken = Deno.env.get("ACCESS_TOKEN") ?? "";
  const instagramUserId = Deno.env.get("INSTAGRAM_USER_ID") ?? "";

  const client = createInstagramClient({
    apiVersion: "v24.0",
    accessToken,
    instagramUserId
  })

  await client.publishReel({
    shareToFeed: true,
    videoUrl,
    ...post,
  })
}

type InstagramPost = {
  caption: string
}

const project: BlenderRenderProject = {
  job: randomKaleidoscope({
    end: 300,
    start: 1,
    type: "animated"
  }),
  outputPath: "./output",
  type: "blender",
  cleanup: true
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
    postInstagram(project, { caption: tags.map(v => "#" + v).join(" ") });
    console.log("upload finished")
    break;
}