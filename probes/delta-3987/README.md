# Delta-3987 probes: how to mount them

- `delta3987_probes.rs` is in-crate. Copy it to `crates/topo/src/boolean/delta3987_probes.rs` and add the following beside `mod offer_rows;` in `crates/topo/src/boolean/refusal_routes.rs`:
  `#[cfg(test)] #[path = "delta3987_probes.rs"] mod delta3987_probes;`
  Run it with `cargo nextest run -p topo --lib -E 'test(delta3987)' --no-capture`.
- `delta3987_r2_probes.rs` is reviewer B's file, verbatim. Copy it to `crates/topo/tests/` and declare it in `tests/all.rs` with a `#[path]` line.
- `delta3987_mutants.py <name>` applies one mutant to the worktree, runs the four suites under `ci`, prints the red rows, and restores the files. The names are MA_no_dual_orientation, MB_no_structural_gate, MC_reduce_takes_body, M3p_sort_only_at_fallback_sites and M2_mains_gate. Set ROOT and the target dir first.
- `delta3987_ksweep.sh <tree> <outdir> <target>` runs the nightly dev-probe dumps at 1e-6 and 1e-9, then `tools/k-lint`. The two `k-lint-*.txt` files are its outputs, with paths stripped.
