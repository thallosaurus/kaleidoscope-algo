/** Minimal-dependency Deno client for TikTok's official Content Posting API. */

import { BlenderRenderProject } from "../renderer/executor.ts";

const API = "https://open.tiktokapis.com";
const MIN_CHUNK = 5_000_000;
const MAX_CHUNK = 64_000_000;
const MAX_FILE = 4 * 1024 ** 3;

export type TikTokPrivacy =
    | "PUBLIC_TO_EVERYONE"
    | "MUTUAL_FOLLOW_FRIENDS"
    | "FOLLOWER_OF_CREATOR"
    | "SELF_ONLY";

export interface TikTokTokens {
    accessToken: string;
    refreshToken: string;
    expiresAt: number;
    refreshExpiresAt: number;
    openId: string;
    scopes: string[];
    tokenType: string;
}

export interface TikTokOAuthConfig {
    clientKey: string;
    clientSecret: string;
    redirectUri: string;
}

export interface CreatorInfo {
    creator_avatar_url?: string;
    creator_username?: string;
    creator_nickname?: string;
    privacy_level_options: TikTokPrivacy[];
    comment_disabled: boolean;
    duet_disabled: boolean;
    stitch_disabled: boolean;
    max_video_post_duration_sec: number;
}

export interface VideoPostOptions {
    title?: string;
    privacyLevel: TikTokPrivacy;
    disableComment?: boolean;
    disableDuet?: boolean;
    disableStitch?: boolean;
    videoCoverTimestampMs?: number;
    brandContentToggle?: boolean;
    brandOrganicToggle?: boolean;
    isAigc?: boolean;
    chunkSize?: number;
    contentType?: "video/mp4" | "video/quicktime" | "video/webm";
}

export interface VideoUrlPostOptions extends Omit<VideoPostOptions, "chunkSize" | "contentType"> { }

export interface PostStatus {
    status:
    | "PROCESSING_UPLOAD"
    | "PROCESSING_DOWNLOAD"
    | "SEND_TO_USER_INBOX"
    | "PUBLISH_COMPLETE"
    | "FAILED"
    | string;
    fail_reason?: string;
    publicly_available_post_id?: string[];
    publicaly_available_post_id?: string[]; // Current API response spelling in some docs.
    uploaded_bytes?: number;
    downloaded_bytes?: number;
}

export interface PublishResult {
    publishId: string;
    uploadUrl?: string;
}

export interface TikTokClientOptions {
    /** Called after refresh so rotated tokens can be durably stored. */
    onTokensUpdated?: (tokens: TikTokTokens) => void | Promise<void>;
    fetch?: typeof fetch;
}

interface ApiEnvelope<T> {
    data?: T;
    error?: { code?: string; message?: string; log_id?: string };
}

export class TikTokApiError extends Error {
    constructor(
        message: string,
        readonly code?: string,
        readonly logId?: string,
        readonly httpStatus?: number,
    ) {
        super(message);
        this.name = "TikTokApiError";
    }
}

function tokenResponse(raw: Record<string, unknown>, prior?: TikTokTokens): TikTokTokens {
    if (typeof raw.access_token !== "string" || typeof raw.expires_in !== "number") {
        throw new TikTokApiError(`Invalid OAuth response: ${JSON.stringify(raw)}`);
    }
    const now = Date.now();
    return {
        accessToken: raw.access_token,
        refreshToken: typeof raw.refresh_token === "string" ? raw.refresh_token : prior?.refreshToken ?? "",
        expiresAt: now + raw.expires_in * 1000,
        refreshExpiresAt: now + (typeof raw.refresh_expires_in === "number" ? raw.refresh_expires_in : 0) * 1000,
        openId: typeof raw.open_id === "string" ? raw.open_id : prior?.openId ?? "",
        scopes: typeof raw.scope === "string" ? raw.scope.split(",").filter(Boolean) : prior?.scopes ?? [],
        tokenType: typeof raw.token_type === "string" ? raw.token_type : "Bearer",
    };
}

async function oauthRequest(
    config: TikTokOAuthConfig,
    values: Record<string, string>,
    fetchImpl: typeof fetch,
): Promise<Record<string, unknown>> {
    const body = new URLSearchParams({ client_key: config.clientKey, client_secret: config.clientSecret, ...values });
    const response = await fetchImpl(`${API}/v2/oauth/token/`, {
        method: "POST",
        headers: { "Content-Type": "application/x-www-form-urlencoded", "Cache-Control": "no-cache" },
        body,
    });
    const json = await response.json() as Record<string, unknown>;
    if (!response.ok || typeof json.access_token !== "string") {
        throw new TikTokApiError(
            String(json.error_description ?? json.error ?? `OAuth failed (${response.status})`),
            typeof json.error === "string" ? json.error : undefined,
            typeof json.log_id === "string" ? json.log_id : undefined,
            response.status,
        );
    }
    return json;
}

