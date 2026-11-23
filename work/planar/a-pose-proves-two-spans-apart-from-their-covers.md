---
id: a-pose-proves-two-spans-apart-from-their-covers
kind: unit
title: the curve-pair pose proves two spans apart from where each can lie before a carrier reading in the band escalates
status: open
opened: 2026-11-22
priority: P1
cost: H
---


PLANAR's first unit (`work/planar/plan.md`). It helps every reader of
`meet` today, so it lands first and on its own.

## The ruled text

C5 (`crates/geom-brep/README.md`), the sentence PR 7290 added at the
end of the co-surface paragraph: *"A pose proves two spans apart from
where each can lie (its cover's distance range from the other carrier)
before a carrier reading in the band escalates."* `[one-way.readings]`
makes the bound a method on the span's cover. C5's *"A candidate
generator's in-band decision escalates and never prunes"* stands: the
stage decides "apart" only definitely and adds nothing in band.

## The defect it repairs (measured)

`geom_brep::curve_pair::meet` escalates whenever two carriers' clearance
lands in the band's gap, whatever the spans: the pose refuses before any
candidate is read. Measured on main 699a1d2a8b by a PLANAR designer
(probe `crates/geom-brep/examples/planar_probe.rs` on the branch
`planar/probe-meet-on-planar-pairs`, pushed and not for merge; its log
is beside it), ε = 10⁻⁹, K = 10, `f64` and `Interval`,
both orders, a carrier clearance of 5.5·10⁻⁹:
- a line ending 1 m short of a graze;
- a line under a circle whose arc is the far half;
- two half-circles on the far sides of an external near-tangency, and of
  an internal one;

each escalates `curve_pair_pose_extreme`; two arcs on carriers
5.5·10⁻⁹ from one circle, spans far apart, escalate
`curve_pair_pose_one_curve`. Profile validation accepts all five today
(`seg`'s `segment_clear_of_circle`, `arc_clear_of_circle`). CPAIR's
`the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`
(claimed here, `parent:` this unit) has two more, as `meet_rows`
witnesses, and a loft door (`sweep/src/loft/stacking.rs`
`arc_pair_meets`) and a census row reaching the shape.

## Where it goes (read in the tree, 2026-11-22)

`crates/geom-brep/src/curve_pair/pose.rs`. The in-band carrier readings
the stage must precede:
- `conic_conic`: the `constant_as(row::PARALLEL, …)` height screen, the
  `row::SWING` readings, `one_curve` (`curve_pair_pose_one_curve`), then
  `transverse` → `height_extremes` (`curve_pair_pose_extreme`), or the
  coplanar arms `coplanar_circles` (`owed`) and `coplanar_ellipses`
  (`subdivided`);
- `line_conic`: after `route`'s `Route::Level`, the `row::SWING` reading
  and `owed` over the whole circle;
- `line_line`: `route`'s gap already reads the projections
  (`curve_pair_pose_gap`); its in-band arms are the tilt and the offset.

The cover side exists: `geom::cover::Cover::distance_range(&Site)` from
a point or a line, and `Cover::range_about(&Around::Circle(core))` from a
whole circle (`crates/geom/src/cover.rs`); a span's cover is
`Curve3::cover_over(ParamWindow)` (`crates/geom/src/cover/readings.rs`),
an arc's piece met with its whole conic's (`Cover::arc`).

## Done when

- Each of the seven witnesses above (five from the probe, two from the
  claimed row) is a `meet_rows` row, at `f64` and `Interval`, both
  orders, and `meet` reads `Meetings { shared: None, points: [] }`
  where it escalated on main.
- The stage reads only definitely: a row whose spans' cover ranges
  straddle the band still escalates on the carrier reading's own row
  (named), so the stage adds no refusal and removes none that is in
  band on the spans themselves.
- A row where the spans do meet (a crossing within the band's
  escalation width of the carriers' extreme) still returns its meeting:
  the stage never reads "apart" for a pair with a meeting.
- The stage's margins carry their rounding charge (each a named
  `k_stats` row, for example `curve_pair_pose_span_clear`), charged as
  the pose's other screens are.
- The census, boolean and loft suites at the default ε row are green;
  every verdict that moves is an escalation becoming decided, and the PR
  lists them.

## First check, and what to do if it fails

**Whether a cover distance range is tight enough on an arc** to clear
the five fixtures at the band's gap: the stage is useless if an arc's
`cover_over` range from the other circle's centre (or from the other
circle, `Around::Circle`) is a ball or box bound loose by more than the
band. Read `Piece::Arc`'s `distance_range` and `circle_range` in
`cover.rs` (unread by every designer; one checked `distance_range`'s
existence by name only), and run the five fixtures through it first.
**And whether an ellipse pair has a site at all**: `Site` is a point or
a line and `Around` adds a circle; nothing measures a distance from an
ellipse. If the arc ranges are loose, or the ellipse pairs need a site
the cover does not have, stop and report: the cover's readings are
BOXES' and ONECOPY's ground and the fix is a cover method, not a pose
spelling.

## Leaves for later

- The plane instance's entry, which reads this stage through the shared
  pose: `two-curves-in-one-plane-meet-by-the-in-plane-arms`.
- The kind on a meeting: ONECOPY's
  `a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point`.

## Rows

- Closes CPAIR's `the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`
  (claimed into `work/planar/`).
- Touches CPAIR's `a-same-side-touch-pose-escalates-the-curve-pair-extreme-at-1e-12`
  and `the-pose-screens-clear-and-parallel-record-minus-k-eps-and-land-in-band-under-k-lint`
  (same rows of the pose): the lane reads both and says in its PR whether
  either moves.

## Territory

`crates/geom-brep/src/curve_pair/pose.rs` and `meet_rows.rs`: CPAIR and
ONECOPY (`work.py territory`). Possibly `crates/geom/src/cover.rs` (no
program). It changes what `meet` answers for the census
(`topo/src/census/curved.rs`: ATREST, HELDRECORDS), the boolean
(ONEHOME), the loft door (CARVEREST): the PR announces on CPAIR's and
ONECOPY's logs, and on ATREST's and ONEHOME's where their goldens move.

## Review

**Dual**, difficulty **H**: a new definite "apart" in the one reading
every curve-pair consumer shares, argued from an interval enclosure; a
wrong one silently passes bodies.
