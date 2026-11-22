# Reviewer probes for PR 7318 (embed/vertex-bisector at 91f269c5c)

Not for merging.

- `crates/topo/src/injective.rs`: `rv_mut` runtime mutant switches (`RV_MUT=M1..M13`,
  each defined at its site) and three `rv_` rows in `presentation_rows`.
- `crates/sweep/tests/self_intersection_proves_injectivity.rs`: `rv_a_deep_keels_joint_far_out_and_mirrored`.
- `rv_mut.sh <mutant>...`: the `injective::` unit set and the suite file under each mutant.
- `measure-branch-probes.diff`: applies to `origin/embed/vertex-bisector-measure` (5e10ad13a):
  `CV_X` (crossing dump), `CV_ZW` (the no-width quadrant probe), `CV_WHOLE_JET`
  (the wedge read off the whole rectangle's jet); `run.sh`, `summ.py`, `pop-*.filter` drive it.
