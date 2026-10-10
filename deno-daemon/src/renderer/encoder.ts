import { type RenderJobDirectory } from "./executor.ts";

function stitchGif(dirs: RenderJobDirectory) {
    const input = dirs.projectFolderPath + "/frame_%05d.png"
    const outputPath = dirs.projectFolderPath + "/video.gif"
    return {
        cmd: new Deno.Command("ffmpeg", {
            args: [
                "-framerate",
                "60",
                "-i",
                input,
                outputPath
            ]
        }),
        outputPath
    }
}

function stitchMp4(dirs: RenderJobDirectory) {
    const input = dirs.projectFolderPath + "/frame_%05d.png"
    const outputPath = dirs.projectFolderPath + "/video.mp4"
    return {
        cmd: new Deno.Command("ffmpeg", {
            args: [
                "-framerate",
                "60",
                "-i",
                input,
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                outputPath
            ]
        }),
        outputPath
    }
}

export async function stitchVideo(dirs: RenderJobDirectory) {
    const gifTask = stitchGif(dirs)
    const mp4Task = stitchMp4(dirs);

    await Promise.all([gifTask.cmd.spawn(), mp4Task.cmd.spawn()])

    return {
        mp4: mp4Task.outputPath,
        gif: gifTask.outputPath
    }
}