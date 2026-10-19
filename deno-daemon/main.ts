import { HttpApi } from "./src/api/http.ts";
import { setKaleidoBytes, setLoaderBytes } from "./src/renderer/executor.ts";

setLoaderBytes(await Deno.readFile(new URL("./loader.py", import.meta.url)))
setKaleidoBytes(await Deno.readFile(new URL("./kaleido_hdr.blend", import.meta.url)))

const http = new HttpApi();
http.serve();