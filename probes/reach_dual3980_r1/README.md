# Probes for the reach-dual3980-r1 review of PR #3980

- `reach_dual3980_r1_probes.rs`: copy into `crates/sweep/tests/` and add
  `#[path = "reach_dual3980_r1_probes.rs"] mod reach_dual3980_r1_probes;` to
  `crates/sweep/tests/all.rs`. Run with
  `cargo test -p sweep --release --test all reach_dual3980_r1_probes -- --nocapture`
  under `CAD_TOLERANCE_EPS` 1e-9 / 1e-6 / 1e-12. Every op prints a `PROBE` line;
  the main-vs-head differential diffs those lines with `ops.rs` at `origin/main`.
  `claim1_widened_rest_matrix` stays red on head only for full-turn blind/half-in
  UNIONS (`RestZipUnsupported`, already filed) and, at 1e-6, `point_in_solid`
  escalations at the ×1e-3 scale; it has no wrong answer at any ε.
- `mutants/M*.diff`: patches against head's `crates/topo/src/boolean/ops.rs`;
  `mutants/run.sh` is the suite each was run under (topo + sweep + editor-core
  `ci` profile, plus the PR's slow `rest_mate_every_op` rows and these probes).
