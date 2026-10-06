# R2 probes for PR #4135 (frozen head 17c7349bb5)

Not wired into the tree. To run, copy each file to the path below and add the module line.

| File | Copy to | Module line |
|---|---|---|
| `probe_r2_conic.rs` | `crates/topo/src/boolean/conic_quadric/` | `#[cfg(test)] mod probe_r2_conic;` in that dir's `mod.rs` |
| `probe_r2_line.rs` | `crates/topo/src/boolean/` | `#[cfg(test)] #[path = "probe_r2_line.rs"] mod probe_r2_line;` at the end of `reduce.rs` |
| `probe_r2_e2e.rs` | `crates/sweep/tests/` | `#[path = "probe_r2_e2e.rs"] mod probe_r2_e2e;` in `all.rs` |
| `probe_r2_diff_door.rs` | `crates/topo/src/boolean/conic_quadric/` | `#[cfg(test)] mod probe_r2_diff_door;`; set `R2_DIFF_OUT` |
| `probe_r2_diff_pis.rs` | `crates/sweep/tests/` | as e2e; set `R2_DIFF_PIS` |

- The `diff` pair runs the same file on the merge base 94512aac and on the head, then compares the outputs with `cmp`.
- `mutants.py ROOT OUTDIR [names…]` applies one mutant at a time to a probe-wired worktree, runs the rows and the probes, and restores the file. It touches the restored file, because without that cargo keeps a stale crate.
- Each probe prints a census (`PROBE … WRONG n`) rather than panicking.
- Oracles: the double cone's residual `ρ cos α − |h| sin α`, read from each point, sampled densely, with golden-section refinement at every sampled extremum and bisection to the bit. The conic probe skips judging a root's position where its own f64 resolution, ~4e-16·scale over the slope, exceeds ε/10, and counts it as `oracle-unresolved`.
