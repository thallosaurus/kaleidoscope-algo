import { Hono } from "hono";
import { Env } from "../http.ts";
import { zValidator } from "@hono/zod-validator";
import { Kaleidoscope } from "../../renderer/arguments.ts";
import { randomKaleidoscope } from "../../renderer/random.ts";

export const ImageRouter = new Hono<Env>();
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

export const VideoRouter = new Hono<Env>();
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

