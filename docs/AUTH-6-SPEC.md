# AUTH-6 — a committed profile's shape is editable

**Row**: `work/author/the-viewer-keeps-its-profile-lock-and-order-search-after-set-program`
(triaged P0, D on 2026-09-29). It was filed onto VIEW's slate with no
priority and re-homed to AUTHOR when VIEW left the tracker. **Read it
in full.** It lists every site, and its sweep section says what its
pattern could and could not match.

**Branch** `author/profile-reshape`. **Never merge; I merge.**

## Why P0

A person who commits a sketch and then wants a different shape (one
more step, a line becoming an arc, a split circle's `n`) meets controls
drawn **disabled**. Over them sits `SHAPE_LOCKED`, a sentence saying the
document has no edit that rewrites a committed profile's program. **That
sentence is now false.** `DocEdit::SetProgram` exists
(`crates/editor-core/src/edit.rs:168`, ruled by Ev on `[ev]` #2904). It
replaces a live profile's program whole, validated once, under a stated
provenance, and it rebinds or reports every name the reshaping touches.
So this is a door the GUI can't author, which is what this program is
for.

## What is verified

I checked these against the tree on 2026-09-29. Line numbers drift, so
trust the symbols:

- `forms.rs`: `ShapeEdits::Locked` (`:319`) and `SHAPE_LOCKED` (`:332`).
- `sketch.rs`: `program_edits` (`:549`) and `Restructure` (`:607`).
- `session.rs`: `accepted_order` (`:2798`) and `ORDER_SEARCH_CAP`.
- `session/refuse.rs`: `Refusal::ProfileRestructure` (`:388`),
  `ProfileEditOrder` (`:403`) and `ProfileEditOrderCapped` (`:414`).
- The row's remaining sites (`op.rs`, `drafts.rs`, `pane/profile.rs`,
  and three test files) are as it lists them.

## The unit

Commit a profile edit as ONE `DocEdit::SetProgram { node, loops,
provenance }` through `commit`. Then retire the lock, the per-slot diff,
the write-order search and its cap, and the three refusals about states
nobody writes any more. That removes the tests that pinned them too,
turning `a_reshaped_program_refuses_restructure` into "a reshaped
program lands as one edit and its names follow".

**Three design calls. Decide and say:**

1. **The provenance.** The row says the editor already knows it: a
   moved number is `LoopProvenance::identity(&loops)`, and an inserted
   or removed leg is something the editor did itself. Check that it's
   true for every way the editor can reshape a draft (insert, delete,
   reorder, change a verb, change an arc mode, change `n`). **If some
   edit can't state its provenance honestly, say so rather than guess.**
   A wrong provenance rebinds a name onto the wrong step, which is a
   confident wrong answer.
2. **Surfacing what the door reports.** `Applied.maintenance` carries
   `Rebound { from, to }` and `Strand` / `StrandedAppearance`. A strand
   is the fillet the author is about to lose. The row suggests reusing
   the delete cascade's strand-count affordance. Decide whether the
   author sees it before committing, after, or both, and say why.
3. **Drafts.** `drafts.rs` measures a held draft against the base
   program through `program_edits`. With one whole-program edit, decide
   what a draft's validity check becomes.

**Check before deleting:** confirm the three `Refusal` variants and
`ShapeEdits::Locked` have no reader outside the row's sites. The row's
sweep pattern couldn't match a control disabled by a word other than
`Locked`, so also try to check that blind spot.

## Seams, and a sibling lane

- `forms.rs`'s `ShapeEdits` is **FORMS' vocabulary** (FORMS owns the
  vocabulary the forms answer in; a new door is AUTHOR's).
  `pane/profile.rs` is CHROME's, FORMS', VNEWS's and VSEAM's. Append
  one seam note each to `work/forms/log.md` and `work/chrome/log.md`
  naming what you removed.
- **AUTH-5** (`author/path-preview-prefix`) runs in parallel inside
  `sketch::preview` and its drawers. You overlap in `sketch.rs`,
  `drafts.rs` and `pane/profile.rs`, in different functions. Stay in
  yours, merge `origin/main` before pushing, and report a conflict with
  AUTH-5 instead of resolving it by editing that unit's code.
- **The one-undo grouping is lost after save and reopen**
  (`work/vseam/an-action-of-several-edits-becomes-several-undos-after-reopen`).
  One `SetProgram` is one edit, so this unit shouldn't hit it. Say
  whether that holds.

## Verification and deliverable

- A reshaped committed profile lands as one edit, one undo, driven
  through the real panel (`crate::pane::headless`), with names that
  follow. A stranded name is reported to the author.
- Every stated behaviour gets a row that goes red if it stops being
  true, with the mutation named, restored from a byte copy.
- Local: fmt, clippy (both feature sets), `doc-gate.sh`, **every**
  `scripts/gates/*.sh`, the viewer suites, `work.py lint`,
  `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-6-target` on every invocation, including
  the excluded roots. Scratch in `/root/auth-6-scratch/`.
- CI: poll in the FOREGROUND, confirm the head SHA first, count by
  SUBSTRING, and read the per-RUN conclusion.

PR titled `AUTH-6: a committed profile's shape is editable`, carrying
the three design calls, territory output, mutations and §6 rows.
Report to me; do not merge.
