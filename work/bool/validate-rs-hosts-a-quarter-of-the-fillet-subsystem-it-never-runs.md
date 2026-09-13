---
id: validate-rs-hosts-a-quarter-of-the-fillet-subsystem-it-never-runs
kind: issue
title: profile/src/validate.rs carries six FILLET_*_RECOURSE consts whose own docs say no caller reads them, plus a render arm, for a subsystem the module's inventory says never fires in validation
status: open
opened: 2026-09-12
refs: [2409]
---


## Finding

From the full review of WIRE's PR 2409 (S3), found under Q8 — the
reviewer read `crates/profile/src/validate.rs` end to end, 1910 lines,
which nothing in this project's process otherwise does. Filed here by
the WIRE orchestrator because `crates/profile/*` is S-BOOL's glob.
Confidence `sure` that the text is there, `likely` about the
disposition.

Roughly `validate.rs:280-500` is six `FILLET_*_RECOURSE` consts, each
carrying a 10–25-line doc comment, several of which admit in as many
words:

> **No caller reads this sentence.**

and

> Unreachable as rendered prose.

plus the `EscalationSite::Fillet` render arm at `:700-760`. Meanwhile
the module's **own inventory table** at `:106-112` says every `fillet_*`
row *"fires in the arc-carrier fillet construction … never in
validation."*

So a quarter of a file whose job is validation is prose and machinery
for a subsystem that, by the file's own statement, validation never
runs.

## Why it is filed as accumulation rather than as dead code

Nothing here is wrong, and no single unit added an unreasonable amount —
which is exactly the shape Q8 exists to surface. The file opens with a
gate and spends its middle on a constructor that lives elsewhere. The
reviewer's read: the text's home is the fillet constructor's module, not
this one.

A taker should check before moving anything whether the consts are
*referenced* from the fillet construction by path (in which case this is
a placement question) or genuinely unread (in which case the admission
in their own doc comments is the finding, and deletion is on the table).

## Beside it, same file, same shape

S4 from the same review: `validate_with` classifies every segment
**twice** — `build_loop_segs` (`:1444`) runs `build_seg` over the input
chain and `canonicalize_loop` (`:1789`) runs it again over the canonical
chain, guarded only by prose (*"this cannot fail for a chain whose input
just passed — mapped defensively all the same"*). Worth reading beside
this row: "compute it twice and trust determinism" is already the house
idiom in this file, which is context a taker of either row wants.