/** Exchange an OAuth authorization code (PKCE verifier required for desktop apps). */
export async function exchangeAuthorizationCode(
    config: TikTokOAuthConfig,
    code: string,
    codeVerifier?: string,
    fetchImpl: typeof fetch = fetch,
): Promise<TikTokTokens> {
    const values: Record<string, string> = {
        code,
        grant_type: "authorization_code",
        redirect_uri: config.redirectUri,
    };
    if (codeVerifier) values.code_verifier = codeVerifier;
    return tokenResponse(await oauthRequest(config, values, fetchImpl));
}

/** Refresh tokens. Always persist the result: TikTok may rotate the refresh token. */
export async function refreshTikTokTokens(
    config: TikTokOAuthConfig,
    current: TikTokTokens,
    fetchImpl: typeof fetch = fetch,
): Promise<TikTokTokens> {
    const raw = await oauthRequest(config, {
        grant_type: "refresh_token",
        refresh_token: current.refreshToken,
    }, fetchImpl);
    return tokenResponse(raw, current);
}

/** Create the Login Kit URL. Store state and verifier in the caller's session. */
export function createAuthorizationUrl(
    config: TikTokOAuthConfig,
    state: string,
    options: { scopes?: string[]; codeChallenge?: string; codeChallengeMethod?: "S256" | "plain" } = {},
): URL {
    const url = new URL("https://www.tiktok.com/v2/auth/authorize/");
    url.search = new URLSearchParams({
        client_key: config.clientKey,
        response_type: "code",
        scope: (options.scopes ?? ["user.info.basic", "video.publish"]).join(","),
        redirect_uri: config.redirectUri,
        state,
        ...(options.codeChallenge ? { code_challenge: options.codeChallenge, code_challenge_method: options.codeChallengeMethod ?? "S256" } : {}),
    }).toString();
    return url;
}

/** PKCE helper for desktop or public clients. */
export async function createPkcePair(): Promise<{ verifier: string; challenge: string }> {
    const bytes = crypto.getRandomValues(new Uint8Array(32));
    const verifier = base64Url(bytes);
    const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(verifier));
    return { verifier, challenge: base64Url(new Uint8Array(digest)) };
}

function base64Url(bytes: Uint8Array): string {
    let binary = "";
    for (const byte of bytes) binary += String.fromCharCode(byte);
    return btoa(binary).replaceAll("+", "-").replaceAll("/", "_").replace(/=+$/, "");
}

/** Small in-memory token manager; supply a persistence callback for token rotation. */
export class TikTokTokenManager {
    private refreshInFlight?: Promise<string>;

    constructor(
        private tokens: TikTokTokens,
        private readonly config: TikTokOAuthConfig,
        private readonly onTokensUpdated?: TikTokClientOptions["onTokensUpdated"],
        private readonly fetchImpl: typeof fetch = fetch,
        private readonly refreshSkewMs = 5 * 60_000,
    ) { }

    get current(): TikTokTokens {
        return structuredClone(this.tokens);
    }

    async accessToken(): Promise<string> {
        if (this.tokens.expiresAt > Date.now() + this.refreshSkewMs) return this.tokens.accessToken;
        this.refreshInFlight ??= this.refresh();
        try {
            return await this.refreshInFlight;
        } finally {
            this.refreshInFlight = undefined;
        }
    }

    private async refresh(): Promise<string> {
        if (this.tokens.refreshExpiresAt && this.tokens.refreshExpiresAt <= Date.now()) {
            throw new TikTokApiError("TikTok refresh token has expired; authorize the user again.", "refresh_token_expired");
        }
        this.tokens = await refreshTikTokTokens(this.config, this.tokens, this.fetchImpl);
        await this.onTokensUpdated?.(this.current);
        return this.tokens.accessToken;
    }
}

export class TikTokClient {
    private readonly fetchImpl: typeof fetch;
    private readonly tokenManager: TikTokTokenManager;

    constructor(tokens: TikTokTokens, config: TikTokOAuthConfig, options: TikTokClientOptions = {}) {
        this.fetchImpl = options.fetch ?? fetch;
        this.tokenManager = new TikTokTokenManager(tokens, config, options.onTokensUpdated, this.fetchImpl);
    }

    get tokens(): TikTokTokens {
        return this.tokenManager.current;
    }

    async getCreatorInfo(): Promise<CreatorInfo> {
        return await this.api<CreatorInfo>("/v2/post/publish/creator_info/query/", {});
    }

