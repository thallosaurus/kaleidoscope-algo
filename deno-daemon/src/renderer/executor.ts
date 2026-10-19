import { TKaleidoscope } from "./arguments.ts";
import { exists } from "jsr:@std/fs/exists";
import { KaleidoVideo, stitchVideo } from "./encoder.ts";
import { expandGlob } from "jsr:@std/fs/expand-glob";


//import loader_bytes from "../loader.py" with { type: "bytes" };
//import kaleido_bytes from "../kaleido.blend" with { type: "bytes" };

const BLENDER_PATH_MAC = "/Applications/Blender.app/Contents/MacOS/Blender";
let loader_bytes = await Deno.readFile(new URL("../../loader.py", import.meta.url));
let kaleido_bytes = await Deno.readFile(new URL("../../kaleido_hdr.blend", import.meta.url));

export function setLoaderBytes(b: Uint8Array<ArrayBuffer>) {
    loader_bytes = b
}

export function setKaleidoBytes(b: Uint8Array<ArrayBuffer>) {
    kaleido_bytes = b
}

export class RenderJobDirectory {
    #directory: string
    #id: string
    constructor(id: string, dir: string) {
        console.log(dir);
        this.#id = id
        this.#directory = dir
    }

    get directory() {
        return this.#directory
    }

    get id() {
        return this.#id;
    }

    get projectFolderPath() {
        return `${this.#directory}/${this.id}`
    }

    get blenderStdoutPath() {
        return `${this.projectFolderPath}/blender.stdout.log`
    }

    get blenderStderrPath() {
        return `${this.projectFolderPath}/blender.stderr.log`
        //return `${this.projectFolderPath()}/blender.stderr.log`
    }

    get parametersPath() {
        return `${this.projectFolderPath}/parameters.json`
        //return `${this.projectFolderPath}/parameters.json`
    }

    get blenderProjectPath() {
        return `${this.projectFolderPath}/project.blend`
    }

    get loaderPath() {
        return `${this.projectFolderPath}/loader.py`
    }

    get blenderFramePath() {
        return `${this.projectFolderPath}/frame_#####`
        //return `${this.projectFolderPath()}/frame_#####`
    }

    get blenderNativeLogPath() {
        return `${this.projectFolderPath}/blender.log`
        //return `${this.projectFolderPath()}/blender.log`
    }

    get socket() {
        return `${this.projectFolderPath}/status.socket`
    }

