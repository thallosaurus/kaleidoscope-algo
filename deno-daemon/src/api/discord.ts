const TOKEN = Deno.env.get("DISCORD_TOKEN") ?? "";
const SERVER_ID = 1542169370896371855;

async function botSeesChannel(): Promise<void> {
    console.log(TOKEN)
    const response = await fetch(
        `https://discord.com/api/v10/guilds/${BigInt(SERVER_ID)}/channels`,
        {
            headers: {
                Authorization: `Bot ${TOKEN}`,
            },
        },
    );

    const channels = await response.json();
    console.log(channels)

    console.log(
        channels.map((channel: { id: string; name: string }) => ({
            id: channel.id,
            name: channel.name,
        })),
    );

}

export async function send(message: string, channelId: bigint): Promise<void> {
    //const token = Deno.env.get("DISCORD_TOKEN") ?? "";
    //botSeesChannel()

    const url = `https://discord.com/api/v10/channels/${channelId}/messages`

    console.log(url);

    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Authorization": `Bot ${TOKEN}`,
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            content: message
        })
    })

    if (!response.ok) {
        const body = await response.text();
        throw new Error(
            `Discord API error ${response.status}: ${body}`,
        );
    }
}