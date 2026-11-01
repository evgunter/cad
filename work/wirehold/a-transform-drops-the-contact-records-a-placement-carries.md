---
id: a-transform-drops-the-contact-records-a-placement-carries
kind: issue
title: Transform drops its input's contact records while PlaceInWorld carries them verbatim, though both are rigid motions that keep arena keys
status: closed
opened: 2026-10-08
closed: 2026-10-23
---

Found by the review of INTENT stage 2 C (PR #4359, NOTE-3).

`wire_place_in_world` (`crates/editor-core/src/eval/wire.rs`) carries
its input's contact records and declaration rows verbatim, on the
argument that a rigid placement keeps every arena key; the posed row
`intent_s2_c_world::a_posed_placement_moves_its_copy_and_its_records_rigidly`
pins it. `wire_transform` returns `OpOut::plain` and drops a boolean
input's records, though a rigid `Transform` keeps the same keys. One
of the two rules is wrong, or the stated reason is not the one that
decides.

`Transform` retires in INTENT stage 3 (`[ev]` #4326: nothing moves a
body), so the cheapest settlement may be to let it retire; until then
a `Transform` of a boolean silently loses the records a placement of
the same boolean would keep.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. Still true (`wire_transform` returns `OpOut::plain`). `Transform` retires into a placement, which already carries records; very likely moot. Parked on `transform-retires-into-a-placement`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Closed (2026-10-23)

Moot in D (`5bae0fff20`, "Datum, Transform and the placement chain
retire"). `wire_transform`, `wire_place_in_world` and `Node::Transform`
no longer exist anywhere under `crates/`. `Transform` (11) and
`PlaceInWorld` (37) are on `RETIRED_NODE_KIND_TAGS`
(`crates/editor-core/src/eval/mod.rs`). The one placement left,
`wire_place` (`crates/editor-core/src/eval/wire.rs:4086`), carries its
input's records, a boolean's `contacts` or `value.contacts`
(`wire.rs:4104-4114`). With one rule left, there is no second rule to
disagree with it. The Pattern half stays on
`transform-and-pattern-drop-a-values-contact-records`.
