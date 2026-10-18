/** Minimal client for publishing a Reel through Meta's Instagram Graph API.
 *
 * Nothing is sent until one of the returned async methods is explicitly called.
 * `videoUrl` must be publicly reachable by Meta's servers (a local file path is
 * not enough); the account must be an eligible professional account.
 */

import { BlenderRenderProject } from "../renderer/executor.ts";
import { getBaoTokens } from "./bao.ts";
import { TInstagramRequest } from "./http.ts";

export type InstagramConfig = {
  /** Graph API version, for example `vXX.0`. Keep this explicit so it can be
   * upgraded deliberately when Meta retires an API version. */
  apiVersion: string;
  accessToken: string;
  instagramUserId: string;
};

export type ReelOptions = {
  videoUrl: URL;
  caption?: string;
  shareToFeed?: boolean;
};

export type MediaContainer = { id: string };
export type PublishedMedia = { id: string };
export type ContainerStatus = {
  id: string;
  status_code: "IN_PROGRESS" | "FINISHED" | "ERROR" | "EXPIRED" | string;
  status?: string;
};

export type WaitOptions = {
  /** Delay between status checks. Defaults to 5 seconds. */
  pollIntervalMs?: number;
  /** Stop waiting after this duration. Defaults to 10 minutes. */
  timeoutMs?: number;
};

type GraphError = {
  error?: { message?: string; type?: string; code?: number; error_subcode?: number };
};

/** Create an API client. Constructing it does not make a network request. */
export function createInstagramClient(config: InstagramConfig) {
  if (!/^v\d+\.\d+$/.test(config.apiVersion)) {
    throw new Error(`Invalid Graph API version: ${config.apiVersion}`);
  }

  const baseUrl = `https://graph.facebook.com/${config.apiVersion}`;

  async function request<T>(path: string, method: "GET" | "POST", params: Record<string, string>): Promise<T> {
    const url = new URL(`${baseUrl}/${path}`);
    const headers = new Headers();
    let body: URLSearchParams | undefined;

    // Meta accepts access_token as a request parameter. Use form encoding for
    // POSTs so parameters are unambiguous and never accidentally send JSON.
    if (method === "GET") {
      for (const [key, value] of Object.entries(params)) url.searchParams.set(key, value);
      url.searchParams.set("access_token", config.accessToken);
    } else {
      body = new URLSearchParams({ ...params, access_token: config.accessToken });
      headers.set("Content-Type", "application/x-www-form-urlencoded");
    }

    const response = await fetch(url, { method, headers, body });
    let data: unknown;
    try {
      data = await response.json();
    } catch {
      throw new Error(`Instagram API returned non-JSON (${response.status} ${response.statusText})`);
    }

    /*const graphError = data as GraphError;
    if (!response.ok || graphError.error) {
      const detail = graphError.error?.message ?? `${response.status} ${response.statusText}`;
      const code = graphError.error?.code;
      throw new Error(`Instagram API error${code === undefined ? "" : ` ${code}`}: ${detail}`);
    }*/

    if (!response.ok || (data as GraphError).error) {
      console.error(data);
      console.error("Meta API response:", JSON.stringify(data, null, 2));

      const graphError = data as GraphError;
      throw new Error(
        `Instagram API error: ${JSON.stringify(graphError.error ?? data)}`
      );
    }
    return data as T;
  }

  return {
    /** Ask Instagram to fetch the public video URL and create a Reel container. */
    createReelContainer(options: ReelOptions): Promise<MediaContainer> {
      if (options.videoUrl.protocol !== "https:") {
        throw new Error("Instagram requires a publicly reachable HTTPS video URL.");
      }
      return request(`${config.instagramUserId}/media`, "POST", {
        media_type: "REELS",
        video_url: options.videoUrl.href,
        ...(options.caption === undefined ? {} : { caption: options.caption }),
        ...(options.shareToFeed === undefined ? {} : { share_to_feed: String(options.shareToFeed) }),
      });
    },

    /** Read processing state for a container. */
    getContainerStatus(containerId: string): Promise<ContainerStatus> {
      return request(containerId, "GET", { fields: "status_code,status" });
    },

    /** Wait until Instagram has finished processing or reports a terminal error. */
    async waitForContainer(containerId: string, options: WaitOptions = {}): Promise<ContainerStatus> {
      const pollIntervalMs = options.pollIntervalMs ?? 5_000;
      const timeoutMs = options.timeoutMs ?? 10 * 60_000;
      if (pollIntervalMs <= 0 || timeoutMs <= 0) throw new Error("Polling intervals must be positive.");

      const deadline = Date.now() + timeoutMs;
      while (Date.now() < deadline) {
        const status = await this.getContainerStatus(containerId);
        if (status.status_code === "FINISHED") return status;
        if (status.status_code === "ERROR" || status.status_code === "EXPIRED") {
          throw new Error(`Instagram could not process the Reel: ${status.status ?? status.status_code}`);
        }
        await new Promise((resolve) => setTimeout(resolve, Math.min(pollIntervalMs, deadline - Date.now())));
      }
      throw new Error(`Timed out waiting for Instagram to process container ${containerId}.`);
    },

    /** Publish a finished Reel container. */
    publishContainer(container: MediaContainer): Promise<PublishedMedia> {
      return request(`${config.instagramUserId}/media_publish`, "POST", {
        creation_id: container.id,
      });
    },

    /** Full workflow. Calling this is the point at which API requests are made. */
    async publishReel(options: ReelOptions, waitOptions: WaitOptions = {}): Promise<PublishedMedia> {
      const container = await this.createReelContainer(options);
      await this.waitForContainer(container.id, waitOptions);
      const c = await this.publishContainer(container);
      const status = await this.getContainerStatus(container.id);
      console.log(status);
      return c
    },
  };
}

type InstagramPost = {
  caption: string
}

export async function postInstagram(project: TInstagramRequest, post: InstagramPost) {
  //const videoUrl = new URL(`https://${base}/${project.job.id}/video.hdr.mp4`);
  const h = await fetch(project.url, {
    method: "HEAD"
  })

  if (!h.ok) { throw new Error(`${project.url} is not public`) }
  console.log("posting", project.url);

  //const accessToken = Deno.env.get("ACCESS_TOKEN") ?? "";
  const accessToken = await getBaoTokens("instagramAccessToken", "/v1/kv/data/tarascope")
  //const instagramUserId = Deno.env.get("INSTAGRAM_USER_ID") ?? "";
  const instagramUserId = await getBaoTokens("instagramUserId", "/v1/kv/data/tarascope")
  
  const client = createInstagramClient({
    apiVersion: "v24.0",
    accessToken,
    instagramUserId
  })

  await client.publishReel({
    shareToFeed: true,
    videoUrl: new URL(project.url),
    ...post,
  })
}