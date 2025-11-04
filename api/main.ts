import { Hono } from 'hono'
import postgres from 'postgres';
import vento from 'ventojs';

//const app = new Hono();
const pg = postgres({
  host: Deno.env.get("PG_HOST"),            // Postgres ip address[s] or domain name[s]
  port: 5432,          // Postgres server port[s]
  database: Deno.env.get("PG_DB"),            // Name of database to connect to
  username: Deno.env.get("PG_USER"),            // Username of database user
  password: Deno.env.get("PG_PASS"),            // Password of database user
});

interface ShowcaseItem {
  video: string,
  gif: string,
  thumbnail: string
}

const app = new Hono();
const tmpl = vento();

const api_full = async (): Promise<ShowcaseItem> => {
  const res = await pg`SELECT * FROM showcase ORDER BY ts DESC`;
  return res as unknown as ShowcaseItem;
}

const api = async (): Promise<ShowcaseItem> => {
  const res = await pg`SELECT id, video, gif, thumbnail, ts FROM showcase ORDER BY ts DESC`;
  return res as unknown as ShowcaseItem;
}

app.get("/api", async (c) => {
  const data = await api();
  return c.json(data);
})

app.get("/", async (c) => {
  const template = await tmpl.load("index.html");
  const page = await template({ content: await api() });

  return c.render(page.content);
})

if (import.meta.main) {
  Deno.serve({ port: 8787 }, app.fetch)
}