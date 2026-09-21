---
id: k-probe-sweep-says-no-test-compares-probe-against-f64
kind: issue
title: k_probe_sweep.sh says no test in this tree compares Probe against f64; one now does
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What

`scripts/k_probe_sweep.sh`'s `THE DEFAULT SELECTION` comment block
(above `PLAIN_EPS`) states, of the plain probe selection:

> WHAT THESE TESTS ASSERT, stated because it is what decides the
> placement: one-sided GREENNESS at `Probe` — `failures(&ev).is_empty()`
> — and not bit-identity against an f64 run, **which no test in this
> tree compares**.

The emphasised clause is false as of SUITE's D114:
`crates/editor-core/tests/m4_pr8_k_probe.rs`'s
`probe_agrees_with_f64_bit_for_bit_over_the_corpus` compares the two
arithmetics' evaluations bit for bit — per-node value channel, mass
properties, content keys, name tables, verdict and escalation logs —
over the whole Band 4 corpus, at every ε row `run_dump` sweeps.

The sentence was accurate when written and is the kind that goes false
silently: nothing reads it. **The clause about the PLAIN selection is
still true** — the differential is `#[ignore]`d, so it is the
`--ignored` half that runs it, and what the plain selection asserts is
still one-sided greenness. Only the parenthetical about the tree is
wrong.

## Why CIW's

`scripts/k_probe_sweep.sh` is CIW's ground
(`scripts/work.py territory --files -`: *owned by ciw*). D114's fence
keeps SUITE out of it, so the sentence is reported rather than edited.

## Fix

Re-word the clause to name the differential, e.g. *"…and not
bit-identity against an f64 run, which the `--ignored` half's
`probe_agrees_with_f64_bit_for_bit_over_the_corpus` compares and this
selection does not."* No behaviour changes.

Filed by SUITE/D114.

## A second site, same sentence, same owner

`.github/workflows/ci.yml`'s probe-sweep step carries prose of the same
shape (around the `K-telemetry probe sweep` / probe test-surface
comment block). `.github/workflows/*` is CIW's too, so it is added here
rather than opened as a second row.

## And a scope note the re-wording should not repeat

The clause is false about this tree in a second way that predates
D114: `crates/profile/tests/review_m2_pr2_probe.rs` has compared
`Probe` against `f64` bit for bit at the canonical-form lane since M2.
So the re-wording should say which LANE the plain selection does not
compare, not that nothing in the tree compares anything.

