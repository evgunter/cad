---
id: issue555s-skewed-prism-reads-its-skew-through-powi
kind: issue
title: issue555_subfloor_cap builds its skewed prism through f64::powi, whose bits vary with how the test binary is compiled
status: open
opened: 2026-11-23
priority: P4
---


`crates/mesh/tests/issue555_subfloor_cap.rs`, `the_written_zero_keeps_the_rebuild_bit_identical` (and the file's other rows) build the skewed prism through `skewed_prism(10.0f64.powi(exp), …)`. Rust documents `f64::powi`'s precision as non-deterministic, and it is: measured by PR 7526's implementer, the prism's vertex `z` (`2·10⁻³⁰`) is `0x39c4484bfeebc29f` in one build of `mesh`'s test binary and `0x39c4484bfeebc2a0` in another of the same source with two unrelated test functions added to the file, each build stable from run to run. The body differs by an ulp, so its mesh's bytes do; the kernel is not at fault (both bodies mesh, each bit-identically on a rebuild, which is what the row asserts).

**Hazard:** any committed digest or cross-build comparison of these bodies can move with an unrelated edit to the test binary. **Cure:** write the skews as literals (`1e-30`, `1e-45`), or `10.0f64.powf` (correctly rounded on the platforms this repo builds on, but not promised either), and say why in the file.
