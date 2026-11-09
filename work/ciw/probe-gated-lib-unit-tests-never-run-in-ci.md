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

**A second instance went red unseen** (`onecopy/curve-pair-tidy`, PR 6541, 2026-11-06). `geom_brep::curve_pair::span_rows::a_point_on_a_circles_axis_is_off_with_no_place_read` pinned the foot of a point on a circle's axis at the seed's antipode, `atan2(+0, −0) = π`. The angle reads (`geom::Azimuth::near`, which reads a zero direction as its reference, merged with PR 6482) moved that foot to the seed itself. The pin went red on main and stayed so: the test is `#[cfg(feature = "probe")]`, and nothing ran it. PR 6541 re-pins it. Every other probe-gated `geom-brep` lib test passes at that PR's head (1220/1220, `--features probe --lib`).

**The integration suites are not run either.** The `dev-probe` step runs `cargo test -p <crate> --features <feats> --test all -- --list`, which lists the tests and executes none. `sweep`'s `ray_wall_margin_twins` (`#![cfg(feature = "probe")]`, in `--test all`) is red on main 230be11d unseen; it is filed as `work/onecopy/the-ray-wall-twins-read-the-angle-predicates-off-scale.md`.

**The integration-suite paragraph above is out of date (2026-11-06, `onecopy/twins-dimension`).** The nightly `dev-probe` row also runs `scripts/k_probe_sweep.sh`. That script executes every suite `scripts/gates/probe-suite-census.sh` rosters in `RUN_FLOOR`, at ε 1e-9, and the census refuses a probe suite that is neither rostered nor declares itself unrun in its own header. `sweep`'s `ray_wall_margin_twins` is rostered (`plain:sweep:ray_wall_margin_twins:2`). The `--all-features` nightly also runs it at the default ε. What this row owes is the lib unit tests only.
