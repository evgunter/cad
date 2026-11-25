---
id: a-vertex-alone-quadrant-a-cell-holds-along-one-edge-has-no-width-to-depart-across
kind: issue
title: A plane face presented as its quadrant at a vertex alone, in a cell that holds it only along one of its edges, has a rectangle of no width across that edge at every depth, so its pair splits to the floor though a plane parts the wedges
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Two plane faces sharing a vertex alone, each presented there as its
quadrant (`Zero::Point` on both), refuse `Uncertified { Departure }` at
the vertex although `planes_through`'s wedge plane parts them
(`wedges_apart`, `topo::injective`). In every refusing pair one of the
two presented rectangles has an extent of at most `5e-15` along one
axis, at every depth from the first split to the floor, for example
`(0, 6.9e-16) × (0, 0.125)` and `(0, 2.0e-16) × (0, 4.9e-4)` at depth
40. `sign_on`'s `Zero::Point` arm decides each axis's departure as
`∇ψ·Φ_axis·span` at the band, so an axis with no span is decided by no
plane, however wide its margin.

## Evidence

Measured on `embed/vertex-bisector-measure` (main `9bc9d6517` with the
at-rest gate of `embed/gate-measure`, and the wedge plane), with the
gate asking, over the census's vertex plane/plane tests. Four tests
still refuse first at a pair presented `Zero::Point` on both faces:

- `split_through_an_apex::a_split_through_an_apex_closes_each_corner_of_its_section_on_its_own`
  (`topo::all`), faces `3v1`/`14v1`, `7v1`/`14v1`, `10v1`/`14v1`;
- `join_pierce_runs_sweep::a_shared_vertex_fan_reads_its_cuts_from_its_own_first_entry`
  (`sweep::all`), eleven pairs, among them `8v3` against six faces;
- `join_rc_probes::reflex_corner_edge_in_face_poses_build_every_op_undeclared`
  (`sweep::all`), `3v1`/`14v1` and `8v1`/`14v1`;
- `band_mixed_vertex::the_odd_edge_with_one_other_chamfers_at_the_flush_fixture`
  (`sweep::all`), `13v1`/`18v1`.

In each, the wedge plane leans at least `0.10` (cosine) from every one
of the four edge directions (`0.10` to `0.99`), so the plane is not the
cause; the heading plane does not part them either. Without the wedge
plane `split_through_an_apex` and `a_shared_vertex_fan` refuse on the
same first pair (`3v1`/`14v1`, `5v1`/`8v3`), while `band_mixed_vertex`
and `join_rc_probes` refuse earlier on another (`9v3`/`20v1`,
`3v1`/`12v1`), which the wedge plane certifies.

**The cause, probed** (the review of PR 7318, `CV_ZW` on
`embed/vertex-bisector-review`'s measuring diff), on
`split_through_an_apex` at `3v1`/`14v1`, vertex `6v1`: the cell side
`v = 0` runs through the vertex along `14v1`'s edge, and the face lies
on the far side of it. The presented rectangle is `x = (0, 2.4e-15)`,
whose exact extent is 0: `present_plane`'s corner reading
(`present_vertex_corner`) returns `Vacuous` where a corner coordinate's
upper end is `≤ 0`, and the outward rounding of
`(d × b)/det` leaves it `2.4e-15` above. The wedge plane's margins are
strict and opposite (`+0.0535`, `+0.0528` against `−0.0528`,
`−0.2377`), and `sign_on` reads `None` only because
`−0.0528 × 2.4e-15` is inside the band. The same probe on the other three (2026-11-22, `CV_ZW` on
`embed/vertex-bisector-measure`) finds the same at every refusing pair's
last cell: a cell side lies on the vertex, inside the vertex's own
chart bracket, and the face's rectangle has no width across it (the
wedge plane's slopes strict and opposite on both faces):

| test | pair, vertex | the cell side on the vertex | width | slopes |
|---|---|---|---|---|
| `band_mixed_vertex::the_odd_edge_with_one_other_chamfers_at_the_flush_fixture` | `13v1`/`18v1`, `23v1` | `18v1`'s `v = −0.14142135623730967` | `2.0e-16` | `+0.879, +1.884` / `−0.080, −0.184` |
| `join_rc_probes::reflex_corner_edge_in_face_poses_build_every_op_undeclared` | `3v1`/`14v1`, `7v1` | `14v1`'s `v = 0.8944271909999163` | `1.9e-18` | `+0.279, +0.790` / `−0.312, −0.452` |
| `join_pierce_runs_sweep::a_shared_vertex_fan_reads_its_cuts_from_its_own_first_entry` | `5v1`/`8v3`, `11v1` | `8v3`'s `v = 2.0000000000000013` | `3.0e-15` | `+0.362, +0.373` / `−0.460, −0.746` |

So all four are this cause.

**This is an instance of the ruling on PR 7095**: a departure margin is
metered at the reach of the faces it governs, never at a cell of the
question's own subdivision. Here the margin is metered at the cell's
`2.4e-15`, where the face reaches the edge's whole length. The
orchestrator has passed the four tests to `embed/remeter`, which builds
that metering, as pins for the vertex-only case.

## What a fix needs

Three readings would close it, and the ruling picks among them:

- **meter the margin at the face's reach** (the ruling): `sign_on`'s
  `Zero::Point` arm reads each axis's departure as `∇ψ·Φ_axis` times the
  face's reach along that axis (the edge's length from the vertex), not
  times the rectangle's span, so a rectangle with no span is decided by
  its slope;
- **decide the vacuity exactly**: where the cell's bound is the vertex's
  chart coordinate exactly and the edge lies along it, the face has no
  point in the cell off its edge, so the cell is `Vacuous` by structure,
  with no band;
- **read the vacuity at the band**: a rectangle whose extent across an
  axis is within the band as `Vacuous`. That is a sliver concession, and
  the sliver is with Ev to retire (PR 7095), so not this one.
