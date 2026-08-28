# CLAUDE.md

Greenfield B-rep CAD kernel in Rust. API-first (GUI last), functional
style, fail-loud.

## Read before working

- `docs/DESIGN.md` — the **ratified design contract** (decisions D1–D9 +
  open questions). Do not re-litigate settled decisions; propose changes
  as revisions to the doc, discussed with Evan first.
- The live programs' plans and logs — each program is a
  `docs/<NAME>-PLAN.md` / `docs/<NAME>-LOG.md` pair, and the tail of a
  log is that program's live status. A program is closed when its
  `docs/<NAME>-EXIT-WALK.md` is ratified; that walk is then its
  done-state of record.
- `memories/MEMORY.md` — memory index; read it, follow pointers as
  relevant.

## Memory convention (important)

This repo is worked on through mngr's ephemeral worktrees, so the
built-in per-project memory directory does NOT persist between sessions.
**Use `memories/` in this repo instead**: read `memories/MEMORY.md` at
session start; save new memories there (same format — one file per fact
with name/description/type frontmatter, plus an index line in
`memories/MEMORY.md`); commit them like any other change. The in-repo
copies are canonical. Before adding one, read the memory-writing
criteria in `memories/cad-working-style.md` — the index is read at
the start of every session and its pointers followed as relevant, so
a new memory has to earn that.

## Working style

Design decisions get discussed in chat, refined through Evan's pushback,
then ratified into `docs/DESIGN.md` and committed — keep the doc synced.
Details: `memories/cad-working-style.md`, `memories/evan-profile.md`.

## Git workflow

- Private remote; push branches freely and often.
- **Merge-only, never rewrite history**: merge commits only (no squash,
  no rebase, no force-push). Frequent, messy commits are fine — commits
  are the record of actual work done.
- The sanitized/logical documentation of a change lives in the **PR
  description**, not in commit messages.
- Agents own this codebase and merge their own PRs to main. Exception:
  PRs that ratify open design questions (e.g. M0's Q1-residue PRs) are
  design conversations — wait for Evan's sign-off before merging.

## Repo notes

- `references/` (git-ignored) holds book scans (NURBS Book, Mäntylä
  complete, Hoffmann complete); they are scans — read pages visually (poppler
  is installed).
- License: dual MIT OR Apache-2.0. Project name: pending (Q9), placeholder
  acceptable.
