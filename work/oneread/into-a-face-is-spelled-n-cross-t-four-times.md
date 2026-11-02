---
id: into-a-face-is-spelled-n-cross-t-four-times
kind: issue
title: The direction into a face across one of its edges is spelled four times, one an analytic copy of the dihedral side door
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral]
---



Filed by the SPLICE lane that built `geom_brep::dihedral_side`, on its
review (PR 5764).

A face's interior lies to the left of each of its half-edges, so the
direction into it across an edge walked along `t` is `n × t` with `n`
its outward normal. That one fact is spelled four times:

- `crates/geom-brep/src/dihedral_side.rs` `dihedral_side` (~:215, the cross at ~:228):
  `n_dep × walk`, with the walk an `EdgeWalk` minted from the
  half-edge's own side (`EdgeHalf`), then `enters_material` against the other
  face over the folded arm.
- `crates/topo/src/splitting/rules.rs` `edge_wedge` (~:468, the cross at ~:494): `along × n_mate`
  (the mate walking `−along`) against `n_own` through `enters_material`
  over `folded_lever_arm`. This is an analytic copy of the door's
  reading. It reads at a vertex, along the edge's chord, after
  `classify_dihedral` has read the pair transverse, and it maps a
  `Tangent` reading to smooth. So folding it onto `dihedral_side` means
  passing the chord as the mate's walk, keeping the smooth gate in
  front, and reading `SideEscalation::Tangent` as `None`. That is a
  splitting-lane edit, and not one the door's lane makes in passing.
- `crates/topo/src/stands.rs` `inward` (~:325): `normal × t` for a face's rung-3
  interior candidates.
- `crates/topo/src/census.rs` `wedge_side` (~:7449): `m × (r − p)`, the in-plane
  left of a ray about a face's normal. This is the same shape on a ray
  rather than an edge.

Looked at and not in the class: `boolean/sectors.rs` `Reach::departure` (~:121)
(a rise off a plane, `d̂·n̂·arm`) and `boolean/fate.rs` `departure` (~:367) (a
half-edge's own leaving direction).

The fix is one spelling of "the direction into a face across its
half-edge" that each site reads, with the walk taken from the half-edge
rather than a bare vector, as `EdgeWalk` takes it. `edge_wedge` folds
onto `dihedral_side` if the vertex chord is an acceptable walk there.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). ONEREAD holds the boolean questions read by two homes; id and body unchanged. (CLEAVE orchestrator)
