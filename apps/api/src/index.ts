import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { logger } from "hono/logger";

const app = new Hono();

app.use("*", logger());

// ─── Health check ──────────────────────────────────────────
app.get("/health", (c) => c.json({ status: "ok", service: "kasuwa-api" }));

// ─── Phase 5 routes (stubs) ───────────────────────────────
// POST /tasks      — submit a new task with budget
// GET  /tasks/:id  — get task status + step progress
// GET  /workers    — list registered workers with reputation
// GET  /ledger     — paginated on-chain ledger entries

app.get("/", (c) =>
  c.json({
    name: "Kasuwa API",
    version: "0.1.0",
    docs: "Phase 5: routes for tasks, workers, ledger",
  }),
);

const port = Number(process.env["API_PORT"] ?? 4000);

serve({ fetch: app.fetch, port }, (info) => {
  console.log(`Kasuwa API listening on http://localhost:${info.port}`);
});
