import { type RenderJobDirectory } from "./executor.ts";

function stitchGif(dirs: RenderJobDirectory) {
    return new Deno.Command("ffmpeg", {
        args: [
            "-framerate",
            "60",
            "-i",
            dirs.projectFolderPath + "/frame_%05d.png",
            dirs.projectFolderPath + "/video.gif",
        ]
    })
}

function stitchMp4(dirs: RenderJobDirectory) {
    return new Deno.Command("ffmpeg", {
        args: [
            "-framerate",
            "60",
            "-i",
            dirs.projectFolderPath + "/frame_%05d.png",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            dirs.projectFolderPath + "/video.mp4",
        ]
    })
}

export async function stitchVideo(dirs: RenderJobDirectory) {
    return await Promise.all([stitchGif(dirs).spawn(), stitchMp4(dirs).spawn()])
}