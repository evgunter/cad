---
id: a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door
kind: issue
title: a loft section hinged on one of its own edges collapses that wall, and refuses at the attach gate (or as NoParameterStep on strip 0) rather than naming the pinned wall
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body]
closed: 2026-11-02
pr: 6141
---


Found by `carve/one-door-for-coincident-sections`'s sweep (2026-10-06).

A loft section rotated about one of its own edges, so that edge is
the same world segment in it and its neighbour, passes the stacking
fold (`crates/sweep/src/loft.rs` `stacking_fold`: the centroid moves
along the base normal) and leaves the wall over that edge with two
identical section curves. What happens next depends on which strip
the edge is, not on the fact:

- **The edge is segment 0 of loop 0** (the strip
  `first_strip_parameters` measures v on) **and the hinge is
  bit-exact**: the parameters do not ascend and the skin refuses
  `SkinError::NoParameterStep` naming the section. True, but it says
  nothing about the wall, and the same section authored from another
  start vertex takes the next arm.
- **The edge is segment 0 of loop 0, pinned only up to rounding**
  (PR 4186's review, measured): under a generic placement (rotation
  about (0.3, 0.4, 0.866) by 0.7 plus a translation) the strip-0 hinge
  moves by about 7.9e-17. The parameters come out `[0, 7.68e-17, 1]`,
  pass the exact ascent check, and `loft_geometry` (and so
  `sweep_geometry`) return `Ok` with a largest control coordinate of
  about 3.46e15 for a 2-unit section: a silent wild answer at a public
  door. `loft_body` refuses at the attach gate. Pinned as measured by
  `one_door_for_coincident_sections.rs`
  `a_hinge_pinned_only_up_to_rounding_passes_the_exact_ascent_check`,
  which goes red when this is fixed.
- **Any other edge**: the parameters ascend, the skin builds, and the
  assembly refuses at the attach gate. Measured on three 2×2 squares
  at z = 0, hinged about the edge y = +1 by −0.3 rad, z = 2, v-degree
  2: `an Euler operation of the assembly refused: geometry attachment
  gate: the stored interval's span (not a sampled check) escalated:
  margin -8.468018830476105e-1 lies past the ambiguity band (1e-9,
  1e-8). Recourse: move the geometry so this edge is not vanishingly
  short` — a refusal about an edge's span, past the band, for a wall
  that collapsed to a segment.

The pinned-row decision itself is a bare compare: `skin.rs`
`skin_parameters`' `!(total > 0.0)` decides a control row is pinned
and abstains from the average, unbanded, so a row that moves by
rounding counts as moving.

The fixture is `one_door_for_coincident_sections.rs`'s hinged row,
with the hinge moved off strip 0. Whether a pinned wall should refuse
at all (a loft that touches itself along an edge) or build, and with
what refusal, is a design question; the v-parameterization item
(`refs`) changes which arm strip 0 takes.

## Evidence (2026-10-06, after `loft-v-parameterization-…` landed on its branch)

The loft's v is now Eq. 10.8 over every row of the OUTER loop, so a
section hinged on one of its own edges no longer pins the
parameterization: the loop's other rows step. Both hinges in
`one_door_for_coincident_sections` (the exact strip-0 hinge, and the one
pinned only up to rounding) now parameterize normally (`[0, 0.1487, 1]`
for the rounding one), `loft_geometry` answers walls the size of the
sections, and `loft_body` refuses at the attach gate. The exact hinge:
`Euler(Certification { Escalated { check: ParamSpan, predicate:
"nurbs_span_meter", margin −0.349 } })`. The hinge's pinned corners make
their seams double back. Before, the refusal depended on WHICH edge was
the hinge (strip 0: `NoParameterStep`; any other edge: this one). It is
now the same for every edge, and still at the wrong door.

## Weighed (2026-10-31): pinned corners are classified, and the sections are apart beyond them

Two designers (an Opus and a Fable lane) were given the problem and no
options. They split in round 1 and converged in round 2, each taking
the other's point. Nothing went to Ev. The record is on
`analysis/design-fork/carve-pinned-wall`.

