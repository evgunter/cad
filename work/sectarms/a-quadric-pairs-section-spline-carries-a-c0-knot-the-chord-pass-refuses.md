---
id: a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses
kind: issue
title: a closed section loop seated off its start carries a full-multiplicity join knot inside an edge, so booleans of four quadric pairs build and never mesh (and tier 3 will refuse them)
status: open
opened: 2026-11-14
priority: P0
cost: M
design: true
---


Found by TESSLANE's reach measurement (2026-11-14, `d6e1eb1f`; probe `reach_probes::a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass` on branch `tesslane/measure-reach`).

## Measured

A cylinder of radius 0.5 against a ball of radius 0.3 centred at `(0.5, 0, 0)` (the first pose of `cylinder_sphere_frame`): all six boolean ops build, and `mesh::tessellate` refuses all six at δ 2e-2, 5e-3 and 1e-3:

    UnsupportedCurve { note: "B-spline carrier with a C⁰ kink (interior multiplicity = degree) …" }

The raising site is `chords::nurbs_chord_count`, in the chord pass, before any face is dispatched. The section edges carry `Curve3::Nurbs` with `Pcurve::Projected` images on both the cylinder and the sphere face. Three more poses (the ball's centre inside the cylinder, outside it, and a two-loop section; ∩ only, δ 5e-3) refuse the same way. The cylinder face has a trimmed arm and would mesh; the refusal is ahead of it.

So no boolean of a cylinder and a ball in a general pose can be drawn.

## Where the fix belongs

`nurbs_chord_count`'s secant bound needs C¹, and it refuses any interior knot of multiplicity above `p − 1`. Ev's ruling on PR 5913 (DESIGN `[topology]`) says an edge carrier is C¹ by its knot structure strictly inside its cell, and that a construction whose carrier is not puts a vertex there; no reader carries a crease arm. Read with that, the carrier this section mints is the defect, and the chord pass is right to refuse it. Not yet measured: whether the knot is a true tangent break or a full-multiplicity knot on a smooth curve (a piecewise-Bézier fit), which decides between cutting the edge at the knot and minting the curve with a C¹ knot vector.

The mint is on other ground: `geom-brep`'s `ssi.rs` (`cylinder_sphere_ssi`; SSIARITH, SSIEDGE) and `pair_section.rs` (the SECT family). This row sits here because the refusal is the mesh's and the mint's owner is not yet identified; whoever owns the mint claims it by moving it. `tier-3-refuses-a-carrier-with-an-interior-c0-knot` (MEASUREDOOR) will refuse these bodies at rest once it lands, so this row is on its path too.

## Done when

The six ops of the measured pose mesh and pass `check_mesh`, and the probe is re-pinned to that.

## Measured: who mints the knot, and what it is (2026-11-14, TESSLANE) — re-homed to SECTARMS

A second lane traced the carrier and measured the knot at `ee384dc243`. Run unless marked read. It corrects this row's guesses above: the mint is not in `ssi.rs`, and the knot is neither a tangent break nor a piecewise-Bézier joint.

**The mint is the seat.** `pair_section::seat_at` (reached from `topo::boolean::join::carried_seats` → `SectionTable::seated` → `PairSection::seat`) takes a closed fitted section loop that has a site on it, splits it at the site's foot and rejoins the two pieces with `rotated`, whose doc says the join knot has "multiplicity the degree … so the joined carrier is continuous"; `ssi::certify_rung3` re-certifies it and `chord_join::cut_arc` cuts the edges out. The loop itself is one global cubic interpolant of the marched points (`ssi::refine::fit_branch`: clamped, non-periodic, no end-tangent condition), all of whose interior knots are simple. So the full-multiplicity knot is the closed loop's own old start and end, moved inside an edge by the rotation.

**What it is.** On the measured pose each op has four spline section edges of degree 3 and about 265 control points; exactly one carries one interior knot of multiplicity 3, strictly inside its cell (4.9e-2 m from the nearest vertex). At that knot the point is on both surfaces exactly, the left and right unit tangents differ by **2.4e-7 rad**, the speeds by 2.4e-6 relative and the curvatures by 1.3e-5 relative. It is a C⁰ knot by structure on a locus that is smooth there: the two sides are the two free ends of one open interpolant.

**It is every seated quadric pair, not this one.** Each of these built and refused the same way at three δ, with one multiplicity-3 knot per seated loop:

| pair | ops run | tangent jump at the knot |
|---|---|---|
| cylinder × sphere (this row's pose) | all six | 2.4e-7 rad |
| cylinder × cylinder, a rod r 0.2 across a rod r 0.5, off-axis, two loops | ∩, ∪, ∖ | 9.95e-7 rad |
| sphere × torus (a pose of `germ_sphere_torus_placements`) | ∪, ∩ | 7.2e-8 rad |
| cylinder × torus, a rod through a donut, two loops | ∪, ∩, ∖ | 2.2e-7 rad |

Read, not run: torus × torus takes the same route; a closed circle or ellipse is seated by span alone, with no knot edit; a spline × implicit closed chart loop and a spline × spline whole are refused by `seat_at`, so are never seated; a cone pair refuses earlier. The knot appears when a closed fitted loop has a site and is seated off its own start; a loop with no site, or seated at its start, was not run.

**Seating was a logged decision.** `work/sect/log.md`, 2026-10-28, step 1b (PR 5894): a closed fitted component is seated by knot insertion to full multiplicity, then rotation, then re-certified; "Periodic NURBS in `geom` was the alternative … Reopen it as a fork if seating proves unsound." That is the day of Ev's ruling on PR 5913, which makes an edge carrier C¹ inside its cell. The two contradict, and what a seated loop's carrier is is a design question on the section's ground, not the mesh's (`design: true`).

**Who else is on its path.** Tier 3 has no knot-structure check today; `tier-3-refuses-a-carrier-with-an-interior-c0-knot` (MEASUREDOOR) will refuse every body above, and names two predecessors, neither of them this. Readers that already carry their own handling of a creased edge carrier: `mesh::chords::nurbs_chord_count` (refuses), `geom::nonrational_second_derivative_sup`, `NurbsCurve3::accel_upper_bound` (∞), `topo::boolean::edge_join::crease_at`, `geom_brep::curve_face::composite::creases`, `step_import::knot_split::split_edges`.

**Why SECTARMS.** `pair_section.rs` is SECT's, SECTARMS's, SECTCONE's and SECTTORUS's; the seat is SECT's step 1b by charter, and SECTARMS is the one of the four with an orchestrator. Moved there by `git mv`; if SECT is the better home, its owner moves it on. `work/tesslane/cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm` stays parked on it, and `reach_probes::a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass` pins the pose.
