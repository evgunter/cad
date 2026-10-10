---
id: the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op
kind: issue
title: vtxfac's admission probe drops the Tangent offer for an undeclared pair whose face has a null-scaffold edge mid-op
status: parked
blocked_on: [declared-pairs-retire]
opened: 2026-10-02
priority: P3
cost: E
---


## What

`vtxfac::classify_vertex_on_face`'s admission probe (the `tangent`
offer, beside the `bool_sector_coplanar` refusal) asks
`geom_brep::tangent_locus` whether the pair's tangency derives, so that
the refusal can offer a `Tangent` declaration. A DECLARED pair reads its
extent from `DeclaredPairs::reach_of`, measured at rest. An UNDECLARED
pair has no measured extent, so the probe reads
`rest::pair_extent` on the mid-operation bodies, and discards a failure
with `.ok()`: a face whose boundary carries a null-scaffold edge
mid-union has no readable box (`census::face_reach` answers `None`;
see `work/restread/census-face-reach-returns-a-nan-box-for-an-unclaimable-boundary-edge.md`),
so the probe reads "no tangency" and the refusal drops the `Tangent`
offer it would otherwise make.

Nothing answers wrong — the refusal still refuses, and only the offer
of a declaration is lost — but the offer then depends on whether the
face happens to carry scaffolding at that site.

## The fix's shape

Measure the extent of every candidate pair the probe can ask about on
the operands at rest (or the operand faces' at-rest extents, keyed by
face, and enclose the pair's from those), so the mid-operation probe
never reads a box. Filed from the fix pass of PR 3795 (MINOR-5's
item 6a), on the TANG slate since the probe is the lever's consumer.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: what is lost is vtxfac's offer of a Tangent declaration inside an undeclared refusal; both retire into unproven-coincidence findings at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Re-parked by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E leaves the probe as it was (`crates/topo/src/boolean/vtxfac.rs:599`–`:611`). A pair with no class still reads `pair_extent` mid-operation and drops the `Tangent` offer on a face whose box does not read. E narrows its reach. The glue door verifies every candidate tangency on the operands at rest and declares it (`crates/topo/src/boolean/glue.rs:72`–`:83`), so a pair that is tangent arrives here with a class. The `None` arm sees only pairs the door did not verify. What the probe feeds is the offer of a declaration (`admitted`, read by `DeclaredPairs::read`), and stage 4 F deletes it with `BooleanCoincidence` and the declare menu. Re-parked on `declared-pairs-retire`: close it there if the offer is gone, or fix the extent read if F keeps it.
