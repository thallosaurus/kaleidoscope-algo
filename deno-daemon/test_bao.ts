import { getBaoTokens } from "./src/api/bao.ts";

console.log(await getBaoTokens("discordToken", "/kv/data/tarascope"))