# Reviewer probes for PR 7318 (embed/vertex-bisector at 91f269c5c)

Not for merging.

- `crates/topo/src/injective.rs`: `rv_mut` runtime mutant switches (`RV_MUT=M1..M13`,
  each defined at its site) and three `rv_` rows in `presentation_rows`.
- `crates/sweep/tests/self_intersection_proves_injectivity.rs`: `rv_a_deep_keels_joint_far_out_and_mirrored`.
- `rv_mut.sh <mutant>...`: the `injective::` unit set and the suite file under each mutant.
- `measure-branch-probes.diff`: applies to `origin/embed/vertex-bisector-measure` (5e10ad13a):
  `CV_X` (crossing dump), `CV_ZW` (the no-width quadrant probe), `CV_WHOLE_JET`
  (the wedge read off the whole rectangle's jet); `run.sh`, `summ.py`, `pop-*.filter` drive it.

## Delta re-read (fix pass, code head 24f66a1d5)

The branch now holds `origin/embed/vertex-bisector` (45d0e0256) merged in, the probe rows re-inserted,
and the switches that still have a site: `RV_MUT=M1|M3|M4|M5|M7`, plus the selection mutants
`S1` (heading plane for every pair, main's reading), `S2` (no plane where a face is not a quadrant),
`S3` (corner axes read at the corner point), `S4` (heading plane as a fallback where the wedge plane is `None`).
`RV_COUNT=1` prints `RV_TWO_LINE_GATED` per two-line-arm ask with readable headings; `RV_TWO_LINE_UNGATED` prints
where they are unreadable (the asks the arm newly serves).
