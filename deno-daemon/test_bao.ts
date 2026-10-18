import { getBaoTokens } from "./src/api/bao.ts";

console.log(await getBaoTokens("discordToken", "/v1/kv/data/tarascope"))