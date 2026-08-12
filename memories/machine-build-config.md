---
name: machine-build-config
description: This box's local cargo build config (line-tables-only only) and the 2026-08-11 measurements — mold and sccache were tried and REVERTED; docs/LOCAL-BUILD-PERF.md is the full report
metadata:
  type: project
---

**Read `docs/LOCAL-BUILD-PERF.md` before touching local build config.**
Three knobs were measured on 2026-08-11; two were reverted the same day.
This memory is the index into that report, not a substitute for it.

**What is set.** `local-scripts/setup-build-env.sh` writes
`~/.cargo/config.toml` with exactly one thing: `debug =
"line-tables-only"` on dev+test. `target/` 4.7 GB -> 1.5 GB (-68%). It
buys NO compile time — a size knob, kept because ~10 lanes each carrying a
`target/` on a 10 GB box is page-cache and disk pressure.
`debug-assertions`/`overflow-checks` unaffected; backtraces keep file:line.

**What was reverted, so it is not re-adopted from first principles:**

* **mold** — 189 s baseline vs 186 s, i.e. noise. Not a contradiction of
  #174's -38%: that was 261 test binaries, and after #179 + #387 this
  workspace has 14. Also note `-fuse-ld=mold` cannot work here (gcc 9.4,
  needs 12.1+); it requires mold's `libexec/mold/ld` shim via
  `-C link-arg=-B<dir>`. Revisit only if test targets approach triple
  digits.
* **sccache** — cold lane build 156 s -> 96 s at 99.4% hits, BUT sccache
  hard-refuses `CARGO_INCREMENTAL` ("incremental compilation is
  prohibited"), so it forces `incremental = false`, which measured **5-7x
  slower on the edit-rebuild loop** (geom-core 91 s vs 18 s; topo 74 s vs
  10 s). Wrong term optimized: sccache saves ~60 s once per LANE,
  incremental saves ~73 s per EDIT. Right tool only if lane churn ever
  dominates.

**The finding that matters most is not a config knob.** The same cold
build measured **69m23s and 3m08s** in two windows — same config, same
tree, 22x apart. That environmental term dwarfs every flag. Leading
(UNVERIFIED) hypothesis: express-lane jobs overlapping the main-slot build
pushing a 10 GB box into swap, where the penalty is nonlinear rather than
#230's ~40%. Needs its own #230-style measurement — see
[[agent-lane-operations]] and the report's §1. **Investigate this before
compiler flags.**

**What did help:** fewer test binaries. #387 collapsed step-import's 26
`[[test]]` targets to 1 (workspace 39 -> 14), CI build step 148 s -> 108 s.
Gated by `scripts/check-test-aggregation.sh` (one `[[test]]` per member)
plus each crate's `every_suite_file_is_aggregated`; both halves are needed
and both have already caught real regressions.

**Agent-facing consequence:** `cargo test -p <crate> --test <suite>` no
longer resolves — there is one binary `all` and suites are module
prefixes. Use `--test all <suite>::`.

Changing any of these re-fingerprints every lane, so each pays one cold
rebuild (~156 s) on its next build. See also [[agent-lane-operations]] for
the slot machinery and its fd-inheritance traps.
