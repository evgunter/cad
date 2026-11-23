---
id: a-spheroid-is-a-surface-kind-that-nothing-mints-yet
kind: unit
title: Surface::Spheroid lands with its chart, residual, normal and poles, every other arm refusing typed, and nothing minting it
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
---



Unit 1 of `an-ovoid-cannot-be-authored` (Ev, PR 7289), part 2's
vocabulary. The ruled text is `docs/DESIGN.md` `[surfaces]`: "A spheroid
is a sphere stretched along one axis, with an equatorial and a polar
radius that differ past the band ... a sphere stays its own kind", and
the locus sentence (a kind's chart, implicit residual and normal agree
at every point, and its periodic coordinates and singular points are
fixed by the kind, with no tolerance). The precedent is
`Surface::Spindle` stage 1 (PR 6473), whose unit body
(`git show 3faf8a8ef8:work/spiricarm/the-spindle-sheet-away-from-its-apexes.md`)
this unit follows, minus the mint.

## What is true when it is done

- `Surface::Spheroid { center, axis, u_ref, equatorial: a, polar: c }`
  and `SurfaceKind::Spheroid` exist (`crates/geom/src/surfaces.rs`,
  `Surface`, `SurfaceKind`, `kind()`, `name()`, `axis_line()`, the frame
  accessors). One deciding constructor refuses typed where `a > 0`,
  `c > 0` or `a − c` clear of the band is not decided (three named
  margins through `k_stats::decide`), so a spheroid within the band of a
  sphere is unrepresentable. The fields are authored data through
  `sqrt` at most, no transcendental.
- The geom core is the sphere's chart composed with the stretch: the
  chart point, jet, inverse and normal (`crates/geom/src/chart_point.rs`,
  `ChartArm`), the implicit residual (`crates/geom-brep/src/implicit.rs`),
  `scalar_lift` and the cover readings (`crates/geom/src/cover/readings.rs`).
  A test holds chart, residual and normal against each other at sampled
  points of a prolate and an oblate spheroid, at every lane scalar.
- `chart_singularity::singular_points` lists the two poles
  (`crates/geom-brep/src/chart_singularity.rs`, a `sphere_poles`-shaped
  helper at the polar radius). `u` is periodic and `v` is not.
- Every other arm refuses typed, naming that a spheroid has no lane
  yet, in the spelling each site already uses for the spindle sheet:
  `PairRoute { implemented: false, note }` in `geom_brep::intersect::route`
  and its section gate; `curve_face`'s `unbuilt(...)`;
  `PcurveCertifyError::UnsupportedChart`; an `OffsetError` variant ending
  `NOT_YET_ENDING` (as `OffsetError::SpindleSheet` does), surfaced by
  editor-core's `verbs/shell.rs` mapping; `EngineRefusal::Unsupported`
  in `topo::injective`; `topo::clearance`'s `unsupported(..)`;
  `StepExportError::UnsupportedSurface`; `Interval::refused()` in
  `ssi/enclose.rs`; `None` where a reader answers "no closed form"
  (`tangent.rs`, `face_normal.rs`, `blend/reach.rs`, `replace_face.rs`).
  Each new refusal variant declares its standing row 2 (valid,
  unbuilt) where its type has a standing method. ONECOPY's
  `row-2-is-a-typed-standing-in-code` (open) will move these to
  `Refusal::Unbuilt`; this unit uses today's spelling.
- `pncad-py`'s `SurfaceKind` mirror (`crates/pncad-py/src/py/select.rs`)
  and `pncad.pyi` gain `Spheroid`; editor-core's two arms
  (`eval/measure.rs`, `eval/wire/pose.rs`) and the tour's `uvdump.rs`
  and `tools/tess-meter` match it.
- Nothing mints a spheroid: no door outside `geom`'s constructor builds
  one, and a grep shows no caller of the constructor outside tests.

## Size (counted 2026-11-22)

