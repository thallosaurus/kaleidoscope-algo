/// <reference no-default-lib="true" />
/// <reference lib="deno.worker" />
import { BlenderRenderProject, renderBlenderProject } from "../renderer/executor.ts";

/** FIFO queue that waits for new values when empty. */
export class AsyncQueue {
    #items: BlenderRenderProject[] = [];
    #waiting: Array<(value: BlenderRenderProject) => void> = [];

    /** Add a message to the end of the queue. */
    push(value: BlenderRenderProject): void {
        const next = this.#waiting.shift();
        if (next) next(value);
        else this.#items.push(value);
        console.log(this.#items)
    }

    /** Take the next message, waiting until one is pushed if the queue is empty. */
    take(): Promise<BlenderRenderProject> {
        if (this.#items.length > 0) return Promise.resolve(this.#items.shift()!);
        return new Promise((resolve) => this.#waiting.push(resolve));
    }

    /** Run one consumer forever; each handler finishes before the next starts. */
    async run(
        //handle: (value: BlenderRenderProject) => Promise<void>,
        onError: (error: unknown, value: BlenderRenderProject) => void = (error) =>
            console.error(error),
    ): Promise<never> {
        while (true) {
            const value = await this.take();
            try {
                //await handle(value);
                console.log("new job", value)
                const output = await renderBlenderProject(value);
                console.log(output)
                postMessage({
                    type: "finish",
                    output,
                })
            } catch (error) {
                onError(error, value);
            }
        }
    }
}

const queue = new AsyncQueue();
self.onmessage = (e) => {
    //console.log(e);
    queue.push({
        type: "blender",
        outputPath: "./output",
        job: e.data,
        onUpdate: (update) => postMessage({ type: "status", ...update })
    })
}

await queue.run();