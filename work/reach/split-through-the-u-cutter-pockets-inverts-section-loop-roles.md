---
id: split-through-the-u-cutter-pockets-inverts-section-loop-roles
kind: issue
title: a split of the U-cutter subtract at x = 3 evaluates, then its half fails the at-rest gate with LoopRoleInverted on two faces
status: open
opened: 2026-09-25
priority: P0
cost: D
---


Found by the review of the GATHER split-halves lane (PR 3256), and
re-measured by that lane at its fix pass. It is identical on the base the
PR branched from, so the gather is not involved.

**Repro.** Take a 4×4×4 block at the origin, less a U-cutter. The cutter
is the profile in the z = 1 plane, (2,1) (6,1) (6,3) (2,3) (2,2.5)
(5,2.5) (5,1.5) (2,1.5), extruded 2, so z ∈ [1, 3]. Its two prongs, y ∈ [1, 1.5] and
[2.5, 3], cross the x = 4 wall. The helper that builds it is `cutter`
in `crates/editor-core/tests/gather_placed_under_two_roots.rs`, called
with `&[(1.0, 1.5), (2.5, 3.0)]`. Now split that subtract by the plane
through (3, 0, 0) with normal +x and take `Part(Above)` as the only
root. What happens:

- the split node evaluates with no node error;
- `product` then refuses `ProductInvalid`, with
  `LoopRoleInverted { face: FaceKey(14v1), loop: LoopKey(13v1) }` and
  `LoopRoleInverted { face: FaceKey(15v1), loop: LoopKey(16v1) }`;
- a plane at x = 3.9 gives the same two findings.

The review saw it as `SolidInvalid` with the two halves as roots, which is
the per-source gate firing on the same body.

**What it says.** A plane split of an ordinary planar body produces a
half whose flat face has its outer loop and hole roles swapped: tier 3
check 6 (`crates/topo/src/validate.rs`, `LoopRoleInverted`). The split
verb does not gate its own output at rest, so the defect only surfaces
at the gather.

The plane x = 3 runs through the pockets the prongs cut, so the section
faces there carry holes. Those are the likely subject; this is not
confirmed. Planes at y = 1.25, 2, 2.2 and 3.5 and at z = 2 split the same
subtract cleanly; their halves pass the per-source gate in
`gather_placed_under_two_roots`.
