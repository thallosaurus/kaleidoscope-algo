import { DatabaseSync } from "node:sqlite";
import sql from "../../tarascopes.sqlite.sql" with { type: "text" }
import { TKaleidoscope } from "../renderer/arguments.ts";

enum DbStatus {
    Queued = 0,
    Rendering = 1,
    Error = 2,
    Finished = 3,
    Posted = 4,
    PostFailed = 5
}

export class DatabaseHandler {
    #db: DatabaseSync
    constructor(path = "./database.db") {
        this.#db = new DatabaseSync(path);
        this.#db.exec(sql);
    }

    close() {
        this.#db.close();
    }

    getJobs() {
        return this.#db.prepare("SELECT id, parameters, status FROM tarascope")
            .all();
    }

    getJobsForId(id: string) {
        const data = this.#db.prepare("SELECT id, status FROM tarascope WHERE id = ?")
        .get(id)!;

        return {
            id: data["id"]?.toString(),
            status: data["status"]?.toString()
        }
    }

    insertHooks(id: string, hook_url: string) {
        this.#db.prepare("INSERT INTO hooks (id, hook_url) VALUES(?, ?)")
            .run(id, hook_url)
    }

    getHooks() {
        return this.#db.prepare("SELECT id, hook_url FROM hooks")
            .all();
    }

    getHooksForId(id: string) {
        return this.#db.prepare("SELECT id, hook_url FROM hooks WHERE id = ?")
            .all(id);
    }

    insertJob(job: TKaleidoscope) {
        this.#db.prepare("INSERT INTO tarascope (id, parameters) VALUES (?, ?)")
            .run(job.id, JSON.stringify(job));
    }

    insertFrame(frame: { id: string, frame: number }) {
        this.#db.prepare("INSERT INTO frames (kaleidoid, frame_count) VALUES (?, ?)")
            .run(frame.id, frame.frame)
    }

    private setStatus(status: DbStatus, frame: { id: string }) {
        this.#db.prepare("UPDATE tarascope SET status = ? WHERE tarascope.id = ?")
            .run(status, frame.id);
    }
    finishArtifact(frame: { id: string }) {
        this.setStatus(DbStatus.Finished, frame)
    }
    setPosted(frame: { id: string }) {
        this.setStatus(DbStatus.Posted, frame)
    }
    setPostFailed(frame: { id: string }) {
        this.setStatus(DbStatus.PostFailed, frame)
    }
}