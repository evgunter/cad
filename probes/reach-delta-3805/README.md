# PR 3805 delta-review probes

- `fuzz_3805.rs` — in-crate probe. Include it with
  `#[cfg(test)] #[path = "../../../../probes/reach-delta-3805/fuzz_3805.rs"] mod fuzz_3805;`
  after `mod zip;` in `crates/topo/src/boolean/mod.rs`, then
  `cargo test -p topo --lib fuzz_3805 --release -- --nocapture`
  (`CAD_FUZZ_SEED=0x57ca117cf138ede6` replays the numbers in `review.md`).
  Oracle: the TRUE Euclidean distance, densely sampled, its sign changes bisected,
  its signed extrema golden-sectioned (so a root pair between samples is counted).
- `mp_check.py` — 50-digit recheck of the km-scale certified Miss (needs `mpmath`).
- `mutants.py` — re-applies each mutant, runs topo's ci profile plus the slow
  `ellipse_roots::fuzz_rows`, restores the file.
