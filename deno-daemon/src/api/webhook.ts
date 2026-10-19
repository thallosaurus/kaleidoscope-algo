import { TKaleidoscope } from "../renderer/arguments.ts";
import { DatabaseHandler } from "./database.ts";
import * as z from "zod";

const WebhookRequest = z.object({
    webhook: z.string()
})
type TWebhookRequest = z.infer<typeof WebhookRequest>;

export const hasWebhook = (req: object): req is TWebhookRequest => {
    return WebhookRequest.validate(req)
}

export type WebhookRequest = TKaleidoscope & TWebhookRequest

export class WebhookHandler {
    #db: DatabaseHandler

    constructor(db: DatabaseHandler) {
        this.#db = db
    }

    register(msg: { id: string } & TWebhookRequest) {
        this.#db.insertHooks(msg.id, msg.webhook)
    }

    notify(msg: {
        id: string,
        url: string
    }) {
        this.#db.getHooksForId(msg.id)
            .forEach(async (e) => {
                console.log(e, msg);
                const hook_url = e.hook_url!;

                const abort = new AbortController();

                const timeout = setTimeout(() => {
                    abort.abort();
                }, 5000);
                
                try {

                    const res = await fetch(hook_url.toString(), {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        signal: abort.signal,
                        body: JSON.stringify({
                            url: msg.url,
                            id: msg.id
                        })
                    })
                    
                    if (res.ok) {
                        console.log(`webhook ${hook_url} was successful`)
                    } else {
                        console.error(`webhook ${hook_url} returned status:`, res.status, res.statusText)
                    }
                } catch (e) {
                    console.log("webhook error", e)
                } finally {
                    clearTimeout(timeout);
                }
            })
    }
}