const outputRoot = "./output";

Deno.serve({ port: 8000 }, async (request) => {
  const match = new URL(request.url).pathname.match(/^\/([a-zA-Z0-9_-]+)$/);
  if (!match) return new Response("Not found", { status: 404 });

  const path = `${outputRoot}/${match[1]}/video.hdr.mp4`;
  console.log("request")

  try {
    const file = await Deno.open(path, { read: true });
    return new Response(file.readable, {
      headers: {
        "Content-Type": "video/mp4",
        "Cache-Control": "public, max-age=3600",
      },
    });
  } catch {
    return new Response("Video not found", { status: 404 });
  }
});