---
id: arm-readers-left-at-the-sector-arm-after-the-reach-reading
kind: issue
title: Arm readers the reach reading left at the sector's arm: crescent, the ray-event grouping, the shared-cut and beside orders, vtxfac's strut anchors, and the reverse case
status: open
opened: 2026-10-29
priority: P3
cost: M
---

Found by the lane on
`sector-decisions-read-at-the-arm-disagree-with-codes-read-at-the-reach`
(PR 6053, branch `splitdoor/arm-vs-reach`).

## What

PR 6053 moved the Boolean's sector passes to one reading: **a
direction's side of another is read at the shorter of the two's own
reaches**, a direction with no reach of its own (a face–face line, a
bisector) at the other's (`boolean/sectors.rs` `reading_lever`, the
one home every moved site calls). It moved `within` / `within_signs`,
`half_planes`, `runs_into`, `sector_overlap`, `insert.rs`
`strut_order`'s side of the arrival edge and `fate.rs` `corner_holds`,
and routed `coincident_run` and `fate.rs` `tip_wraps`, already the one
reading, through it; each moved site on a witness
(`crates/sweep/tests/a_long_bound_beside_a_short_arm.rs`) or on a
unit row that tells the two readings apart. These readers of two
directions still lever at a sector's `arm`, the shorter chord of its
corner, with no witness that reaches a disagreement through them:

- `boolean/sectors.rs` `crescent` (`sectors.rs:2205`): the shared
  direction, `parallel_same(da, db, arm, …)`, at the pair's shorter
  arm. Left because `pair_search` now reads a coplanar pair touching
  only where its bounds are parallel or part outward at their own
  reaches, so the arm reading here can only widen what reads
  parallel; whether a first-order gap that a second-order crescent
  closes farther out is reached at all is unmeasured.
- `boolean/recl.rs` the ray-event grouping, both sites
  (`recl.rs:788`, `parallel_same(…, arm, RAY_COLLINEAR, …)` at
  `mentions[i].arm.min(mentions[j].arm)`; `recl.rs:798`, the same at
  `m.arm.min(n.arm)`): two mentioned directions, each an edge with a
  reach of its own, read at their sectors' arms. Left: no witness.
- `boolean/insert.rs` `beside_order` (`insert.rs:1641`,
  `bool_beside_flush`, `n·x` at `s.arm`) and `walks_after`
  (`insert.rs:1689`, `bool_shared_cut_order`, `(x × y)·n` at `s.arm`):
  a flanker's departure off the sector's plane and two germs' order,
  each flanker an edge with its own reach. Left: no witness; both
  refuse in band rather than read a side.
- `boolean/vtxfac.rs` `germ_order` (`vtxfac.rs:2791`,
  `strut_order((from, arm), normal, (g, h), arm, tie, band)`) and
  `tie_breaks_after` (`vtxfac.rs:2836`,
  `strut_order((at, arm), …, arm, …)`): they pass their sector's `arm`
  as the anchor's reach, so a germ's half-turn side of the anchor is
  read at the arm, not the anchor edge's reach as
  `insert::walk_faces_first` now reads it. Left: no witness, and the
  anchors there are not always an edge of the sector.
- **The reverse case**: a bound reads `On` the other face's plane at
  its reach while `within` reads the planes' line definitely outside
  the sector. For a bound `b` in face `a`, `within`'s margin is `b`'s
  code over `|n_a × n_b|`, so it is reachable only where the faces
  cross at an angle under `1/K` while `bool_faces_parallel` still
  reads them apart at the arm. Tilting the witness's boss about the
  hole's long edge (`φ = 0.005`, `0.05`) refused first at other
  readings at every pose tried (`bool_cone_arc`, `bool_cone_within`,
  `bool_vertex_face_side`, `bool_contact_edge`).

  **Candidate witness** (PR 6053's review, NOTE 1;
  `crates/sweep/tests/review_arm_vs_reach_probes.rs`
  `review_curved`, its known-refusal filter): the disc slab `r = 0.05`
  centred at `(0, −0.05)`, its vertex at the origin, beside the boss's
  quad slab whose long edge dips `1e-4` into it. `B ∪ A`, `B ∩ A` and
  `B ∖ A` refuse
  `ClassificationInvariant("surviving record with an On code after srecledges")`,
  raised at `recl.rs:1110-1118` (`srecledges`' postcondition),
  identically on main. Unmeasured whether the On code is this case's.

Not the shape, read and left: `bool_faces_parallel` and `germ_dir`'s
`bool_germ_line` (normals, no per-bound reach); `strut_order`'s two
germs' order (germs carry no reach); `sector_shape.rs` `SECTOR_REFLEX`
(a corner's own two bounds at the shorter of their chords, which is
the one reading); `census.rs` `strictly_inside` (levered at the edge's
length); `carrier_eq.rs` and `plane_eq.rs` (normals or axes).

## To do

For each site, build a witness before moving it: a short edge at the
corner beside the two directions read, each long, `θ` apart with
`θ·arm` Zero or in band and `θ` times the shorter of their reaches
definite. Then route the lever through `sectors::reading_lever`. For
the reverse case, first measure which record carries the surviving On
code in the candidate pose.

**2026-11-01 (ONEREAD, PR 6247):** `bool_cone_arc` and `bool_cone_within` are retired with `sectors::arc_side` and `sectors::in_sector`. The vertex-vertex rows now read through one gathered `link::Cone` (`boolean::vtxfac::cone_classes`); a witness pose that refused at those readings reaches the cone's `link_*` readings instead.
