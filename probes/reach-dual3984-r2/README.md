# Probes, review reach-dual3984-r2 (PR #3984 at 8abb6e7931)

- `probe_3984_r2.rs`: a `sweep` integration module. To run it, copy it into
  `crates/sweep/tests/`, add `#[path = "probe_3984_r2.rs"] mod probe_3984_r2;`
  to `crates/sweep/tests/all.rs`, then
  `cargo nextest run -p sweep -E 'test(probe_3984_r2)' --no-capture`.
  It goes through public doors only. To diff against main, run the same module on
  a merge-base (3cb7bc807) worktree **with its own CARGO_TARGET_DIR**. A
  shared target dir hands one tree the other tree's test binary.
- `mutants.py`: six source mutants; `FILTER=<nextest expr> PKG=<crate>`.
- No gate-bypassed probe was run: the permission system denied that step,
  so claim 1 rests on the PR's own in-crate rows plus the mutants.
