---
id: census-containment-refusal-carriage-has-no-executed-fixture
kind: issue
title: the census's point-in-face refusal carriage (CensusUnsupported::Containment) has no fixture that reaches it through the public door: the lens cap it rode on is walked on its carriers
status: closed
opened: 2026-09-26
closed: 2026-09-26
priority: P3
cost: E
---


Filed by CONTACT-4.

`census.rs`'s `contain` helper (the `ContainError` → `CensusUnsupported`
routing for `ArcLoopUnsupported`, `RayExhausted` and `Corrupt`) had
exactly one executed fixture: `crates/sweep/tests/census_containment_cause.rs`'s
lens under a box. The lens cap is a two-vertex loop of two arcs. Once
`contfp` walks loops on their carriers, it answers that cap: the census
now reports four `VertexOnFace` contacts and no refusal
(`the_lens_cap_is_read_as_a_region`). The two refusal rows
(`no_fabricated_containment_margin` and
`the_arc_loop_refusal_reaches_the_user_as_itself`) could no longer fire
and were deleted. The census unit row
`a_region_walk_refusal_on_a_separated_vertex_is_the_filters_to_answer`
(the pruning filter's REFUSAL class member) lost its fixture the same
way and became
`a_separated_vertex_in_a_half_disc_caps_plane_is_decided_by_both_sweeps`.

**What can still reach the arm:** a planar face with a spiric or spline
edge, asked about a point within that edge's reach ball. The sectioned
torus vessel's cavity (`crates/sweep/tests/spiric_rim.rs`,
`vessel_cavity`) has one: its moved section cap `z = −t` carries a
spiric rim. With a brick standing on that cap near the axis,
`validate_pseudomanifold` stops at `VolumeUncomputable` before the
census runs. So no public-door body reaches the arm today.

The candidates are a crate-internal census row on a hand-built planar
face with a spiric or NURBS edge (like the census unit tests'
`half_disc_cap_and_far_cube`, which builds through `mev`/`mef` with a
carrier spec), or a public fixture once the cavity's volume lands. The
editor-core attribution suite still pins the three causes by value.

## Closed by CONTACT-4's fix pass

`census::tests::a_spiric_caps_refusal_reaches_the_census_as_itself`
executes the carriage. Its fixture is `spiric_cap_and_near_cube`:
- a planar cap in `x = 1/2`, built through `mev`/`mef` with a certified
  spiric carrier (the section of the torus `R = 2, r = 1`) and its
  chord;
- a cube grafted beside it, whose `x = 1/2` face lies outside the cap
  but inside the ball the spiric arc is held in.

Under the exact sweep the census pushes `CensusUnsupported` about the
FACE with cause `Containment(ArcLoopUnsupported)`. The row also asserts
the "no fabricated margin" invariant: there is no `CensusEscalated` with
an `Invalid` margin or a `pm_census_containment` /
`bool_contfp_boundary` predicate.

`RayExhausted` and `Corrupt` still have no executed body. The sweep
suite's header says so.
