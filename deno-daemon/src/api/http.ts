import sql from "../../tarascopes.sqlite.sql" with { type: "text" }
import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";
import { Kaleidoscope } from "../renderer/arguments.ts";
import { DatabaseSync } from "node:sqlite";
import { contextStorage, getContext } from "hono/context-storage"
import { randomKaleidoscope } from "../renderer/random.ts";

type Env = {
  Variables: {
    database: DatabaseSync,
    worker: Worker
  }
}

const CreateRouter = new Hono<Env>();
CreateRouter.post("/random", (c) => {
  const database = c.get("database");
  const worker = c.get("worker");
  const project = randomKaleidoscope({
    end: 300,
    start: 1,
    type: "animated",
    cleanup: true
  });

  database.prepare("INSERT INTO tarascope (id, parameters) VALUES (?, ?)")
    .run(project.id, JSON.stringify(project));
  worker.postMessage(project);

  c.status(200);

  return c.json({
    id: project.id
  })
})

CreateRouter.post("/", zValidator("json", Kaleidoscope), (c) => {
  const data = c.req.valid("json");
  const database = c.get("database");
  const worker = c.get("worker");

  database.prepare("INSERT INTO tarascope (id, parameters) VALUES (?, ?)")
    .run(data.id, JSON.stringify(data));

  worker.postMessage(data);

  c.status(200);
  return c.json({ id: data.id })
})

export class HttpApi {
  controller = new AbortController();
  app = new Hono<Env>()
  db = new DatabaseSync("./database.db");
  worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });

  constructor() {
    this.db.exec(sql);
    this.controller.signal.addEventListener("abort", () => this.db.close());

    this.worker.addEventListener("message", (e) => {
      this.db.prepare("INSERT INTO frames (kaleidoid, frame_count) VALUES (?, ?)").run(e.data.id, e.data.frame)
    })

    this.app.use(contextStorage());
    this.app.use(async (c, next) => {
      c.set("database", this.db);
      c.set("worker", this.worker);

      await next();
    })
    this.app.route("/create", CreateRouter);

    Deno.serve({ handler: this.app.fetch, signal: this.controller.signal });
  }

  stop() {
    this.controller.abort();
  }
}