The most recent kind, `Spindle`, is spelled on about 223 non-test
source lines in about 80 files: geom-brep 83, topo 62, geom 39, mesh 21,
sweep 6, editor-core 4, pncad-py 3, step-import 2, step-export 1,
tools 1, demos 1. The heaviest files are `geom/src/surfaces.rs` (22),
`geom-brep/src/pcurve_cache.rs` (20), `pcurve_cache/projected.rs` (12),
`geom/src/chart_point.rs` (8), `geom-brep/src/implicit.rs` (7),
`topo/src/validate.rs` (6), `geom-brep/src/props/curved.rs` (6).
`Sphere` is spelled on about 527 source lines in 181 files (inline test
modules included). Adding a variant is atomic across every crate that
matches `Surface` exhaustively, so this unit is one PR of roughly
Spindle stage 1's width less its revolve arm (PR 6473: 98 files,
+1813 −199).

## Phase 0: the census, reported before code

1. Every exhaustive `match` on `Surface` or `SurfaceKind`, grouped by
   what a `Spheroid` arm needs: agnostic (datum moves, transforms,
   frame and axis accessors); **affine-invariant readings the sphere's
   arm composes with the stretch** (chart eval and inverse, residual
   sign, incidence, the poles); **metric readings that do not compose**
   (distance and clearance, curvature, speed bounds in the pcurve
   envelope lemma, offsets, mesh sizing), which refuse typed here; and
   the pair tables, which refuse typed here.
2. Every non-exhaustive kind test the compiler will not list:
   `matches!(.. Surface::Sphere ..)`, `if let Surface::Sphere`,
   `== SurfaceKind::Sphere`, and `_ =>` arms in a match that names
   `Sphere`. A spheroid falls silently into each. State the grep shapes
   used and what they cannot match (implementer discipline §5).
3. Then build.

## Check first

The designers' "the sphere's chart composed with the stretch" is
claimed, not traced. Composing is sound only for readings an affine map
preserves; the sphere's `v` is a latitude whose speed is `r`, and a
spheroid's `v` is the parametric latitude whose speed varies between
`a` and `c`. Before writing arms, find which sphere readers lean on a
constant speed or an isometric chart (the pcurve envelope lemma's
speed terms are the likeliest). If the census shows the chart core
cannot be stated without a new chart arm shape in `ChartArm` that the
pcurve and mesh layers would also have to learn in this PR, **stop and
report** with the count: the unit may need splitting, and that is the
orchestrator's call.

## Leaves for later

- The mint and the readers a revolved body needs at rest:
  `the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis`.
- Tessellation, measure, STEP: `a-spheroid-face-tessellates-within-its-own-sizing-theorem`,
  `a-spheroid-face-measures-its-closed-form-volume-and-area`,
  `a-spheroid-writes-exact-step-as-a-surface-of-revolution`.
- Plane × spheroid: `a-plane-cuts-a-spheroid-by-pulling-the-plane-back-through-the-stretch`.
- Every other curved pair, blends, offset and shell stay refused (the
  ruling's parity with the sphere at first); no row is owed for them
  until a consumer asks.

## Rows and ground

- Changes no other row. Relies on `[surfaces]`'s locus sentence for
  analytic kinds (the poles are the kind's), not on its NURBS
  collapsed-row sentence.
- Crosses nearly every kernel program: GERM, SECT and TANG
  (`intersect.rs`), PCERT (`pcurve_cache.rs`), SHELL (`offset.rs`),
  NEARPOINT and SPIRICARM (`props/curved.rs`), TESSLANE
  (`mesh/src/chart_table.rs`), EXPORT (`step-export/src/writer.rs`),
  CHARTTAIL (`topo/src/chart_region.rs`), RESTFRONT and ONEDOOR
  (`topo/src/validate.rs`), PYDOOR and BIND (`pncad-py`). Run
  `python3 scripts/work.py territory --base origin/main` and announce
  on each log the PR body names; no behaviour changes for any existing
  kind, so the announcement is a notice, not a coupling.
- Runs beside `a-profile-segment-is-an-ellipse-arc` and
  `measure-what-an-exact-nurbs-wall-of-revolution-does-today`; no
  shared file of substance.
- Review: **dual**. A new surface kind is an architectural decision
  that every later lane builds on, and the census's classification is
  the substance. Difficulty **H**.
