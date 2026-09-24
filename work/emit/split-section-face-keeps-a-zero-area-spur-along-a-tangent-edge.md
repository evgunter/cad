---
id: split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge
kind: issue
title: A split whose plane is tangent to the target along an edge AND crosses it elsewhere keeps the tangent edge as a zero-area spur of the section face
status: dispatched
opened: 2026-09-23
priority: P0
cost: H
branch: emit/split-spur
refs: [split-of-a-fused-declared-union-refuses-duplicate-vertex-name]
---

## What

The plane y + z = 2 (through (0,1,1), normal (0,1,1)/√2) touches the
declared union of `a` = x(0,1) and `b` = x(0.5,1.5) (both y,z ∈ (0,1))
along the union's top/far rim line y = z = 1. A slab `g` = x(1.2,1.3),
y(−1,2), z(0.5,3.0) is also in the union, and the plane really crosses
it.

In `[a,g,b]` and `[g,a,b]` the split SUCCEEDS. Its section face (Above
`19v9`, Below `22v13`) is `g`'s cross-section plus an antenna. The
antenna runs out along the tangent line from (1.2,1,1) to (0,1,1) and
back, and from (1.3,1,1) to (1.5,1,1) and back. So:
- the section loop visits (0,1,1), (0.5,1,1) and (1,1,1) twice;
- the Below body holds TWO vertices at each of (0.5,1,1) and (1,1,1),
  both null-pair copies of ONE original (`vertex_pairs` has two rows
  with that original);
- the Above body's section face reaches out to x = 0, where it has no
  material;
- `topo::validate` passes both halves.

The same plane refuses `DegenerateSection` on `a` alone, on `a ∪ b`
(both orders), and on `a ∪ g0` with `g0` = x(0.2,0.3). There the
tangent contact forms its own null section face, and the area test in
`crates/topo/src/splitting/join.rs` (`split_section_area`, the
`2|A|/P` margin) refuses it. When the tangent contact joins a real
section's loop instead, the loop's NET area is positive, and the
zero-area spur passes the test.

## Consequence downstream

`name_split`'s vertex pass (`crates/editor-core/src/names/emit_topo.rs`,
`name_split_edges_vertices`) names each null-pair copy by the original
it copies, so the two Below copies mint one name. That is
`NamingError::Duplicate` on
`CrossingVertex{Below, a.RimEdge(End,2)#OrderAlong{0 of 2}}` in
`[a,g,b]`, and on `OnToolVertex{Below, a.CapVertex(End,2)}` in
`[g,a,b]`. The refusal is truthful: it reports a degenerate body. The
emitter needs no rule for this. The fix is in the join.

## Fix direction (unmeasured)

The geometrically right result has no spur. The tangent contact
contributes nothing to the section, and y = z = 1 stays an ordinary
Below edge touching the plane.

Failing that, refuse the spur typed, as `DegenerateSection` does for a
null face, so that the verdict does not depend on whether the contact
happens to touch a real section.

## Found by

EMIT's `split-of-a-fused-declared-union-refuses-duplicate-vertex-name`,
measuring the Duplicate's cause (instrumented `name_split` vertex pass
plus per-half face dumps).

## Ownership

`crates/topo/src/splitting/join.rs` is reach's ground. REACH had no
orchestrator on 2026-09-23, and EMIT's lane holds the measurement. So
EMIT carries the fix and announces the crossing, as `work/README.md`
allows for shared ground.
