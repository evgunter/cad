# CLAUDE.md

Greenfield B-rep CAD kernel in Rust. API-first (GUI last), functional
style, fail-loud.

## Read before working

- `docs/DESIGN.md` — the **ratified design contract** (decisions D1–D9 +
  open questions). Do not re-litigate settled decisions; propose changes
  as revisions to the doc, discussed with Evan first.
- `docs/M0-PLAN.md` — the current work order (PR sequence for M0).
- `memories/MEMORY.md` — memory index; read it, follow pointers as
  relevant.

## Memory convention (important)

This repo is worked on through mngr's ephemeral worktrees, so the
built-in per-project memory directory does NOT persist between sessions.
**Use `memories/` in this repo instead**: read `memories/MEMORY.md` at
session start; save new memories there (same format — one file per fact
with name/description/type frontmatter, plus an index line in
`memories/MEMORY.md`); commit them like any other change. The in-repo
copies are canonical.

## Working style

Design decisions get discussed in chat, refined through Evan's pushback,
then ratified into `docs/DESIGN.md` and committed — keep the doc synced.
Details: `memories/cad-working-style.md`, `memories/evan-profile.md`.

## Repo notes

- `references/` (git-ignored) holds book scans (NURBS Book, Mäntylä ch.
  4–6, Hoffmann complete); they are scans — read pages visually (poppler
  is installed).
- License: dual MIT OR Apache-2.0. Project name: pending (Q9), placeholder
  acceptable.
