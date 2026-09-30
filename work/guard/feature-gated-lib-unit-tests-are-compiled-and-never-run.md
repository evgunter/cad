---
id: feature-gated-lib-unit-tests-are-compiled-and-never-run
kind: issue
title: a probe-gated #[test] inside a library's src is compiled by CI and executed by no hosted job
status: open
opened: 2026-09-21
---


**Found by DECIDE-2** (the discharge vocabulary's seam pins), while
choosing where a `probe`-gated pin could live and actually gate.

## What

A `#[test]` inside a library's `src/`, carrying
`#[cfg(feature = "probe")]`, is **compiled** by CI and **run** by
nothing:

- the twelve `test (…)` jobs run `cargo nextest run --archive-file
  nextest-{default,interval}.tar.zst` — archives built at default
  features and at `interval`. Neither selection enables `probe`, so a
  probe-gated lib test is not in either archive.
- `k-lint (gate, dev-probe)`'s `compile and list every probe-gated
  test target` step runs `cargo test -p "$c" --features "$feats"
  --all-targets --no-run`, which COMPILES the lib target's test
  harness and runs nothing, and then `--test all -- --list`, which
  lists the INTEGRATION binary only.
- `scripts/k_probe_sweep.sh` invokes `cargo test -p "$pkg" --features
  "$feats" --test all` at both of its call sites (the ε loop and the
  default selection). `--test all` selects one target: the aggregated
  integration binary. The lib's own unit tests are never a target of
  any invocation.
- `scripts/gates/probe-suite-census.sh`'s census predicate is
  `^#!\[cfg\(.*feature = "probe"` over `crates/$c/tests/*.rs`, so the
  library's `src/` is outside what it can see at all, and `RUN_FLOOR`
  rosters `crate:module` pairs of integration suites.

So the row reports the same green whether it passed or never executed
— the defect `docs/prompts/implementer-discipline.md` names for a
demoted row, one level further in.

## The hit list

Swept with: every `#[test]` in `crates/*/src/**` and `tools/*/src/**`
whose four preceding lines carry `feature = "probe"` or
`feature = "budget"`. Six hits, all `probe`:

| test | file |
| --- | --- |
| `all_lists_every_variant_and_every_token_is_distinct` | `crates/geom-core/src/k_stats.rs` |
| `a_detached_run_splices_its_samples_into_the_callers_sink` | `crates/geom-core/src/k_stats.rs` |
| `a_detached_run_records_samples_with_no_outer_sink` | `crates/geom-core/src/k_stats.rs` |
| `probe_gates_run_the_doors` | `crates/topo/src/props.rs` |
| `the_probe_seam_reaches_the_same_refusal` | `crates/topo/src/validate.rs` |
| `the_probe_has_no_door` | `crates/topo/src/validate.rs` |

The first of those is the completeness pin for
`k_stats::SampleOutcome::ALL` — the roster
`k-lint`'s `tests/outcome_vocabulary.rs` enumerates. Its own doc says
"`tests::all_lists_every_variant` matches on each variant to prove the
list is complete, so adding one without listing it here reds a test";
on this tree no hosted job runs it.

**What the sweep could not match:** a gate written more than four
lines above its `#[test]`; a `#[cfg(all(test, feature = …))]` on an
enclosing `mod` (none exists — `grep` for a feature gate on a `mod`
line in `crates/*/src` and `tools/*/src` returns nothing, and no
`src` file carries a file-level `#![cfg(…feature = "probe"…)]`); a
feature other than `probe` or `budget`; and a cargo root outside
`crates/` and `tools/`.

## Why it is filed here and not fixed here

The fix is a CI/gate change on `guard`'s and `ciw`'s ground —
either a hosted invocation that runs the lib target under the
gating feature, or a census arm that refuses a feature-gated
`#[test]` in `src/` the way the existing one refuses a probe suite
nothing runs (`probe-suite-census.sh`, the `$want_marker`
disposition). DECIDE-2 worked around it instead: its third seam pin
is an integration row in a rostered suite, and
`geom_core::sym::discharge_sample_outcomes` exists to let it be one.
