import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
import { Kaleidoscope } from "../renderer/arguments.ts";

import { contextStorage } from "hono/context-storage"
import { randomKaleidoscope } from "../renderer/random.ts";
import { DatabaseHandler } from "./database.ts";
import { createInstagramClient } from "./instagram.ts";

type Env = {
  Variables: {
    database: DatabaseHandler,
    worker: Worker
  }
}

const VideoRouter = new Hono<Env>();
VideoRouter.post("/random", (c) => {
  const database = c.get("database");
  const worker = c.get("worker");
  const project = randomKaleidoscope({
    end: 300,
    start: 1,
    type: "animated",
    cleanup: true
  });

  database.insertJob(project);
  worker.postMessage(project);

  c.status(200);

  return c.json({
    id: project.id
  })
})

VideoRouter.post("/", zValidator("json", Kaleidoscope), (c) => {
  const data = c.req.valid("json");
  const database = c.get("database");
  const worker = c.get("worker");

  /*  database.prepare("INSERT INTO tarascope (id, parameters) VALUES (?, ?)")
      .run(data.id, JSON.stringify(data));*/
  database.insertJob(data);

  worker.postMessage(data);

  c.status(200);
  return c.json({ id: data.id })
})

const accessToken = Deno.env.get("ACCESS_TOKEN") ?? "";
const instagramUserId = Deno.env.get("INSTAGRAM_USER_ID") ?? "";

export class HttpApi {
  controller = new AbortController();
  app = new Hono<Env>()
  db = new DatabaseHandler();
  worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
  instagram = createInstagramClient({
    apiVersion: "v24.0",
    accessToken,
    instagramUserId
  })

  constructor() {
    this.controller.signal.addEventListener("abort", () => this.db.close());

    this.worker.addEventListener("message", (e) => {
      if (e.data.type == "status") {

        switch (e.data.type) {
          case "status":
            console.log(e.data)
            //this.db.prepare("INSERT INTO frames (kaleidoid, frame_count) VALUES (?, ?)").run(e.data.id, e.data.frame)
            this.db.insertFrame(e.data);
            break;

          case "finish":
            console.log(e.data);
            //this.db.prepare("UPDATE tarascope SET status = ? WHERE tarascope.id = ?").run(DbStatus.Finished, e.data.id);
            this.db.finishArtifact(e.data);
            break;
        }
      }
    })

    this.app.use(contextStorage());
    this.app.use(async (c, next) => {
      c.set("database", this.db);
      c.set("worker", this.worker);

      await next();
    })
    this.app.route("/video", VideoRouter);

    Deno.serve({ handler: this.app.fetch, signal: this.controller.signal });
  }

  stop() {
    this.controller.abort();
  }
}