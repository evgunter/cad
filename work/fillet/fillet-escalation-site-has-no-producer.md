---
id: fillet-escalation-site-has-no-producer
kind: issue
title: The six profile fillet recourse sentences are dead: EscalationSite::Fillet has no producer and PathError::Escalated has no fillet arm
status: open
opened: 2026-09-04
refs: [recourse-sentences-owe-followability-pin, S11]
---

## The class

All six `FILLET_*_RECOURSE` sentences in `crates/profile/src/validate.rs`
(`FILLET_TURN_INBAND_RECOURSE`, `FILLET_NO_CORNER_RECOURSE`,
`FILLET_OFFSET_LEVER_RECOURSE`, `FILLET_ENCLOSING_RECOURSE`,
`FILLET_FIT_RECOURSE`, `FILLET_LEG_EXTENT_RECOURSE`) are rendered by
exactly one Display arm — `ProfileError::Escalated { site:
EscalationSite::Fillet, .. }`, dispatched on the escalation's predicate
name (`crates/profile/src/validate.rs:640`). Nothing in `crates/*/src`
constructs that value:

- every `ProfileError::Escalated` mint in `validate.rs` (`:1287`,
  `:1315`, `:1411`, `:1503`, `:1544`, `:1607`) carries a `Segment`,
  `SegmentPair` or `Loop` site;
- `EscalationSite::Fillet` appears in `src` only at the Display arm;
  the other two mentions are hand-built test values
  (`crates/profile/tests/rejections.rs:539`,
  `crates/profile/tests/fillet_recourse_followability.rs:202`).

The gates themselves fire — the nine `fillet_*` predicate names are
decided in `crates/profile/src/sugar.rs` — but an in-band verdict
leaves as `PathError::Escalated`, whose Display
(`crates/profile/src/path.rs:1505`) has no fillet arm and appends the
shared `COINCIDENCE_RECOURSE`. The tailored sentence reaches nobody.

## What is already known, and where

This is not new to PR 1753, which narrates it as its headline finding:
`crates/profile/tests/rejections.rs:501` (commit 38cb556f, 2026-09-02)
already says "`EscalationSite::Fillet` has no producer in the kernel
today", and `work/code-quality/S11.md` carries the same neighbourhood
("`ProfileError`'s five fillet variants … constructible only from
`test_support.rs`", later "now fully orphaned"). Neither recorded the
six sentences as a dead-recourse instance, and no item owned it. This
file is that home.

## What is pinned

`crates/profile/tests/fillet_recourse_followability.rs` (PR 1753)
asserts, per sentence, that the public door's refusal carries none of
the six — so those rows go red the day a producer lands — and that the
request each sentence endorses builds anyway. The rows are
characterizations, not proof the sentences are worth keeping.

## The decision owed

One of: give `PathError::Escalated` a fillet arm keyed on the predicate
name (the sentences then reach the caller the gates were written for);
route the `sugar` fillet escalations through `ProfileError` at the
`Fillet` site; or retire the six constants and the arm as machinery
with no producer. A door change either way — input to the FILLET
program's H units.
