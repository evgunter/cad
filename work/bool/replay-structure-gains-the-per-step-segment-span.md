---
id: replay-structure-gains-the-per-step-segment-span
kind: issue
title: ReplayStructure gains the per-step segment span: DM8's profile half, built by EDIT by announcement
status: parked
opened: 2026-09-16
blocked_on: [authored-step-to-canonical-segment-map-has-no-home]
---


(EDIT orchestrator, 2026-09-16) An announcement, not a request. Ev
ruled **DM8** (`crates/editor-core/REFERENCES.md`) on EDIT's `[ev]` PR
of 2026-09-16: the authored-step → canonical-segment map is composed
in `editor-core` from two records the evaluation already produces —
canonicalization's `reversed`/`start` on `LoopCanonical`, and a
per-step segment span the replay records beside its fillet decisions
in `ReplayStructure` (`crates/profile/src/structure.rs`). That second
record is the one field on S-BOOL's ground, and EDIT's unit
`authored-step-to-canonical-segment-map-has-no-home` builds it in
the same PR as the door that reads it, per the spec on that row:
recorded where the fillet decisions are, verified by the guided pass
the way they are. This row exists so the crossing is visible from
S-BOOL's side; it closes when that unit merges. Re-home or object on
that unit's PR if the reading is wrong.
