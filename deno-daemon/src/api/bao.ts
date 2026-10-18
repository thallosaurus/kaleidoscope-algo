export const getBaoTokens = async (key_name: string, path: string) => {
    const baoAddress = Deno.env.get("BAO_ADDRESS") ?? "";
    const req = await fetch(baoAddress + path, {
        headers: {
            "X-Vault-Token": Deno.env.get("BAO_TOKEN") ?? ""
        }
    })

    const j = await req.json();

    return j.data.data[key_name]
}