- **The fact is a pinned corner**: a profile vertex at the same world
  point in sections k and k+1. A hinge is two adjacent pinned corners.
  It is decided once per corner per slab, banded, at the loft door
  before the skin, by one named predicate. It is the loft's analogue of
  revolve's on-axis vertex (`revolve/axis.rs`).
  - **Class 1**, pinned throughout the span its wall covers (every slab
    at v-degree ≥ 2, or a slab at v-degree 1, which covers the
    two-section wedge): **a legal body**. The two corners are one
    vertex, and no strut is minted. A segment pinned at both ends has
    no wall in that slab, and its neighbours share the edge, as
    revolve's `WallClass::OnAxis` does. A wall beside a pinned corner
    has a collapsed boundary column there, which is a chart
    singularity and so a vertex (DESIGN's seam clause). The designers
    believe it is **unbuilt today**: `pcurve_cache`'s iso lane refuses
    a degenerate iso. If the unit's measurement confirms that, class 1
    refuses typed as *unsupported* and names that blocker, never as
    invalid input.
  - **Class 2**, pinned in some slabs but moving in others under a wall
    that spans both (v-degree ≥ 2): refuses typed, naming the corner
    or hinged segment and the sections. A quadratic through one point
    twice retraces a line, so the boundary is not embedded. Recourse:
    move the corner apart, or loft at v-degree 1.
  - **Class 3**, in band: escalates, naming the corner.
- **Sections are apart beyond what they share.** The census's number
  (`[topology.census]`): the minimum distance between the two placed
  loops over every segment pair, beyond the corners classified pinned.
  A Zero there refuses `SectionsTouch { slab, segments }`. It catches a
  hinge slid along its own edge and sections that cross, which no
  corner reading sees. A lens turned about its chord (both ends
  pinned, arcs apart elsewhere) is a legal lune wall. Arc–arc pairs
  read a conservative separation (control triangles), and where that
  cannot decide, the door states the frontier.
- **The centroid arm reads the moving control rows**, the outer loop's
  compatible rows that are not classified pinned. Read off the
  vertices, it both misses a hinge and refuses a legal lens.
  Every corner pinned and every rim Zero is coincidence, named as
  `DegenerateStacking`.
- **The skin decides no pinning of its own.** Vertex rows read the
  door's per-corner verdict. Interior control rows keep the unbanded C6
  abstention.
- **Order at the fold's site:** corner classes, then clearance, then
  the normal step over moving rows, then the far normal (interim).

This row no longer waits on the certificate. It reads inputs, not
built walls.

## Closed (2026-11-02, PR 6141)

The loft door (`crates/sweep/src/loft/stacking.rs`, `stacking_door`) reads each slab in order, before the skin:

- **Corner classes** (`loft_corner_pinned`, per corner per slab per loop).
- **Coincidence.**
- **The class-2 retrace**: `PinnedCorner`, naming the corner or the hinged segment, the slab and the moving slab.
- **Clearance.**
  - Line pairs read through the census (`topo::line_segments_meet`, and `line_segments_overlap` over main's `parallel_overlap`), banded.
  - An arc pair refuses only on a meeting the curve-pair reader decided on both spans. An arc pair it cannot settle decides nothing here; it is the embedding certificate's, as on main.
  - A verified meeting refuses `SectionsTouch { slab, segments, shared }`.
  - A reader contradiction refuses as a kernel fault, `ClearanceFault`.
- **The step** reads the vertex centroid of the corners that move, so a slab where no corner moves gets no step.

Class 1, the wedge, refuses `PinnedCornerUnsupported` as a kernel limit, once every slab has passed. Its build waits on `work/charttail/a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint.md`.

On a 3458-case grid of arc lofts and sweeps with tilted, non-parallel sections, nothing that main refuses builds. 51 stadium bodies that main builds now refuse `SectionsTouch`, and each was checked as not embedded. The off-span carrier tangency that made the first fix pass refuse embedded elbows is evidence on CPAIR's `the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`.

**Review.** Single FULL, verdict NOT-MERGEABLE: the plane-only arc separation and the control-row step each refused embedded bodies that main builds. A focused delta review of the fix pass was also NOT-MERGEABLE. The off-carrier tangency still refused, and two census helpers that main had retired came back. The second fix pass closed both, per the orchestrator's rulings in `work/carve/log.md`. The orchestrator read the census diff, which is two thin doors over main's readings, and the grid diff.
