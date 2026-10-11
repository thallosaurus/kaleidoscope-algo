const appId = Deno.env.get("META_APP_ID");
const appSecret = Deno.env.get("META_APP_SECRET");
const shortToken = Deno.env.get("META_ACCESS_TOKEN");
const version = Deno.env.get("META_GRAPH_API_VERSION") ?? "v25.0";

if (!appId || !appSecret || !shortToken) {
  console.error(
    "Missing META_APP_ID, META_APP_SECRET or META_ACCESS_TOKEN.",
  );
  Deno.exit(1);
}

const params = new URLSearchParams({
  grant_type: "fb_exchange_token",
  client_id: appId,
  client_secret: appSecret,
  fb_exchange_token: shortToken,
});

const response = await fetch(
  `https://graph.facebook.com/${version}/oauth/access_token`,
  {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    body: params,
  },
);

const result = await response.json();

if (!response.ok) {
  console.error("Token exchange failed:");
  console.error(JSON.stringify(result, null, 2));
  Deno.exit(1);
}

if (!result.access_token) {
  console.error("Meta did not return an access token.");
  Deno.exit(1);
}

const expiresIn = Number(result.expires_in ?? 0);

console.log("Token exchange successful!");
console.log(`Token type: ${result.token_type ?? "unknown"}`);
console.log(
  `Expires in: ${Math.floor(expiresIn / 86400)} days`,
);
console.log(
  `Expires at: ${new Date(Date.now() + expiresIn * 1000).toISOString()}`,
);

// Print the new token only when explicitly requested.
if (Deno.args.includes("--show-token")) {
  console.log("\nNew access token:\n");
  console.log(result.access_token);
} else {
  console.log("\nRun again with --show-token to print the new token.");
}