import { Hono } from "hono";
import * as z from "zod";
import { zValidator } from "@hono/zod-validator";
import { InstagramClient, postInstagram } from "../instagram.ts";
import { Env } from "../http.ts";
import { DatabaseHandler } from "../database.ts";
//import data from "../../../tarascopes.sqlite.sql#denoRawImport=text.ts";

export type PostEnv = Env & {
    Variables: {
        baseUrl: URL,
        igClient: InstagramClient
    }
};

export const InstagramRouter = new Hono<PostEnv>();

const createHdrLink = (base: URL, id: string) => {
    return new URL(`${id}.mp4`, base)
}

const InstagramRequest = z.object({
    id: z.string(),
    url: z.string()
});
export type TInstagramRequest = z.infer<typeof InstagramRequest>;

const uploadAndMarkDatabase = async (data: TInstagramRequest, database: DatabaseHandler, client: InstagramClient) => {
    const tags = [
        "honk", "digitalgoose", "kaleidoscope", "blender", "render", "digital"
    ]
    await postInstagram(data, { caption: tags.map(v => "#" + v).join(" ") }, client)

    database.setPosted(data)
}

InstagramRouter.post("/upload", zValidator("json", InstagramRequest), async (c) => {
    const data = c.req.valid("json");
    const database = c.get("database");

    uploadAndMarkDatabase(data, database, c.get("igClient"));
})

InstagramRouter.post("/upload/:projectId", async (c) => {
    const { projectId } = c.req.param();
    const id = z.uuidv4().parse(projectId);

    const database = c.get("database");
    const url = c.get("baseUrl");
    console.log(url);

    const data = database.getJobsForId(id);

    try {
        uploadAndMarkDatabase({
            id,
            url: createHdrLink(url, data.id!).toString()
        }, database, c.get("igClient"));
    } catch (e: unknown) {
        return c.json({
            ...(e as Error),
            msg: "error",
        })
    }
})

const ExistingVideoInstagramRequest = z.object({

})