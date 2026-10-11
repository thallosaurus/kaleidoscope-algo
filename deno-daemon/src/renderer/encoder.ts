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

function stitchHdrMp4(dirs: RenderJobDirectory) {
    const input = dirs.projectFolderPath + "/frame_%05d.png"
    const outputPath = dirs.projectFolderPath + "/video.hdr.mp4"
    return {
        cmd: new Deno.Command("ffmpeg", {
            args: [
                "-framerate",
                "60",
                "-i",
                input,
                "-c:v",
                "libx265",
                "-pix_fmt",
                "yuv420p10le",
                "-color_primaries",
                "bt2020",
                "-color_trc",
                "smpte2084",
                "-colorspace",
                "bt2020nc",
                "-tag:v",
                "hvc1",
                "-movflags",
                "+faststart",
                outputPath
            ]
        }),
        outputPath
    }
}

export type KaleidoVideo = {
    type: "video",
    id: string,
    mp4: string,
    hdr: string,
    gif: string
}

export async function stitchVideo(dirs: RenderJobDirectory): Promise<KaleidoVideo> {
    const gifTask = stitchGif(dirs)
    const mp4Task = stitchMp4(dirs);
    const hdrTask = stitchHdrMp4(dirs);

    await Promise.all([gifTask.cmd.output(), mp4Task.cmd.output(), hdrTask.cmd.output()])

    return {
        id: dirs.id,
        type: "video",
        mp4: mp4Task.outputPath,
        hdr: hdrTask.outputPath,
        gif: gifTask.outputPath
    }
}