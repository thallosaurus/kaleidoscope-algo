import { Context, Hono, Next } from "hono";
import { zValidator } from "@hono/zod-validator";
import { Kaleidoscope, TKaleidoscope } from "../renderer/arguments.ts";
import { contextStorage } from "hono/context-storage"
import { randomKaleidoscope } from "../renderer/random.ts";
import { DatabaseHandler } from "./database.ts";
import { createInstagramClient, postInstagram } from "./instagram.ts";
import { hasWebhook, WebhookHandler } from "./webhook.ts";
import * as z from "zod";
import { scpFileToDestionation } from "./publisher.ts";

type Env = {
  Variables: {
    database: DatabaseHandler,
    worker: Worker
    hooks: WebhookHandler
  }
}

const ImageRouter = new Hono<Env>();
ImageRouter.post("/random", (c) => {
  const database = c.get("database");
  const worker = c.get("worker");
  const hooks = c.get("hooks");
  const frame = Math.floor(Math.random() * 100);

  const project = randomKaleidoscope({
    type: "still",
    frame
  });

  const hook_url = c.req.raw.headers.get("X-Callback-URL")

  if (hook_url) {
    hooks.register({
      id: project.id,
      webhook: hook_url
    });
  }

  database.insertJob(project);
  worker.postMessage(project);

  return c.json({
    id: project.id
  })
})

ImageRouter.post("/", zValidator("json", Kaleidoscope), async (c, next) => {
  const data = c.req.valid("json");
  if (data.frames.type == "still") await next(); else return c.status(401)
}, (c) => {
  const data = c.req.valid("json");
  const database = c.get("database");
  const worker = c.get("worker");

  database.insertJob(data);

  worker.postMessage(data);

  const hook_url = c.req.raw.headers.get("X-Callback-URL")

  if (hook_url) {
    const hooks = c.get("hooks");
    hooks.register({
      id: data.id,
      webhook: hook_url
    });
  }

  c.status(200);
  return c.json({ id: data.id })
})

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

  const hook_url = c.req.raw.headers.get("X-Callback-URL")

  if (hook_url) {
    const hooks = c.get("hooks");
    hooks.register({
      id: project.id,
      webhook: hook_url
    });
  }

  c.status(200);

  return c.json({
    id: project.id
  })
})

VideoRouter.post("/", zValidator("json", Kaleidoscope), async (c, next) => {
  const data = c.req.valid("json");
  if (data.frames.type == "animated") await next(); else return c.status(401)
}, (c) => {
  const data = c.req.valid("json");
  const database = c.get("database");
  const worker = c.get("worker");

  /*  database.prepare("INSERT INTO tarascope (id, parameters) VALUES (?, ?)")
      .run(data.id, JSON.stringify(data));*/
  database.insertJob(data);

  worker.postMessage(data);

  const hook_url = c.req.raw.headers.get("X-Callback-URL")

  if (hook_url) {
    const hooks = c.get("hooks");
    hooks.register({
      id: data.id,
      webhook: hook_url
    });
  }

  c.status(200);
  return c.json({ id: data.id })
})

VideoRouter.get("/", (c) => {
  const database = c.get("database");
  return c.json(database.getJobs())
})

const InstagramRequest = z.object({
  url: z.string()
});
export type TInstagramRequest = z.infer<typeof InstagramRequest>;

const InstagramRouter = new Hono<Env>();
InstagramRouter.post("/upload", zValidator("json", InstagramRequest), async (c) => {
  const data = c.req.valid("json");

  const tags = [
    "honk", "digitalgoose", "kaleidoscope", "blender", "render", "digital"
  ]
  await postInstagram(data, { caption: tags.map(v => "#" + v).join(" ") })
})


export class HttpApi {
  hostName = Deno.env.get("HOSTNAME") ?? "http://localhost:8000/"
  controller = new AbortController();
  app = new Hono<Env>()
  db = new DatabaseHandler();
  hooks = new WebhookHandler(this.db);
  worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });

  constructor() {
    this.controller.signal.addEventListener("abort", () => this.db.close());

    this.worker.addEventListener("message", (e) => {
      switch (e.data.type) {
        case "status":
          console.log(e.data)
          //this.db.prepare("INSERT INTO frames (kaleidoid, frame_count) VALUES (?, ?)").run(e.data.id, e.data.frame)
          this.db.insertFrame(e.data);
          break;

        case "finish":
          console.log("finish", e.data);
          //this.db.prepare("UPDATE tarascope SET status = ? WHERE tarascope.id = ?").run(DbStatus.Finished, e.data.id);
          this.db.finishArtifact(e.data);
          scpFileToDestionation({
            local_path: e.data.hdr,
            host: "10.0.0.2",
            remote_path: `/mnt/hdd/Data/kaleidoscopes/${e.data.id}.mp4`,
            user: "rillo"
          }).then(() => {

            const url = new URL(`/${e.data.id}.mp4`, this.hostName)

            this.hooks.notify({
              id: e.data.id,
              url
            });
          })
          break;
      }

    })

    this.app.use(contextStorage());
    this.app.use(async (c, next) => {
      c.set("database", this.db);
      c.set("worker", this.worker);
      c.set("hooks", this.hooks);

      await next();
    })
    this.app.route("/video", VideoRouter);
    this.app.route("/image", ImageRouter);
    this.app.route("/post", InstagramRouter);

    Deno.serve({ handler: this.app.fetch, signal: this.controller.signal });
  }

  stop() {
    this.controller.abort();
  }
}