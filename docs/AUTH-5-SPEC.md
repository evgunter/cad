# AUTH-5 — the path preview draws what replays when a step refuses

**Row**: `work/author/path-preview-draws-nothing-for-a-refused-step`
(P0, D). **Ev's own report** (2026-09-18): *"arc_fillet_arc and other
such paths should try to display something even when invalid, so the
user can figure out how to fix them."*

**Branch** `author/path-preview-prefix`. **Never merge; I merge.**

## What is verified, and what is yours

I checked these against the tree on 2026-09-29:

- `sketch::preview` is `crates/viewer/src/sketch.rs:1063`. The one
  refusal it survives today is the unfinished chain,
  `ReplayErrorKind::Transition { state, verb: None }` (`:1311`), which
  it draws under a provisional `line_to Start`. Every other refusal
  returns `Err` for the whole profile, and the viewport draws nothing.
- Its drawing callers are `app.rs:1839` (the viewport's held-draft
  preview), `pane/profile.rs:748`, and `drafts.rs` (`:1155`, `:1411`,
  `:1424`). `pane/viewport.rs:56` documents the provisional close.

**The unit** is the row's first step. When replay refuses at step `k`,
draw the longest prefix that does replay, `steps[..k]`, under the same
provisional close the unfinished-chain arm already uses, and mark it
open. Keep the refusal sentence beside it, and mark the drawn tip as
the place step `k` failed. **Reuse the machinery:** the prefix goes
through the same `replay`, and nothing about the lattice is
re-implemented.

**Two design calls, both yours. Decide and say:**

1. **What `preview` returns.** It returns `Result` today. A refused
   step now yields geometry AND a refusal together. Pick the shape
   (e.g. a preview value carrying an optional refusal and the index it
   stopped at, or a new variant), and check all four callers keep
   saying what they say now for the cases that already work.
2. **How the failed tip is marked** in the viewport, and how it reads
   as distinct from the unfinished-chain close. An author must be able
   to tell "you have not finished" from "step 4 does not work", and
   **if my framing that these differ is wrong, say so.**

## What is NOT this unit

For a fused step (`fillet_arc`, `arc_fillet`, `arc_fillet_arc`), the
carriers it bound before refusing are what would show *why* a fillet
doesn't fit. Drawing those needs the replay driver to hand them back.
That is a design question on the ratified PATHS lattice, and it waits
for Ev in `work/round/refused-fused-step-reports-no-partial-geometry`.
**The row cites it at `work/paths/`; it moved to `work/round/`.** Fix
that citation. For a fused step this unit draws the prefix *before* it
and stops. **Change nothing about what `replay` or `ReplayError`
return.** If you find the prefix can't be drawn without that, stop and
say so.

## Running beside you

**AUTH-6** (`author/profile-reshape`) runs in parallel. It deletes
`sketch::program_edits` / `Restructure` and the `ShapeEdits::Locked`
sites in `pane/profile.rs` and `drafts.rs`. You are in `sketch::preview`
and its drawers, so the files overlap and the functions don't. Keep
your edits to those functions, merge `origin/main` before you push,
and if you conflict with AUTH-6, report it; don't resolve it by
reshaping the other unit's code.

## Scope, verification, deliverable

In: `sketch.rs` (preview), `pane/viewport.rs`, `app.rs`,
`pane/profile.rs`, `drafts.rs` (preview callers), tests. Double claims
with CHROME/FORMS/VNEWS/VSEAM/VGEOM; run `work.py territory` on your
branch and put the output in the PR.

- Drive the preview through the real drawing path
  (`crate::pane::headless` exists) and assert what is PAINTED for a
  refused step, not only what `preview` returns.
- Every stated behaviour gets a row that goes red if it stops being
  true, with the mutation named. Restore mutations from a byte copy,
  never `git checkout --`.
- Local: fmt, clippy `--workspace --all-targets -D warnings` and again
  `--features viewer/app`, `doc-gate.sh`, **every**
  `scripts/gates/*.sh`, the viewer suites, `work.py lint`.
- `CARGO_TARGET_DIR=/root/auth-5-target` on every invocation, including
  the excluded cargo roots. Scratch in `/root/auth-5-scratch/`.
- CI: poll in the FOREGROUND and confirm the head SHA first. Count by
  SUBSTRING (`interval / test (…)`), and read the per-RUN conclusion.

PR titled `AUTH-5: the path preview draws what replays when a step
refuses`: both design calls, territory output, mutations, §6 rows.
Report to me; do not merge.
