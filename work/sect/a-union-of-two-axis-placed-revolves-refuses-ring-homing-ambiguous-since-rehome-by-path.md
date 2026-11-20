---
id: a-union-of-two-axis-placed-revolves-refuses-ring-homing-ambiguous-since-rehome-by-path
kind: issue
title: A union of a housing, a shaft and a collar on one axis refuses RingHomingAmbiguous since PR 6047 (rehome by path): coincidence_door's axis-edit row red
status: closed
opened: 2026-11-02
priority: P0
cost: M
refs: [a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord, a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading]
closed: 2026-11-20
pr: 7181
---


Filed by a GREEN lane (`green/main-red-editor-core`, 2026-11-02) from
`work/green/main-red-the-axis-edit-mates-the-copy-coincidence-door-row`.
Banded P0: this is a union of a housing, a shaft through its bore and a
collar, all revolved about one axis. That union built until this merge and
refuses now, so it is a normal verb broken on normal geometry.

## What

`crates/editor-core/tests/coincidence_door.rs`,
`the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order`
(slow set), fails at every ε row. The fixture is a tube housing (r
0.5..1), a shaft (r 0..0.5) placed in its bore, and a collar (r
0.5..0.75), each placed on the housing's axis by a mate or by values,
then unioned. The union no longer evaluates:

    UnionFoldStep { member: 2, read: VarId(…ordinal: 50…),
      refusal: Boolean(Join(BooleanJoin(RingHomingAmbiguous { ring: LoopKey(1v3) }))) }

## Where it came from

The first bad merge is PR 6047 (`e81e4ff9c1`, `sect/rehome-by-path`,
closing `a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord`).
Its first parent `7ac2e3025b` passes the row, and so do `6335ffd324` and
`5acea3cb35` before it. PR 6047 moved a cylinder face's ring re-homing
in `topo::chord_join` from the chart walk to the path reading
(`ring_path`). The ring the union's third member leaves on a coaxial
cylinder wall now reads ambiguous. Perhaps every vertex of the ring sits on
the run (`a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading`
on TANGTAIL, P3, describes such a ring), but this is not read here.
It is red on nightly 79 (run 41276693129, main `fbfebd9207`) and on main
`a285cdbf`.

Reproduce (opt-level 2, as the nightly runs it):

    CARGO_PROFILE_DEV_OPT_LEVEL=2 CARGO_PROFILE_TEST_OPT_LEVEL=2 \
    cargo nextest run -p editor-core --profile ci --ignore-default-filter \
      -E 'test(=coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order)'

## Pinned red

The row is pinned at every ε to stop on this refusal
(`test_utils::pinned_red::row`), citing this row.

## Done when

The union evaluates again and its wall findings hold as the row states,
and the pin is dropped.

## Closed

2026-11-20, by `sect/coaxial-rehome` (PR 7181). The refusing ring was
a pierce ring at (0.5, 1, 0) on the shaft's half-wall, strictly inside
the run y ∈ [0, 1.5]. It was not the TANG row's all-on-the-run ring.
Every ruled path from it to every outer-loop reference ran through a run
vertex, because the coaxial revolves' seam and section vertices share
its azimuth and height. `chord_join::path_ring_side` now asks detours
after the plain pairs (`ring_path::Carrier::detours`). Each detour turns
at the azimuth or the height of a run edge's midpoint. The row is
un-pinned and passes at all three ε. The fast reproducer is
`wall_ring_rows::a_ring_vertex_aligned_with_run_vertices_reads_its_side`.
The same gap on a sphere, a torus, a spline wall and in the island
winding is filed as
`a-path-reading-offers-no-detour-on-a-sphere-a-torus-a-spline-wall-or-to-an-island-chord`.
