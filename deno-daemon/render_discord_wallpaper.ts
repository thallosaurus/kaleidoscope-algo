import { send as sendDiscordMessage } from "./src/api/discord.ts";

await sendDiscordMessage("hello", 1557107172163199097 as unknown as bigint)