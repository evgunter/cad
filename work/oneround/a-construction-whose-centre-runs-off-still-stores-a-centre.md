---
id: a-construction-whose-centre-runs-off-still-stores-a-centre
kind: unit
title: a construction whose centre runs off still stores a centre: four mints hold a point of their arc
status: parked
opened: 2026-11-22
priority: P1
cost: H
refs: [7258, 7203]
blocked_on: [one-stored-form-waits-on-the-readers-of-a-conics-centre, a-section-conic-is-anchored-at-a-general-point-of-its-arc, the-cone-sections-chart-image-and-its-kin-are-stated-from-the-conics-centre, a-conic-made-from-a-conic-is-rebuilt-from-its-centre-and-frame, the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre]
---


Owed by Ev's answer on PR 7258 (2026-11-22, "ok sounds good!";
`docs/DESIGN-FORK-LOG.md` row 296): "a construction whose centre runs
off at a pose it can reach holds the arc"
(`crates/geom-core/README.md`, "Float rounding").

Four kernel constructions whose conic's centre can run off to infinity
at a pose they can reach still mint a centre-held conic:

- the cylinder's tilted ellipse (a cylinder cut almost along its axis);
- `bisector_ellipses` (two equal cylinders almost parallel;
  `geom-brep/src/pair_section/cylinders.rs`);
- blend's oblique end face (`sweep/src/blend/open/end_face.rs`);
- `recognize_curve`'s circle in the STEP import.

Each holds a point of its arc instead, through the arc-held placement
PR 7203 lands for the ellipse (`Held::Arc { phase }`). **The wrong
choice is closed in code, not checked at run time** (Ev, PR 7258: "i
think rejecting ill-conditioned stuff is ok if we can do it as a panic,
i.e., it is a kernel bug if we choose the wrong one"): which constructor
such a mint may call is what stops a centre being stored, since there is
no number for a run-time check to test.

What the unit must find out first, per mint: which point of the arc the
construction holds exactly (PR 7203's cone section holds its near
vertex); whether the arc-held circle PR 7203 removed (no caller then)
comes back for the import's circle; and for the cylinder's shallow
off-axis cuts, whose in-box arc lies about half a radian from any
principal vertex, whether a general anchor is needed first
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc`, filed on
PR 7203's branch).

Ground shared with `sect`, `germ`, `tang`, `importgate`, `exch` and
`band`; a note goes on each live program's log when this is dispatched.

Waits on PR 7203.

## Parked (2026-11-23)

On `oneround/conic-readers`
(`one-stored-form-waits-on-the-readers-of-a-conics-centre`): until the
readers go through the conic, a second arc-held conic meets the same
derived-centre reads the cone's section meets today. And on
`a-section-conic-is-anchored-at-a-general-point-of-its-arc`: the
cylinder's tilted ellipse needs an anchor at a general point.

## Owed by the mint of the import's circle, from the conic-reader fork (2026-11-23)

If `recognize_curve`'s circle is held on its arc, two things on its path are owed first (`analysis/design-fork/conic-reader-interface`, rounds 5 and 6; all three designers):

- **An edge's `RevolvedPoint` description is not built from the circle's centre.** `step-import/adopt.rs` `mapped_self_description` (and `geom-brep/certify.rs` `arc_of_circle`) synthesize the description about the carrier's own axis line, and certification then compares `carrier(t)` with `description.eval(s)` sample by sample: one statement of the circle metered against another. The axis is the revolution's datum: that of the surface the rim lies on (the import has already held the rim to it), or, for a transient scaffold with no other entity, a description built from the conic that evaluates as the conic does.
- **`MappedSource::RevolvedPoint::eval` sums from the far axis.** It is `q + R·(p − q)` with `q` the axis origin, so its points round at `u·r` whatever supplies the axis (about 1e-10 m at `r` = 1e6, by one designer's arithmetic, not measured). A revolved point is a point of the arc and an axis, which is already the arc-held shape: evaluated as `p + (R − I)·(p − q)`, the axis origin enters at the arc's scale. `crates/geom-brep` `mapped.rs`; not traced into the pcurve derivations that consume the description class.

The mint is accepted on the centre datum's caller list holding only reporters of a format that is centre-form by its own definition (the STEP writer, a sketch arc written back) and mints of a new entity's datum (a blend surface about a rim's axis, a section loop's turning axis).
