import { Hono } from "hono";
import { contextStorage } from "hono/context-storage"
import { DatabaseHandler } from "./database.ts";
import { WebhookHandler } from "./webhook.ts";
import { scpFileToDestionation } from "./publisher.ts";
import { InstagramRouter, PostEnv } from "./http/post.ts";
import { VideoRouter, ImageRouter } from "./http/generate.ts";
import { getBaoTokens } from "./bao.ts";
import { createInstagramClient } from "./instagram.ts";

export type Env = {
  Variables: {
    database: DatabaseHandler,
    worker: Worker
    hooks: WebhookHandler
  }
}

export class HttpApi {
  hostName = Deno.env.get("HOSTNAME") ?? "http://localhost:8080/"
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
            remote_path: `/mnt/hdd/Data/kaleidoscopes/media/${e.data.id}.mp4`,
            user: "rillo"
          }).then(() => {
            const url = new URL(`/media/${e.data.id}.mp4`, this.hostName)

            this.hooks.notify({
              id: e.data.id,
              url: url.toString()
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
    InstagramRouter.use()
    this.app.use<PostEnv>(async (c, next) => {

      const accessToken = await getBaoTokens("instagramAccessToken", "/kv/data/tarascope")
      //const instagramUserId = Deno.env.get("INSTAGRAM_USER_ID") ?? "";
      const instagramUserId = await getBaoTokens("instagramUserId", "/kv/data/tarascope")

      c.set("baseUrl", new URL(`/media/`, this.hostName))
      c.set("igClient", createInstagramClient({
        apiVersion: "v24.0",
        accessToken,
        instagramUserId
      }))

      await next();
    }).route("/post", InstagramRouter);

  }

  serve(hostname = "0.0.0.0", port = 8080, onListen = logOnListen) {
    Deno.serve({
      handler: this.app.fetch,
      onListen,
      signal: this.controller.signal,
      hostname,
      port,
    });
  }

  serveAsync(hostname = "0.0.0.0", port = 8080): Promise<Deno.NetAddr> {
    return new Promise((res, rej) => {
      this.serve(hostname, port, (addr) => {
        logOnListen(addr);
        res(addr)
      })
    })
  }

  stop() {
    this.controller.abort();
  }
}

const logOnListen = (addr: Deno.NetAddr) => console.log(`listening on ${addr.hostname}:${addr.port}`)