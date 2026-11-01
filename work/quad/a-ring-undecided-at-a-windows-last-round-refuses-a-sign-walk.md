---
id: a-ring-undecided-at-a-windows-last-round-refuses-a-sign-walk
kind: issue
title: A ring undecided at a window's last round refuses the sign walk while rounds remain
status: open
opened: 2026-10-22
priority: P2
cost: M
---


## What

`geom_brep::props::quad::cylinder_cut_face_rounds` and
`geom_brep::props::spiric::rim_face_rounds` read every ring's winding
(`props_ring_winding`) at whichever round a window returns. Since the
fix for
`an-undecided-ring-at-the-first-converged-round-refuses-instead-of-refining`,
a ring the converged enclosure leaves straddling the band refines on
while the WINDOW has rounds (`quad::Run::ringed`). At the
window's own last round the escalation still stands, though the
schedule has rounds left: an `Open { end: OpenEnd::Window }` would
hand back an enclosure that encloses the face's flux only if the ring
is a hole, and `RoundOutcome` has no outcome for "no enclosure at this
round, resume at the next".

So the whole schedule (`RoundWindow::SCHEDULE`, `mass_properties`)
measures a face the single-round windows refuse. Measured on the
witness row
`props::quad::cylinder::tests::a_ring_undecided_at_the_first_converged_round_refines_on`
(a 2 × 1 chart rectangle with a circular hole of radius 1e-5 or 2e-6
about `(1, 0.2)`, 3e-7 about `(1, 0.95)`, 3e-8 about `(1, 0.2)`):
`RoundWindow::at(0)` escalates `props_ring_winding` on all four,
while the schedule measures `2 − π·r²`. Probed per round on main
before that fix, rings of radius 3e-8 to 3e-7 off the anchor level
escalated at `at(k)` for k up to 2–4 and converged with the ring
decided from k = 3–5 on.

`topo::props`' sign walk (`walk`, `RoundWindow::at(round)`) and
`target_walk` drive the face one window at a time, so a face like the
witness reaches them as a hard refusal at round 0. Whether a body the
kernel builds reaches that state is unmeasured.

## The shape of a fix

An outcome for a window that ends on an undecided in-band decision
with rounds remaining, carrying no enclosure and the round to resume
at; the walks then treat the face as unsettled at that round (no sum
can settle without it) and resume it, and the reporting walk
escalates only at the schedule's end.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `RoundOutcome`, `OpenEnd` and `target_walk` are gone
(PR 5628), but the gap is unchanged. `Run::ringed`
(`crates/geom-brep/src/props/quad/run.rs:391`) refines an undecided
ring only while `round < self.window.last`, and its doc (`:385`) cites
this row: `LaneRun` has no end for an enclosure whose rings are
undecided. The one-round windows are now `walk_set`'s
(`crates/topo/src/props.rs:877`, `RoundWindow::at(round)`) and round
0's (`:982`). The ringed lanes are `cylinder_cut_face_rounds`
(`props/quad/cylinder.rs:303`) and `rim_face_rounds`
(`props/spiric.rs:841`). The fix's shape now reads: a `LaneEnd`
for a window that ends undecided on its rings with rounds left, which
`walk_set` resumes.
