# TikTok Content Posting API client for Deno

A small TypeScript client for TikTok's official OAuth and Content Posting APIs. It has no third-party dependencies and targets Deno 2.

## TikTok app setup

1. Create a TikTok for Developers app and add the **Content Posting API** product.
2. For Direct Post, enable its Direct Post configuration and request the `video.publish` scope. For inbox uploads, request `video.upload`. Add `user.info.basic` if your login flow also needs basic profile information.
3. Get the scope approved, then have each creator grant it through TikTok Login Kit. Your app also needs TikTok's audit before posts from public accounts can be public; unaudited clients are restricted to private posts.
4. Register the exact redirect URI. Keep `clientSecret` on a server; do not ship it in a browser app.

Scope approval by TikTok and consent from each creator are both required. For Direct Post, call `getCreatorInfo()` before presenting the export UI, show the returned privacy choices and applicable interaction settings, and collect the creator's explicit consent before `publishVideo()`. Pass a `privacyLevel` contained in `privacy_level_options`.

## OAuth and token persistence

```ts
import {
  createAuthorizationUrl,
  createPkcePair,
  exchangeAuthorizationCode,
  TikTokClient,
  type TikTokTokens,
} from "./tiktok_client.ts";

const config = {
  clientKey: Deno.env.get("TIKTOK_CLIENT_KEY")!,
  clientSecret: Deno.env.get("TIKTOK_CLIENT_SECRET")!,
  redirectUri: "https://your.example.com/tiktok/callback",
};

// Keep state and verifier in a server-side session and validate state on callback.
const state = crypto.randomUUID();
const pkce = await createPkcePair();
const authorizeUrl = createAuthorizationUrl(config, state, {
  scopes: ["user.info.basic", "video.publish"],
  codeChallenge: pkce.challenge,
});
console.log(authorizeUrl.href); // Redirect the creator here.

// In the callback handler, after checking state:
const tokens: TikTokTokens = await exchangeAuthorizationCode(
  config,
  new URL(request.url).searchParams.get("code")!,
  pkce.verifier,
);
// Persist tokens securely. Access tokens last about 24h; refresh tokens about 365d.
```

The token manager refreshes shortly before access-token expiry. Supply `onTokensUpdated` and persist the full returned token set because TikTok can rotate the refresh token.

## Direct Post a local video

```ts
const client = new TikTokClient(tokens, config, {
  onTokensUpdated: async (updated) => await tokenStore.save(updated),
});

const creator = await client.getCreatorInfo();
// Render creator.privacy_level_options and the duet/comment/stitch settings in
// your export UI, obtain explicit consent, and use the creator's chosen values.
const result = await client.publishVideo("./video.mp4", {
  title: "A new video #fyp",
  privacyLevel: creator.privacy_level_options[0],
  disableComment: creator.comment_disabled,
  disableDuet: creator.duet_disabled,
  disableStitch: creator.stitch_disabled,
  isAigc: false,
});

const status = await client.waitForPost(result.publishId, {
  intervalMs: 5_000,
  timeoutMs: 15 * 60_000,
});
if (status.status === "FAILED") {
  console.error(status.fail_reason);
} else if (status.status === "PUBLISH_COMPLETE") {
  console.log(status.publicly_available_post_id ?? status.publicaly_available_post_id);
}
```

If your video is already hosted on a publicly accessible HTTPS URL under a domain or URL prefix verified in your TikTok developer app, you can pass it directly instead:

```ts
const result = await client.publishVideoFromUrl("https://media.example.com/verified/video.mp4", {
  title: "A new video #fyp",
  privacyLevel: creator.privacy_level_options[0],
});
const status = await client.waitForPost(result.publishId);
```

## Optional: upload to the creator's inbox

This workflow sends the video to TikTok for the creator to review, edit and publish manually in TikTok. Authorize `video.upload` and initialize the client with that user's tokens:

```ts
const result = await client.uploadVideoToInbox("./video.mp4");
const status = await client.waitForPost(result.publishId);
// SEND_TO_USER_INBOX means TikTok delivered the inbox notification.
console.log(status.status);
```

## Transfer and API notes

- Direct Post initialization: `POST /v2/post/publish/video/init/` (`video.publish`), with either `FILE_UPLOAD` or `PULL_FROM_URL`. Inbox upload: `POST /v2/post/publish/inbox/video/init/` (`video.upload`). Status: `POST /v2/post/publish/status/fetch/` (either scope). OAuth exchange and refresh: `POST /v2/oauth/token/`.
- Local uploads use TikTok's returned `upload_url` with sequential `PUT` requests and `Content-Range`. The client uses 64,000,000-byte chunks by default. TikTok allows chunks of 5,000,000–64,000,000 bytes; a final chunk may be up to 128,000,000 bytes. Files below 5 MB are sent in one request; maximum video size is 4 GB and maximum upload duration is 10 minutes.
- Supported transfer MIME types are MP4, QuickTime/MOV and WebM. MP4 with H.264 is recommended. TikTok's restrictions also specify 23–60 FPS and dimensions from 360 to 4096 pixels.
- `waitForPost()` stops on `PUBLISH_COMPLETE`, `FAILED`, or `SEND_TO_USER_INBOX`. Processing duration is not guaranteed. Status polling is every five seconds by default, below TikTok's 30 requests/minute status limit.
- TikTok limits initialization to 6 requests/minute per user access token. The upload URL expires one hour after issuance.
- For `PULL_FROM_URL`, TikTok fetches the video itself. The URL's domain or URL prefix must be verified in your developer app; redirects are not followed.
- Videos can only be posted publicly if the creator's account supports that privacy option, the app is audited, and TikTok moderation completes. The creator's `max_video_post_duration_sec` may also be shorter than the API's 10-minute upload limit.

Official references: [Direct Post](https://developers.tiktok.com/docs/en/content-posting-api-reference-direct-post), [Upload](https://developers.tiktok.com/docs/en/content-posting-api-reference-upload-video), [Media Transfer Guide](https://developers.tiktok.com/docs/en/content-posting-api-media-transfer-guide), [Get Post Status](https://developers.tiktok.com/docs/en/content-posting-api-reference-get-video-status), [OAuth token management](https://developers.tiktok.com/docs/en/oauth-user-access-token-management), [Scopes](https://developers.tiktok.com/docs/en/scopes-overview).
