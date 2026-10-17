import { DatabaseSync } from "node:sqlite";
import sql from "../../tarascopes.sqlite.sql" with { type: "text" }
import { TKaleidoscope } from "../renderer/arguments.ts";

enum DbStatus {
    Queued = 0,
    Rendering = 1,
    Error = 2,
    Finished = 3,
}

export class DatabaseHandler {
    #db: DatabaseSync
    constructor(db = new DatabaseSync("./database.db")) {
        this.#db = db;
        this.#db.exec(sql);
    }

    close() {
        this.#db.close();
    }

    getJobs() {
        return this.#db.prepare("SELECT id, parameters, status FROM tarascope")
            .all();
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

    finishArtifact(frame: { id: string }) {
        this.#db.prepare("UPDATE tarascope SET status = ? WHERE tarascope.id = ?")
            .run(DbStatus.Finished, frame.id);
    }
}