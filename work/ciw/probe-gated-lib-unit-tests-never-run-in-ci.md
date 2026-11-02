---
id: probe-gated-lib-unit-tests-never-run-in-ci
kind: issue
title: #[cfg(feature = "probe")] lib unit tests compile in the nightly dev-probe row and never execute
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [6333]
---

Found by the delta review of PR 6333 (ONEHOME, 2026-11-02).

The only probe lane, nightly `dev-probe` (`.github/workflows/nightly.yml`), runs `cargo test -p <crate> --features probe --all-targets --no-run`, then executes only `--test all`. A `#[cfg(feature = "probe")]` unit test in a crate's lib, such as `geom_brep::ladder`'s `the_sample_stream_interleaves_in_rung_order` (which pins a recording order a regression would break), therefore compiles every night and never runs. It goes red only for someone who runs it by hand. This is a class, not one test: every probe-gated lib unit test. The fix is the probe row executing the lib tests too, or a gate refusing probe-gated lib tests. A guard that cannot go red is P3 (`work/README.md`, Priority).