    async createProjectFolder() {
        if (!await exists(this.#directory, { isDirectory: true })) { await Deno.mkdir(this.#directory) }
        await Deno.mkdir(this.projectFolderPath)
    }
}

type RenderStatusCallback = (update: { id: string; frame: number }) => void;

async function pumpLines(stream: ReadableStream<Uint8Array>, onLine: (line: string) => void): Promise<void> {
    const reader = stream.pipeThrough(new TextDecoderStream())
        .getReader();

    let pending = "";

    try {
        while (true) {
            const { value, done } = await reader.read();
            if (done) break;

            pending += value;
            const lines = pending.split("\n");
            pending = lines.pop() ?? "";
            for (const line of lines) onLine(line);
        }

        if (pending) onLine(pending);
    } finally {
        reader.releaseLock();
    }
}

async function handleUpdateSocket(dirs: RenderJobDirectory, onUpdate: RenderStatusCallback, abort = new AbortController()) {
    const listener = Deno.listen({
        transport: "unix",
        path: dirs.socket
    })

    const conn = await listener.accept();
    const decoder = new TextDecoder();
    const buffer = new Uint8Array(4096);
    let pending = "";

    conn.write(new TextEncoder().encode("hello" + "\n"))

    abort.signal.addEventListener("abort", () => {
        listener.close();
        Deno.removeSync(dirs.socket)
    })

    try {
        while (true) {
            const n = await conn.read(buffer);
            if (n === null) break; // connection was closed

            pending += decoder.decode(buffer.subarray(0, n), { stream: true });
            const lines = pending.split("\n");
            pending = lines.pop() ?? "";

            for (const line of lines) {
                if (!line.trim()) continue;
                const update = JSON.parse(line);

                conn.write(new TextEncoder().encode("hello" + "\n"))
                onUpdate(update)
            }
        }

        if (pending.trim()) {
            console.log("Last message:", JSON.parse(pending));
        }
    } finally {
        conn.close();
    }
}

async function renderJob(job: TKaleidoscope, dirs: RenderJobDirectory, onUpdate: RenderStatusCallback = console.log, abort = new AbortController()) {
    const socket = handleUpdateSocket(dirs, onUpdate, abort);

    const child = renderCommand(dirs, job).spawn();

    abort.signal.addEventListener("abort", () => {
        child.kill();
    })

    const stdoutLog = Deno.open(dirs.blenderStdoutPath, {
        write: true,
        create: true,
        truncate: true,
    });

    const stderrLog = Deno.open(dirs.blenderStderrPath, {
        write: true,
        create: true,
        truncate: true,
    });

    const [stdoutFile, stderrFile] = await Promise.all([stdoutLog, stderrLog]);

    const stdoutTask = pumpLines(child.stdout, (line) => {
        stdoutFile.writeSync(new TextEncoder().encode(`${line}\n`));
        console.info(line);
    })

    const stderrTask = pumpLines(child.stderr, (line) => {
        stderrFile.writeSync(new TextEncoder().encode(`${line}\n`));
        console.error(line);
    })

    try {
        const status = await child.status;
        await Promise.all([stdoutTask, stderrTask, socket]);
        return status;
    } finally {
        stdoutFile.close();
        stderrFile.close();
        await Deno.remove(dirs.socket);
    }
}

function renderCommand(dirs: RenderJobDirectory, job: TKaleidoscope) {
    const argument = () => {
        const bytes = new TextEncoder().encode(JSON.stringify(job));

        switch (job.frames.type) {
            case "animated":
                return [
                    dirs.blenderProjectPath,
                    "--factory-startup",
                    "--log-file",
                    dirs.blenderNativeLogPath,
                    "-s", String(job.frames.start), "-e", String(job.frames.end),
                    "-o",
                    dirs.blenderFramePath,
                    "-Y",
                    "-P",
                    dirs.loaderPath,
                    "-f",
                    String(0),
                    "-b",
                    "-a",
                    "--",
                    bytes.toBase64()
                ]

            case "still":
                return [
                    dirs.blenderProjectPath,
                    "--factory-startup",
                    "-o",
                    dirs.blenderFramePath,
                    "-Y",
                    "-P",
                    dirs.loaderPath,
                    "-f",
                    String(job.frames.frame),
                    "-b",
                    "--",
                    bytes.toBase64()
                ]
        };

    }
    return new Deno.Command(BLENDER_PATH_MAC, {
        args: argument(),
        env: {
            STATUS_SOCKET: dirs.socket,
            //PYTHONUNBUFFERED: "1",
        },
        stdout: "piped",
        stderr: "piped"
    })
}

export type BlenderRenderProject = {
    type: "blender",
    job: TKaleidoscope,
    outputPath: string,
    //cleanup: boolean,
    onUpdate?: RenderStatusCallback,
    //kaleidoscope_project: Uint8Array<ArrayBuffer>
    //loader: Uint8Array<ArrayBuffer>
};

type KaleidoStill = {
    type: "still",
    img: string,
    id: string
}

export async function renderBlenderProject({
    type,
    job,
    outputPath,
    onUpdate,
}: BlenderRenderProject,
    kaleidoscope_project = kaleido_bytes,
    loader = loader_bytes): Promise<KaleidoVideo | KaleidoStill> {
    const dirs = new RenderJobDirectory(job.id, outputPath)
    await dirs.createProjectFolder()

    //project file
    await Deno.writeFile(dirs.blenderProjectPath, kaleidoscope_project)

    //loader file
    await Deno.writeFile(dirs.loaderPath, loader)

    //write parameters
    const paramsJson = JSON.stringify(job);
    await Deno.writeTextFile(dirs.parametersPath, paramsJson, { create: true });
    const j = await renderJob(job, dirs, onUpdate ?? console.log);
    if (job.frames.type == "animated") {
        if (j.success) {
            const v = await stitchVideo(dirs)
            if (job.frames.cleanup) {
                await cleanup_frames({ job, outputPath });
            }
            return v;
        } else {
            throw new Error("command didnt yield a success")
        }
    } else {
        return {
            type: "still",
            img: dirs.projectFolderPath + "/frame_00000.png",
            id: dirs.id
        }
    }
    //console.log(dirs);
}

async function deleteFiles(directory: string, pattern: string) {
    for await (const entry of expandGlob(pattern, {
        root: directory,
        includeDirs: false,
    })) {
        if (entry.isFile) {
            console.log(`Deleting ${entry.path}`);
            await Deno.remove(entry.path);
        }
    }
}

type CleanupRequst = {
    job: {
        id: string,
    },
    outputPath: string
};

export async function cleanup_frames({ job, outputPath }: CleanupRequst) {
    const dirs = new RenderJobDirectory(job.id, outputPath)
    await Deno.rename(dirs.projectFolderPath + "/frame_00000.png", dirs.projectFolderPath + "/thumbnail.png")
    await deleteFiles(dirs.projectFolderPath, "frame_*.png")
}