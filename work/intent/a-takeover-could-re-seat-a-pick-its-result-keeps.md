---
id: a-takeover-could-re-seat-a-pick-its-result-keeps
kind: issue
title: A takeover refuses a pick it strands where it could re-seat the pick exactly on a piece its result keeps
status: open
opened: 2026-10-26
priority: P3
cost: M
---

Filed by D′3 (PR 5636) on the INTENT orchestrator's 2026-10-26 ruling on AUTHTAIL's stale-mate questions (`work/intent/log.md`, 10-25 and 10-26 entries). A pick is never re-pointed (D10 `[placement.placement]`; `crates/editor-core/src/var.rs:279`). So a takeover that would re-point a copy a pick reads, onto a result holding no pieces, refuses at the viewer's takeover door: `Refusal::StrandedPick`, asked by `stranded_pick` (`crates/viewer/src/session.rs:3767`) from `sketch_on_face` (`:2775`), `add_boolean` (`:2941`), `add_mirror` (`:3003`) and `feature_over` (`:3302`). Row: `crates/viewer/tests/combine_ops.rs`'s `a_combine_that_would_strand_a_pick_refuses_typed`.

Out of scope there: a takeover whose result keeps a piece the pick could be re-seated on exactly. An example is a fillet of one `⊔` operand, where the `⊔` is rebuilt over the filleted member. In that case the takeover could write the re-seat as an authored edit in the same action (DOORS' `Rebind` design (from AUTHTAIL), `work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`, `## Decided`) instead of refusing. Today no gesture's result is a `⊔`, so the refusal is exact for every takeover the viewer has.

**Decide when dispatched:** whether an authored pick re-seat is admitted beside "never re-pointed by resolution". It is a re-wording of `[placement.placement]` and binds future work, so it is Ev's.
