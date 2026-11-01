# KNOT leaves the tracker — 2026-10-26

KNOT (the spline, net and fit doors: refinement inside an enclosure,
the net-state reads, and the NURBS and fit doors that refuse for
reasons that are not geometric) was opened on 2026-10-10 at FLUX's
priority-seam cut. Nothing in it had been dispatched; NURBS's lanes
closed one of its rows from outside
(`surface-knot-insertion-writes-weights-validate-counts-refuses`,
already fixed by PR 4923).

It shared `crates/geom-core/src/spline/*` with NURBS, whose charter is
the same doors, so the 2026-10-26 tracker sweep merged KNOT into NURBS.
`f64-refinement-inside-an-enclosure-has-five-more-sites` closed at the
sweep: its last site, `mesh/src/chords.rs`'s `rational_carrier_m_bound`,
no longer refines. Every row moved to `work/nurbs/` by `mv`, ids kept,
the two closed ones included:
- `nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` (P2);
- `project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry` (P2);
- `interpolate-with-params-passes-a-non-finite-point-through-as-nan` (P3);
- `net-state-reads-an-infinite-net-as-described` (P3, design);
- `nurbs-curve-has-no-net-state-door` (P3);
- `nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t` (P3);
- `two-reversal-policies-for-a-spline-net` (P3);
- `net-refinement-copies-the-differencing-skeleton-and-the-plan-ratio-is-optional` (P4);
- `the-approximation-refit-solves-banded-normal-equations-densely` (P4);
- `unordered-fit-parameters-refuse-as-a-count-mismatch` (P4).

Its plan set no `## Exit criteria`, so it closes without an exit walk.

Recover SHA `bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2`.

    git show bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2:work/knot/<FILE>
