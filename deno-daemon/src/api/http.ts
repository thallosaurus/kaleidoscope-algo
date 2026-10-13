import { Hono } from "hono";
import { zValidator } from "@hono/zod-validator";

import { Kaleidoscope } from "../renderer/arguments.ts";
import * as z from "zod";

const app = new Hono();

import { DatabaseSync } from "node:sqlite";
const db = new DatabaseSync("tarascopes.test.db");

db.prepare(`INSERT INTO people (name, age) VALUES (?, ?);`).run("Bob", 40);
const rows = db.prepare("SELECT id, name, age FROM people").all();

console.log("People:");
for (const row of rows) {
  console.log(row);
}
db.close();

const schema = z.object({
  name: z.string(),
  age: z.number(),
});

app.post("/create", zValidator("json", Kaleidoscope), (c) => {
    const data = c.req.valid("json");
    console.log(data);
    c.status(200);

    return c.text("ok")
})

app.post(
  "/lol",
    zValidator("json", schema),
  (c) => {
    const data = c.req.valid("json");
    // ... do something
    return c.json({
      success: true,
      message: `${data.name} is ${data.age}`,
    });
  },
);

Deno.serve(app.fetch);
