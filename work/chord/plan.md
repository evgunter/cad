# CHORD — the plan

The tessellator's arithmetic and the certificates over it.

Opened 2026-09-20 by TESS's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

**34.5 budget points** against a ceiling of 30 — a little over, and
the overage is deliberate: `S28` (one shared core for the three
tessellation lanes) is the row every other row here gets easier
behind, so cutting it out to make the arithmetic fit would leave both
halves worse.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `D304` | D | walk.rs's rim-anchored out.first() unreachable!; ask whether the polygon can stop being a bare Vec |
| P1 | `S28` | D | Give mesh's three tessellation lanes a shared core; the duplication half after #648/#674 |
| P1 | `approx-face-mesh-certifies-against-fit` | H | mesh + props - widen an Approx face's tolerance by its certificate's bound, so the mesh certifies against the DESCRIPTION |
| P1 | `chart-azimuth-and-bbox-anchor-idioms` | E | Two three-line idioms with no home - chart-frame azimuth of a direction, and the bbox-centre conditioning anchor |
| P1 | `chord-count-arithmetic-is-plain-f64-across-every-speed-arm` | H | Every chord-count arm multiplies a certified sup by a span in plain f64 and ceils it, with next_up on the sup as the only outward pad |
| P1 | `chords-spells-the-half-edge-to-face-walk-twice` | E | mesh/src/chords spells the half-edge to face walk twice |
| P1 | `degenerate-triangle-normal-is-substituted` | D | A degenerate triangle's substituted +Z normal is argued locally and unowned upstream |
| P1 | `tessellation-chart-frame-is-a-hand-rolled-undecided-triple` | E | planar.rs chart_frame builds the whole tessellation triple by hand, with no length decided |
| P3 | `C23` | D | Decide whether RATIONAL_CERT_SPLITS and geom's RATIONAL_METER_SPLITS are one schedule at 16, then sync or separate |
| P3 | `D300` | D | Give nurbs_cert.rs's hessian-hull domination row a measured ceiling and anti-vacuity floor at more than one epsilon |
| P3 | `S236` | D | Give cert_cylinder an empirical falsifier; the FaceMeasure contract half belongs to Track K |
| P3 | `S237` | E | Add a measured floor beside the worst_ratio ceiling at its three live instances in mesh |
| P3 | `dist-line-triangle-takes-an-unchecked-unit-direction` | E | mesh/cert.rs dist_line_triangle takes a unit direction it does not check — the surface's stored axis, so no caller holds the witness yet |
| P3 | `memo-dumps-hide-the-closed-bit-the-counters-depend-on` | E | PatchMemo's and PickMemo's Debug print hit/miss counters without the closed bit that says which picture the counters describe |
| P3 | `mesh-materialized-form-is-writable` | D | mesh::Mesh (with FacePatch and BoundaryPolyline) has all-public fields — any downstream crate can hand-build a mesh whose watertightness contract is false |
| P4 | `degenerate-normal-rows-model-resolution-cites-a-deleted-helper` | E | the degenerate-normal row's model resolution cites datums::unit, which no longer exists |

## Order

`S28` first — the three tessellation lanes' shared core. It is the
entrenching row (each new lane makes the collapse harder) and the
chord-count and chart-frame rows behind it are all easier once there
is one place to put them.

Then `chord-count-arithmetic-is-plain-f64-across-every-speed-arm`,
which is the substantive claim of this program: a certified sup
multiplied by a span in plain `f64` and ceiled, with `next_up` as the
only pad, is an uncertified number wearing a certificate's name.

The measured-floor rows (`D300`, `S236`, `S237`) are TINT's class on
this ground — a ceiling with no floor beside it cannot tell a bound
that holds from one that went vacuous — and batch into one unit rather
than three PRs.

## Review posture

OPEN, for this program's first dispatch. TESS inherited S-MESH's full
v6 dual on kernel units; protocol v7 (`docs/MODEL-AB-LOG.md`, Ev
2026-09-19) runs the dual on triaged-in units only. This program's
ground is arithmetic whose failure mode is a confident wrong number,
which is the case v7 says to triage IN — the first orchestrator
confirms that rather than inheriting it.
