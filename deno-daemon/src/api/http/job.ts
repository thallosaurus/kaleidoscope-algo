import { Hono } from "hono";
import { Env } from "../http.ts";
import z from "zod";

// MARK: - Job Control
export const JobControlRouter = new Hono<Env>();

//JobControlRouter.post("/")