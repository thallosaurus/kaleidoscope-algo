import { Hono } from "hono";
import * as z from "zod";
import { zValidator } from "@hono/zod-validator";
import { postInstagram } from "../instagram.ts";
import { Env } from "../http.ts";

export const InstagramRouter = new Hono<Env>();

const InstagramRequest = z.object({
    id: z.string(),
    url: z.string()
});
export type TInstagramRequest = z.infer<typeof InstagramRequest>;

InstagramRouter.post("/upload", zValidator("json", InstagramRequest), async (c) => {
    const data = c.req.valid("json");

    const tags = [
        "honk", "digitalgoose", "kaleidoscope", "blender", "render", "digital"
    ]
    await postInstagram(data, { caption: tags.map(v => "#" + v).join(" ") })

    const database = c.get("database");
    database.setPosted(data)
})

const ExistingVideoInstagramRequest = z.object({

})