---
id: step-import-runs-no-recourse-roster-over-its-band-funnels
kind: issue
title: step-import decides predicate names through its FileBand funnels and runs no recourse roster: the funnels want *_as spellings and the roster its carriers
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Filed by GREEN (2026-11-21) while greening `work/green/main-red-predicate-name-coverage-step-import-decides-names-and-runs-no-roster.md`, which lists step-import in `KNOWN_MISSING` (`crates/test-utils/tests/predicate_name_coverage.rs`) citing this row.

## What

Since PR 7091 (`importgate/interp-layer`) step-import decides predicate names: `FileBand::read` in `crates/step-import/src/interpret.rs` calls `k_stats::decide(name, …)` on the name its caller hands it, which is the crate's first decide site (`git grep 'decide[a-z_]*('` on the merge's first parent `1490c99c^1` finds none under `crates/step-import/src`). Later merges added const-named sites (`decide(STEP_BUILT_SENSE, …)` and `decide(STEP_AT_DOMAIN_END, …)` in `build.rs`; `STEP_SNAP_RELATION` and `STEP_SNAP_SIDE` in `snap.rs`). So the tree-wide census (`predicate_name_coverage`) reads step-import as a deciding crate, and the crate runs no `tests/recourse_roster.rs`.

## Why GREEN did not write the roster

The names reach `decide` through a family of `FileBand` funnels, and most of them are spelled with tokens other calls share. `NameCarrier::Call` matches by token, so declaring one reads every other call that shares it:

- `read(name, gap)` in `interpret.rs`: 39 call sites hand it a `STEP_*` const, but `read(` also appears in `build.rs` (`read(file.band())`) and `normalize/singularity.rs` (`span.read(s, …)`), where nothing is a name.
- forwarders of it: `agree`, `agree_at` (`interpret.rs`), `on` and `not_apart` (`normalize/singularity.rs`), and the closure `zero` in `build.rs` (`let zero = |name, gap: f64| file.read(name, gap) == Read::Zero`).
- refusal builders that take the reading's name as a label without deciding it: `touching`, `apart_in_band`, `ambiguous` (`interpret.rs`), `in_band`, and `build.rs`' `ambiguous`; and `prove`.

The convention the other rosters follow (`crates/geom/tests/recourse_roster.rs` header) spells a funnel that hands on its caller's name `*_as`, so no other call shares its token, and declares it in `CARRIERS` with a `FORWARDS` line in `INDIRECT`. Doing that here means renaming `FileBand`'s band funnels across about 50 call sites, and deciding which of the label-taking builders are carriers and which are references. That decision belongs to IMPORTGATE, which owns the band layer.

## Owed

`crates/step-import/tests/recourse_roster.rs` now runs (CIW, PR 7496): it declares the `STEP_*` consts and the forwarders `read`, `agree`, `agree_at`, `not_apart` and `on_as` (renamed from `on`, whose token matched `if on {`) as carriers, and the harness reds any forwarder of a parameter that is not declared. `KNOWN_MISSING` is gone: the coverage row derives which crates must run a roster. What stays IMPORTGATE's: `read` and `agree` are common tokens, so declaring them reads every call that shares them (today none hands a non-name); spelling them `*_as`, and deciding which label-taking refusal builders (`touching`, `apart_in_band`, `ambiguous`, `in_band`, `prove`) are references, is the band layer's call.
