export async function scpFileToDestionation(r: {
    local_path: string,
    user: string,
    host: string,
    remote_path: string
}) {
    const command = new Deno.Command("scp", {
        args: [r.local_path, `${r.user}@${r.host}:${r.remote_path}`],
        stdout: "piped",
        stderr: "piped",
    });

    const { code, stdout, stderr } = await command.output();

    if (code === 0) {
        console.log("Datei erfolgreich übertragen.");
    } else {
        console.error("Fehler:", new TextDecoder().decode(stderr));
    }
}