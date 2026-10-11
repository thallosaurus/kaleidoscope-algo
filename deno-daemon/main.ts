import { createInstagramClient } from "./src/api/instagram.ts";
import { BlenderRenderProject, cleanup_frames, renderBlenderProject } from "./src/renderer/executor.ts";
import { randomKaleidoscope } from "./src/renderer/random.ts";

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

async function postInstagram(project: BlenderRenderProject) {
  const base = "lenna.duckdns.org";
  const videoUrl = new URL(`https://${base}/${project.job}/video.hdr.mp4`);
  console.log("posting", videoUrl);

  const accessToken = Deno.env.get("ACCESS_TOKEN") ?? "";
  const instagramUserId = Deno.env.get("INSTAGRAM_USER_ID") ?? "";

  const client = createInstagramClient({
    apiVersion: "v24.0",
    accessToken,
    instagramUserId
  })

  await client.publishReel({
    caption: "testpost",
    shareToFeed: true,
    videoUrl
  })
}

const output = await renderBlenderProject(project);

switch (output.type) {
  case "still":
    break
  case "video":
    console.log("upload to instagram")
    postInstagram(project);
    break;
}