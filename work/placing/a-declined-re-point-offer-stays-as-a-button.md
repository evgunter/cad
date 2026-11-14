---
id: a-declined-re-point-offer-stays-as-a-button
kind: issue
title: The re-point offer is derived from the document, so a declined offer stays as a button for as long as the body's placements read the body
status: dispatched
opened: 2026-11-08
priority: P2
cost: M
branch: placing/offer-any-depth
---


Found by the dual review of PR 6581 (R1 s7, R2 Q7), filed by `placing-copy-reads` on the orchestrator's ruling.

**What.** `world::repoint_offer` (`crates/viewer/src/world.rs`) derives the offer from the document: any drawn, unplaced body built from a body whose placements read it (and built on no copy) shows "Re-point the N placements of B onto …" in the Properties pane whenever it is selected (`frame::placement_offers`). Nothing records that the person declined, so a declined re-point stays offered for as long as the placements read the body: on a fillet of `B`, a mirror's union of `B`, a cut `B − T`, a union `B ∪ T`. Ev asked only that "filleting the body should offer to repoint that variable" (`[ev]` PR 6353).

**Why not fixed here.** Retiring the offer needs either state the recipe does not hold (a declined flag, which is history in the document or display state that a reopen loses) or a narrower rule for which bodies offer (only the gesture's own result, only right after the gesture). Each is a design choice about the offer, not a defect in the build; a "dismiss" control was not invented.

**Done when** Ev or a designer pair rules when the offer stands and how a person declines it, and the pane follows the ruling.

## Ruled (Ev, 2026-11-14, `[ev]` PR 6915: "this seems reasonable!")

The offer is a reading of the document, not a question. It stands on every drawn body that is built, at any depth, on a body a source places, and on no placed copy. It says so as a fact, with its button: "bracket (2 copies) copies B, which this body is built on", then "Re-point bracket onto G". It stands for as long as that is true. Nothing records a decline. The README paragraph states the rule (PR 6915).

**Owed (the build).**
- The depth walk in `world::repoint_offer`, likely shared with `built_on_copy`'s walk, so that "built on a copy" and "built on bodies" are one answer.
- The sentence per row in `frame::placement_offers`.
- `repoint_offer`'s doc comment: drop "accepted or not".
- A row: fillet `B` with the offer unpressed, then chamfer; the chamfer offers `bracket`. The probe on `placing/probe-offer-depth` (`007a2f5b89`, `probe_offer_depth::a_feature_on_an_unaccepted_fillet_offers_nothing`) asserts today's lapse and turns into that row.

**Done when** that row passes and the pane reads the sentence.