    /** Direct-post a local video. Query creator info and obtain explicit user consent first. */
    async publishVideo(filePath: string, options: VideoPostOptions): Promise<PublishResult> {
        const file = await Deno.open(filePath, { read: true });
        try {
            const size = (await file.stat()).size;
            if (size <= 0 || size > MAX_FILE) throw new RangeError("Video must be between 1 byte and 4 GB.");
            const chunkSize = chooseChunkSize(size, options.chunkSize);
            const totalChunkCount = Math.floor(size / chunkSize) || 1;
            const contentType = options.contentType ?? mimeFromPath(filePath);
            const init = await this.api<{ publish_id: string; upload_url: string }>("/v2/post/publish/video/init/", {
                post_info: {
                    ...(options.title !== undefined ? { title: options.title } : {}),
                    privacy_level: options.privacyLevel,
                    ...(options.disableComment !== undefined ? { disable_comment: options.disableComment } : {}),
                    ...(options.disableDuet !== undefined ? { disable_duet: options.disableDuet } : {}),
                    ...(options.disableStitch !== undefined ? { disable_stitch: options.disableStitch } : {}),
                    ...(options.videoCoverTimestampMs !== undefined ? { video_cover_timestamp_ms: options.videoCoverTimestampMs } : {}),
                    // TikTok's current reference marks this field required; false is the default.
                    brand_content_toggle: options.brandContentToggle ?? false,
                    ...(options.brandOrganicToggle !== undefined ? { brand_organic_toggle: options.brandOrganicToggle } : {}),
                    ...(options.isAigc !== undefined ? { is_aigc: options.isAigc } : {}),
                },
                source_info: { source: "FILE_UPLOAD", video_size: size, chunk_size: chunkSize, total_chunk_count: totalChunkCount },
            });
            if (!init.upload_url) throw new TikTokApiError("TikTok did not return an upload URL.");
            await this.uploadFile(file, init.upload_url, size, chunkSize, totalChunkCount, contentType);
            return { publishId: init.publish_id, uploadUrl: init.upload_url };
        } finally {
            file.close();
        }
    }

    /** Direct-post a video hosted at a public URL on a TikTok-verified domain/prefix. */
    async publishVideoFromUrl(videoUrl: string, options: VideoUrlPostOptions): Promise<PublishResult> {
        const parsedUrl = new URL(videoUrl);
        if (parsedUrl.protocol !== "https:") {
            throw new TypeError("TikTok PULL_FROM_URL requires a publicly accessible HTTPS URL.");
        }
        const data = await this.api<{ publish_id: string }>("/v2/post/publish/video/init/", {
            post_info: {
                ...(options.title !== undefined ? { title: options.title } : {}),
                privacy_level: options.privacyLevel,
                ...(options.disableComment !== undefined ? { disable_comment: options.disableComment } : {}),
                ...(options.disableDuet !== undefined ? { disable_duet: options.disableDuet } : {}),
                ...(options.disableStitch !== undefined ? { disable_stitch: options.disableStitch } : {}),
                ...(options.videoCoverTimestampMs !== undefined ? { video_cover_timestamp_ms: options.videoCoverTimestampMs } : {}),
                brand_content_toggle: options.brandContentToggle ?? false,
                ...(options.brandOrganicToggle !== undefined ? { brand_organic_toggle: options.brandOrganicToggle } : {}),
                ...(options.isAigc !== undefined ? { is_aigc: options.isAigc } : {}),
            },
            source_info: { source: "PULL_FROM_URL", video_url: parsedUrl.href },
        });
        return { publishId: data.publish_id };
    }

    /** Upload to the creator's TikTok inbox for editing and manual publishing. */
    async uploadVideoToInbox(filePath: string, chunkSize?: number, contentType?: VideoPostOptions["contentType"]): Promise<PublishResult> {
        const file = await Deno.open(filePath, { read: true });
        try {
            const size = (await file.stat()).size;
            if (size <= 0 || size > MAX_FILE) throw new RangeError("Video must be between 1 byte and 4 GB.");
            const actualChunkSize = chooseChunkSize(size, chunkSize);
            const totalChunkCount = Math.floor(size / actualChunkSize) || 1;
            const mime = contentType ?? mimeFromPath(filePath);
            const init = await this.api<{ publish_id: string; upload_url: string }>("/v2/post/publish/inbox/video/init/", {
                source_info: { source: "FILE_UPLOAD", video_size: size, chunk_size: actualChunkSize, total_chunk_count: totalChunkCount },
            });
            if (!init.upload_url) throw new TikTokApiError("TikTok did not return an upload URL.");
            await this.uploadFile(file, init.upload_url, size, actualChunkSize, totalChunkCount, mime);
            return { publishId: init.publish_id, uploadUrl: init.upload_url };
        } finally {
            file.close();
        }
    }

