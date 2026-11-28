---
id: conic-constructions-from-computed-sizes-unaudited
kind: issue
title: About 240 kernel sites build a circle or ellipse from a computed size; each should decide its sign before construction or be provably positive, and most are unaudited
status: open
opened: 2026-11-20
priority: P1
cost: H
---


Filed by GERM's `germ/ellipse-speed-magnitudes` lane (PR 7183). That PR
makes a negative or non-finite conic size unrepresentable:
`geom::Circle::new` and `geom::Ellipse::new` panic on one
(`crates/geom/src/curves/conic.rs`, `stored_size`). A kernel site that
computes a size and builds a conic from it now panics where it used to
store a signed or NaN size. The hosted suite surfaced six such sites,
fixed in that PR: `sweep::blend::battery::frustum_arm` (a chamfer's
plane trim past the axis), `geom_brep::pcurve_cache::limb2_memo::rendered`
(a memo key built conics at bracket ends), and four test fixtures.

## The sweep that remains

The lane listed every `Curve3::circle`, `Circle::new` and `Ellipse::new`
call outside the obvious test files whose size argument is not a
literal. There were about 248, the count including in-file test
modules that the listing could not tell apart. The lane read these:

- **decided before construction:** `geom_brep::intersect` plane × sphere
  (`ps_center_gap`, `intersect.rs` near 1457), cone × torus
  (`kot_apex_station`, `kot_reach`, near 3276), `Curve3::ellipse`, and
  STEP `CIRCLE`/`ELLIPSE` (`interpret.rs`, `positive_length`);
- **provably non-negative:** `sweep::blend::arms::Meridian::blend`
  (`ring_radius` turns a signed radius into its magnitude and the seam),
  STEP `build.rs` sphere stations (`radius·dir.x`, with `dir` the unit
  direction of a radial distance), `topo::offset_axial` concentric
  sphere seam (`radius + distance`, a great circle of the offset sphere);
- **stored values re-read:** `scalar_lift`, `end_face` (`major_frame`),
  `ring_path`, and `signed_zero`.

The rest are unread. Each needs one of those dispositions, or a pose
decision before the constructor on its own lever and band.
Reproduce the listing by grepping `(Curve3::circle|Circle::new|Ellipse::new)\(`
under `crates/*/src` and dropping the literal sizes.

## Progress

2026-11-23, branch `germ/conic-constructions-audit`. The listing on
main (e3fdefa36) is 796 hits under `crates/**/src`, of which 729 sit in
test modules (in-file `#[cfg(test)]` blocks, and `*_rows.rs` /
`*_fuzz.rs` files wired by `#[cfg(test)] #[path]`), leaving 67
production sites: the "about 240" above counted test modules. All 67
are read, and the PR body carries one line per site.

- **Sign.** 3 literals; 24 decided before construction; 9 provably
  non-negative; 25 stored values re-read; the door itself; and 5 that
  are open in part. Two were defects on data and are fixed with rows:
  `plane_cylinder_classified` stated its rim at an unguarded stored
  radius (`pc_cylinder_radius` now), and the offset doors decided
  `surface.radius + d` while they stored `circle.radius() + d`
  (`topo::replace_face::moved_radius` decides the stored sum; the torus
  parallel is re-posed as `ρ + d·cos v`). `ring_path`'s `parallel` is
  computed (norms), not stored as listed above.
- **Finiteness is a second question, and it is the larger one.** No
  sign decision gives a finite size, and the constructors require one:
  `no-sign-decision-gives-a-conic-a-finite-size` has the reproductions,
  what this PR closed, and the design question it left.
- **Filed elsewhere:** BAND
  `the-blend-arms-mint-conics-before-a-predicate-reads-an-oversized-radius`,
  CPAIR `on-surface-curve-doors-state-a-raw-surfaces-radius-undecided`,
  TRACE `circle-crossings-take-a-callers-radius-undecided`, and a note
  on SECT's `section-arms-off-the-value-read-radii-unguarded`.

2026-11-26, the review's fix pass on the same branch. Finiteness is
read through one predicate, the constructors' own
(`geom::conic_size_read`); `is_finite_length` passed an `Interval`
with an infinite upper end. A collapsing moved circle refuses
`MovedCircleCollapses` and an infinite blend size `SizeBeyondRange`,
both limits on their own lever. Row 17's open sign moved to SECTTORUS
(`a-torus-parallel-through-a-crossing-states-a-radius-no-row-decides`);
SHELL's `a-moved-rims-radius-is-positive-by-agreement-not-by-a-decision`
is closed by this PR; the finiteness row is ruled by `[sizes]` and
lists the guards its build retires.
