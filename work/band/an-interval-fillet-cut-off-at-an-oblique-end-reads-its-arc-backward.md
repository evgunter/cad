---
id: an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward
kind: issue
title: blend: a fillet cut-off at the sheared box's oblique end face refuses BodyNotIntact at Interval (its arc reads backward), where f64 builds
status: open
opened: 2026-10-13
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

**Pinned** by
`crates/sweep/tests/band_turn_overrun_fillet.rs`
`the_skewed_boxs_fillet_refuses_at_interval_as_its_plain_cut_off_does`;
the fix flips it, and the overrun's `Interval` row should then take the
skewed box too.