    async getPostStatus(publishId: string): Promise<PostStatus> {
        return await this.api<PostStatus>("/v2/post/publish/status/fetch/", { publish_id: publishId });
    }

    /** Poll until TikTok reaches a terminal state, or the caller's timeout expires. */
    async waitForPost(
        publishId: string,
        options: { intervalMs?: number; timeoutMs?: number; signal?: AbortSignal } = {},
    ): Promise<PostStatus> {
        const intervalMs = options.intervalMs ?? 5_000;
        const timeoutMs = options.timeoutMs ?? 15 * 60_000;
        const deadline = Date.now() + timeoutMs;
        while (true) {
            options.signal?.throwIfAborted();
            const status = await this.getPostStatus(publishId);
            if (status.status === "PUBLISH_COMPLETE" || status.status === "FAILED" || status.status === "SEND_TO_USER_INBOX") return status;
            const remaining = deadline - Date.now();
            if (remaining <= 0) throw new TikTokApiError(`Timed out waiting for ${publishId}; last status: ${status.status}`, "poll_timeout");
            await delay(Math.min(intervalMs, remaining), options.signal);
        }
    }

    private async uploadFile(
        file: Deno.FsFile,
        uploadUrl: string,
        fileSize: number,
        chunkSize: number,
        chunkCount: number,
        contentType: string,
    ): Promise<void> {
        let offset = 0;
        for (let index = 0; index < chunkCount; index++) {
            const isLast = index === chunkCount - 1;
            const length = isLast ? fileSize - offset : chunkSize;
            const bytes = new Uint8Array(length);
            let read = 0;
            while (read < length) {
                const count = await file.read(bytes.subarray(read));
                if (count === null) throw new Error("Unexpected end of video file.");
                read += count;
            }
            const end = offset + length - 1;
            const response = await this.fetchImpl(uploadUrl, {
                method: "PUT",
                headers: {
                    "Content-Type": contentType,
                    "Content-Length": String(length),
                    "Content-Range": `bytes ${offset}-${end}/${fileSize}`,
                },
                body: bytes,
            });
            const expected = isLast ? 201 : 206;
            if (response.status !== expected) {
                const message = await response.text().catch(() => "");
                throw new TikTokApiError(`Video upload chunk ${index + 1}/${chunkCount} failed: HTTP ${response.status} ${message}`, "upload_failed", undefined, response.status);
            }
            offset += length;
        }
    }

    private async api<T>(path: string, body: unknown): Promise<T> {
        const accessToken = await this.tokenManager.accessToken();
        const response = await this.fetchImpl(`${API}${path}`, {
            method: "POST",
            headers: { Authorization: `Bearer ${accessToken}`, "Content-Type": "application/json; charset=UTF-8" },
            body: JSON.stringify(body),
        });
        const json = await response.json() as ApiEnvelope<T>;
        if (!response.ok || (json.error && json.error.code !== "ok")) {
            throw new TikTokApiError(
                json.error?.message || `TikTok API request failed (${response.status})`,
                json.error?.code,
                json.error?.log_id,
                response.status,
            );
        }
        if (json.data === undefined) throw new TikTokApiError("TikTok API response did not include data.");
        return json.data;
    }
}

function chooseChunkSize(size: number, requested?: number): number {
    if (requested !== undefined && (!Number.isInteger(requested) || requested < MIN_CHUNK || requested > MAX_CHUNK)) {
        throw new RangeError(`chunkSize must be an integer from ${MIN_CHUNK} to ${MAX_CHUNK} bytes.`);
    }
    if (size <= MAX_CHUNK) return size;
    const chunkSize = requested ?? MAX_CHUNK;
    // TikTok defines chunk count as floor(video_size / chunk_size); the remainder
    // is merged into the final chunk, which may be up to 128 MB.
    const count = Math.floor(size / chunkSize);
    const finalSize = size - chunkSize * (count - 1);
    if (count < 1 || finalSize < MIN_CHUNK || finalSize > 128_000_000) {
        throw new RangeError("This file size and chunkSize do not meet TikTok's chunk rules.");
    }
    return chunkSize;
}

function mimeFromPath(path: string): "video/mp4" | "video/quicktime" | "video/webm" {
    const extension = path.toLowerCase().split(".").pop();
    if (extension === "mov") return "video/quicktime";
    if (extension === "webm") return "video/webm";
    return "video/mp4";
}

function delay(ms: number, signal?: AbortSignal): Promise<void> {
    return new Promise((resolve, reject) => {
        if (signal?.aborted) return reject(signal.reason);
        const timer = setTimeout(resolve, ms);
        signal?.addEventListener("abort", () => {
            clearTimeout(timer);
            reject(signal.reason);
        }, { once: true });
    });
}