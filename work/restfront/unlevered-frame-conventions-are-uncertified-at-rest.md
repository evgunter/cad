---
id: unlevered-frame-conventions-are-uncertified-at-rest
kind: issue
title: A line's unit dir, a plane's unit frame and u_ref ⊥ normal, and a cone's frame are certified nowhere at rest, and check 9 reads a line's dir as unit
status: open
opened: 2026-09-25
priority: P4
cost: D
refs: [3238]
---


## What

ATREST-13 made check 1 read every analytic frame: a zero `normal`,
`axis`, `u_ref` or `dir` is `PoisonedSurfaceDatum` /
`PoisonedCurveDatum`, and the `axis`/`u_ref` frame of a cylinder,
sphere, torus, circle, ellipse or spiric must be unit and orthogonal to
within the run's ε of locus movement at the kind's radius
(`geom::Surface::representability_margins`,
`geom::Curve3::representability_margins`, `geom::surfaces::frame_margins`).
The frame conventions it does NOT certify, each for a stated reason:

- **a line's unit `dir`** and **a plane's unit `normal` / `u_ref`**:
  each spans the same locus at any length, so a non-unit one is not a
  different locus. It is a different METRIC — `t` stops being arc
  length, `(p − o)·n` stops being a distance — and consumers read that
  metric as metres;
- **a plane's `u_ref ⊥ normal`**: a tilted `u_ref` tilts the chart
  plane off the implicit plane by an amount that grows with the
  distance from `origin`, which no stored datum bounds;
- **a cone's frame**: moves the half-angle, a locus movement that grows
  along the slant.

**Consumers that read the unlevered half as if certified** (ATREST-13's
sweep, curves): check 9's `locus_gap` (`crates/topo/src/validate.rs`)
computes a point's distance to a `Line` as `|d − dir·(d·dir)|`, which is
the distance only for a unit `dir` — at `|dir| = 2` it is not. Check 9's
`meet_segment` / `lines_meet` read the same `dir`. The measured import
surface: `step-import` adopts a near-unit `DIRECTION` verbatim within
the file's ε_in and divides by the norm otherwise
(`Resolver::direction`), so its lines are unit to ε_in; a struct-literal
`Curve3::Line` is not, and **ATREST-13's D-2 table** measured a pillow
chord re-minted with `dir = 2·x̂` (parameters `0 … 0.5`) and with
`dir = ½·x̂` (`0 … 2`): both mint and pass tier 3.

**Measured (ATREST-13, CI run 36154432046, every `validate_geometric`
call of the suite, the tour and the wild corpus):** the unlevered half
is not at rounding everywhere. A demos-job body (the tour's or the wild
montage's binary) carries a plane whose `normal` is `0.26` off unit; a
`bool1` row (`near_flush_regimes_pin_per_band`) a plane whose `u_ref`
is `1.6e-11` off `⊥ normal`; the wild corpus's lines are `6.4e-13` off
unit. None is refused, and none is a different locus as check 1 now
reads one.

## What must be decided

Whether each unlevered convention is tier 3's (with which lever: the
face's or edge's own extent is a body quantity, not a datum, so it
would be a metered decision rather than a representability read), or
whether the consumers above must read the metric they assume (a
normalized `dir`) and the convention is dropped.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1, check 9);
`crates/geom/src/lib.rs`'s conventions paragraph is `props` ground.

## Evidence from PR 3768's review (2026-10-02)

**A further consumer that reads a plane's `normal` as unit: the
plane×conic sections.** `geom_brep::plane_cylinder_section`
(`crates/geom-brep/src/intersect.rs:923`) takes the tilted semi-major
as `r / |axis·n|` and the parallel lane's gap as `(o − q)·n`; the
plane×sphere, ×cone and ×torus arms read the gap the same way. Fed a
plane whose `normal` is 1.264 long, the cylinder arm minted an
ellipse off the cylinder or a circle-shaped "ellipse" refused as
`CircularAxes` — measured on main at 5ab36cc95 through `topo::split`
(`work/tquery/split-refuses-cylindrical-feature-box.md`, Diagnosis).
PR 3768 closed the SPLIT door (`SplitPlane.normal: UnitVec3`), but
the boolean's germ planes still reach the same arms with a
`Surface::Plane` carrier's bare normal
(`crates/topo/src/boolean/join.rs:451-500`, through
`chord_join::SectionPlane`, `crates/topo/src/chord_join.rs:641`,
and back into a transient `Surface::Plane` at `chord_join.rs:1331`).
`SectionPlane`'s doc calls those carriers "unit under the surfaces'
at-rest rule", which is the convention this row records as
uncertified (`crates/geom/src/lib.rs:69-71`). No public door that
mints a non-unit plane carrier on that head was found by the review
(step-import divides a non-unit `DIRECTION` by its norm; the split now
mints unit section faces), so the hole is latent rather than shown.

**The measured 0.26-off-unit demos plane is very likely the cutaway's
section face.** `|(0.75, 0.1875, 1)| = 1.264`, and before PR 3768 the
split stored the caller's normal verbatim on every section face it
minted. Once PR 3768 is on main, the "What" section's demos-job
measurement wants re-taking before it is cited.
