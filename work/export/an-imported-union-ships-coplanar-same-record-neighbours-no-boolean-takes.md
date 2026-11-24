---
id: an-imported-union-ships-coplanar-same-record-neighbours-no-boolean-takes
kind: issue
title: an imported union ships coplanar same-facing neighbours on one plane record: it passes the at-rest gate and no boolean takes it as an operand (NonMaximalFaces)
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
refs: [half-revolve-caps-are-never-an-operand, step-import-discards-the-entity-ids-that-are-its-identity-channel]
---


Found while weighing the `[ev]` fork on PR 7331 (records follow the file). The finding is independent of how records are chosen: it holds today, with the bit dedup.

## Measured (probe, default ε, main at 3a9a9b82e1)

- `crates/step-import/tests/fixtures/freecad/fuse_boxes.step` (a FreeCAD union of two boxes) imports as 14 faces; the native union has 10.
- Top: three faces on one plane record, all sense true, outward +z. Bottom: three faces on one record, all sense false, outward −z. There are 8 same-key planar edge adjacencies. `one_record` joins the records, and nothing merges the faces after.
- Tier 3 (`validate_geometric`) returns `Ok`.
- As a boolean operand, union, subtract and reversed union against a far brick all return `NonMaximalFaces { edge }` (`gate_maximal_faces`, `crates/topo/src/boolean/reduce.rs`).
- The 14-face count is pinned in `tests/freecad.rs` and `tier_gate.rs`.

## Not import-only

A native `Revolution::Partial(π)` of an axis-touching profile ships the same state: two same-facing caps on one plane record across the axis edge, tier 3 `Ok`, refused as an operand. That is `work/lathe/half-revolve-caps-are-never-an-operand.md` (P2, Ev's band), re-measured 2026-11-22.

## The design question (why `design: true`)

DESIGN says a body is "the unique complex with maximal faces and maximal edges … and the form is checked at rest on the result alone". Tier 3 checks maximal edges (check 11) but not maximal faces; F7 checks them only at the boolean's input. So two doors ship a form the boolean then refuses. The options the fork's designers named:
- a tier-3 maximal-faces check beside check 11, plus a reported merge at every door that leaves such a pair: import's end, after `one_record`, and the half revolve's caps;
- or the gate admits the pair and the boolean merges on entry.

This changes `[topology]`'s at-rest list, so it is Ev's. The designers agreed it is its own question, not part of PR 7331's. Its band follows the half-revolve row's (P2, Ev's ruling).
