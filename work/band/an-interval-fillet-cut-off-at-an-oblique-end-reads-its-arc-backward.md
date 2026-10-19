---
id: an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward
kind: issue
title: blend: a fillet cut-off at the sheared box's oblique end face refuses BodyNotIntact at Interval (its arc reads backward), where f64 builds
status: closed
closed: 2026-10-19
pr: 5346
opened: 2026-10-13
priority: P2
cost: M
---


Found by the fillet overrun's `Interval` row (step 3 of
`a-non-isosceles-turn-overruns-past-the-mitre`); reproduces on main at
`977a8c5d` with no turn involved.

**Witness.** `common::operands::parallelepiped_at::<Interval>(0.3)`,
finished, one edge requested: the top edge from `(0.3, 0.3, 1)` to
`(2.3, 0.3, 1)`, filleted at `r = 0.1`. It refuses
`BodyNotIntact { at: Vertex(12v1), detail: "a cut-off's round end curve
does not run forward from one foot to the other" }`, raised by
`crates/sweep/src/blend/open/end_face.rs` `CapSliver::of` where
`(tb − ta).lo() <= 0.0` after `param_near` reads the two feet on the end
ellipse. The same request at `f64` builds, and so does the lateral edge
alone at `Interval`. Requesting the top edge with the lateral edge (the
skewed box's overrun turn) refuses the same way at the turn vertex.

**Suspected, not measured.** Both end faces of that edge are oblique to
it, so each end is an ellipse whose minor axis runs along `y`, the
direction of the top rim; the foot on that rim is then where the
ellipse's tangent runs along its minor axis, a major vertex, param `π`
from `u_ref` (`cap_transverse` puts `u_ref` on the major axis). An
`Interval` param read at `±π` may land on the other branch from the hint
`0`, which would leave `tb` before `ta`. The measure-first step is to
print `ta`, `tb` and the feet's enclosures at the raising site.

**A second reader of the same window.** The description pass
(`crates/sweep/src/blend/surgery.rs` `attach_contact`, its
`ContactCarrier::Transverse` arm) windows every cut-off arc between the
edge's two vertices with the same hint, `param_near(p0, T::zero())`, and
since PR 4962 that includes a fillet overrun's edge. An `Interval` foot
at a major vertex can read the other branch there too once
`CapSliver::of` stops refusing first: where `(t1 − t0).lo() > 0` fails
the carrier is run back, so a wrong branch would describe the long way
round rather than refuse. A fix closes both readers.

**Pinned** by
`crates/sweep/tests/band_turn_overrun_fillet.rs`
`the_skewed_boxs_fillet_refuses_at_interval_as_its_plain_cut_off_does`;
the fix flips it, and the overrun's `Interval` row should then take the
skewed box too.

## Closed

Measured at the raising site: `ta = [−π, π]` (`Def`), `tb` the whole
`[−2π, 2π]`. `foot_a`, on the top rim, is the end ellipse's major
vertex half a turn from `u_ref`, and the `atan2` box read from the
seam straddles its branch cut, which `Interval::atan2` answers with the
hull. `f64` reads `±π` there and runs forward from either.

`sweep::blend::surgery::arc_window` is now the one window read for both
readers, `CapSliver::of` and `attach_contact`'s `Transverse` arm: it
reads `p0` at the seam and, where that enclosure spans half a turn or
more, again at the half turn, whose cut is half a turn away. Each
mutant of one reader alone is caught (the description pass refuses at
`interval_span_forward`, not the long way round).

Rows: `band_planar_cut_off_interval`
`a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar` (shears
`±0.1, ±0.3, ±0.6`, convex and concave, both verbs, `Interval` against
`f64`, tiers 3 and 3′), and the skewed box joins
`band_turn_overrun_fillet::the_overrun_carves_at_the_certified_scalar`;
the refusal pin is gone. Filed from the class sweep:
`a-rim-split-at-its-carriers-anti-seam-refuses-at-interval` and
`work/issues/a-seam-anchored-param-read-straddles-the-cut-at-interval.md`.
