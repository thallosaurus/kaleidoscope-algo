import { send as sendDiscordMessage } from "./src/api/discord.ts";

await sendDiscordMessage("hello", 1559280633350590584 as unknown as bigint)