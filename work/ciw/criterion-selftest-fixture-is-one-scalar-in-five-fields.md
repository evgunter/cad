---
id: criterion-selftest-fixture-is-one-scalar-in-five-fields
kind: issue
title: criterion-emit.py's selftest passes 11 of 20 single-token mutations - its fixture writes one scalar into five collected fields
status: open
opened: 2026-09-11
refs: [criterion-selftest-nightly-only, 2330]
---


Measured by the unit that promoted `scripts/criterion-emit.py --selftest` into
the per-PR gate, and by the style lane that reviewed it. The row is strictly
better sited than it was; what it COVERS is a fraction, and the fraction is
written here rather than in a sentence at the row.

## The measurement

A battery of 20 single-token mutations of `scripts/criterion-emit.py`, each run
against `--selftest`. On the tree as inherited, **5 died and 15 survived**. The
three repairs this unit took (the `new/`-not-`base/` rule, the roster pin's
single-sided shapes, the cpuinfo parse) bring it to **9 dead, 11 surviving**.

| mutation | after the unit's repairs |
| --- | --- |
| glob loses the `new/` segment | KILLED |
| roster pin demands BOTH sides move (`or` → `and`) | KILLED |
| roster pin disabled outright | KILLED |
| median CI lower bound reads `upper_bound` | KILLED |
| `samples` dropped from the row | KILLED |
| `rustflags` dropped from the block | KILLED |
| cpu flag parse emptied (`flags = []`) | KILLED |
| cpu model: a later socket wins | KILLED |
| unreadable cpuinfo degrades to `[]` rather than `None` | KILLED |
| `median_ns` reads the mean | **survives** |
| `benchmark.json` no longer required | **survives** |
| `full_id` type check deleted | **survives** |
| `mean_ns` dropped from the row | **survives** |
| `median_abs_dev_ns` dropped from the row | **survives** |
| `nproc` dropped from the block | **survives** |
| `mem_total_kb` dropped from the block | **survives** |
| `runner` dropped from the block | **survives** |
| `rustup_toolchain` dropped from the block | **survives** |
| `cargo_profile_overrides` dropped from the block | **survives** |
| `debug_assertions` claim flipped to `True` | **survives** |

## The one defect behind most of the survivors

`selftest()`'s `plant()` writes **one scalar** into five collected fields —
`median.point_estimate`, both confidence bounds (derived from it) and
`mean.point_estimate` all come from the `median` argument — and the assertions
read two of them. So `median_ns` reading the mean is invisible, and so is any
swap between two fields the fixture gave the same value.

The rest are absence rather than confusion: nothing asserts the SHAPE of the
emitted environment block, so seven of its fields can be deleted and eight of
the twenty mutations pass for that reason alone.

**Repairing per field regenerates the blindness at the next field added**, which
is why this is one item and not eleven. Two changes close the class:

1. `plant()` takes five distinct values rather than deriving four from one, and
   the row assertions read all five.
2. The environment block is asserted against its KEY SET, derived from one
   place — the same shape the parity obligation at `criterion-emit.py`'s
   `cpu_identity()` already asks for by hand. A field added without an
   assertion then fails rather than passing silently.

The `debug_assertions` claim is the odd one out and needs its own line: it is
stated rather than detected (`benches/Cargo.toml`'s `[profile.bench]` holds the
decision), so what an assertion can hold is that the emitted value agrees with
the manifest, not that it agrees with the build.

## Why this is not a blocker on the promotion

None of the 20 is a regression the promoting PR introduced, and every one of
them was previously guarded by a row that ran on a schedule, to nobody. The
promotion strictly improves the siting; this item is the coverage, filed
because the rows now describe it.
