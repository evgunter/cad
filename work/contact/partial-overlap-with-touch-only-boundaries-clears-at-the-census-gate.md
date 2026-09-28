---
id: partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate
kind: issue
title: Two half-overlapping cubes whose boundaries meet only in touches (coplanar faces, edges in faces) clear the census's instance arm at the box gate — the census has no arm for a partial overlap that produces no pierce
status: dispatched
opened: 2026-09-16
refs: [2767, 750]
priority: P0
cost: H
parent: CONTACT-5
---

Found by both of BOOL-4's reviews (PR 2767) as the class beside the
unit's defect, pre-existing at the base and at the head, and filed by
the S-BOOL orchestrator on CURVED's slate (`crates/topo/src/census.rs`
is CURVED's path). The instance-containment arm's box gate clears a
pair when both orderings separate definitely on some axis, on the
premise that a partial overlap always produces a boundary event the
exact sweeps push (an edge piercing a face, an edge crossing an
edge). Two axis-aligned cubes overlapping by half — `[0,2]³` and
`[1,3]×[0,2]×[0,2]` — produce only touches (coplanar faces, edges lying
in faces, vertices on faces), no pierce and no cross, and the pair
clears at the gate while the materials overlap over a unit cube. The
material test BOOL-4 landed cannot see it because the gate clears
before it runs; BOOL-4 pins the wrong clear as a row with a pointer to
this item and corrects the census header's "same-solid only" sentence.
The fix is an arm for the partial-overlap class — the overlap region's
boundary is made of faces of each solid lying inside the other, which
a material test over the touch findings' incident faces (or the
exclusion ring, when it lands) can decide. Measured, not acted on;
difficulty M.
