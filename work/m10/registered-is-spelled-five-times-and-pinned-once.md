---
id: registered-is-spelled-five-times-and-pinned-once
kind: issue
title: the discharge vocabulary is spelled five times across two crates and only one pair is pinned
status: open
opened: 2026-09-06
---

**Found by M10-9's fix pass** (the registered-identity door, branch
`m10/m10-9-registered-identity`), filed as a CLASS item: nothing here
is wrong today, and the shape is the kind that goes wrong quietly.

A discharge outcome is spelled **five** times, in two crates:

| spelling | where | what it is |
| --- | --- | --- |
| `SymCounts::registered` | `crates/geom-core/src/sym.rs` | the session receipt's column |
| `Discharge::Registered` | `crates/geom-core/src/sym.rs` | how the tier answered |
| `ShapeOutcome::Registered` | `crates/geom-core/src/sym/report.rs` | the shape report's row |
| `SampleOutcome::Registered` | `crates/geom-core/src/k_stats.rs` | the K sample's token |
| `Scan::registered` | `tools/k-lint/src/lib.rs` | the linted CSV column |

They are five different types by design — a count, a discharge reason,
a report row, a K token and a lint column are not one concept — but
they must MOVE TOGETHER: a sixth discharge kind that reaches four of
the five produces a receipt that adds up and a K corpus that silently
drops a population.

**Exactly one pair is pinned.** `tools/k-lint/tests/outcome_vocabulary.rs`
holds `SampleOutcome::ALL`'s tokens against `k_lint::ACCEPTED_OUTCOMES`
across the workspace boundary, which is the one seam a `cargo` build
cannot see for itself. The other three are held by nothing: adding a
`Discharge` variant without a `SymCounts` column, or a `ShapeOutcome`
row without a K token, compiles.

M10-9 added the fifth spelling and felt the shape — the door's own
five edits were made by hand, in five files, and the order they were
made in is why one of them (`SampleOutcome`) is pinned and the rest are
not.

## What is owed

A decision on which of three shapes this becomes, taken by whichever
unit next adds a discharge kind:

1. **One enum, projected.** A single `Discharge` with `From` impls into
   the other four, so the compiler's exhaustiveness check is the pin.
   Costs: `k_stats` would depend on `sym`, which today it does not.
2. **A pin per seam.** Three more rows in the shape of
   `outcome_vocabulary.rs`, each holding one projection total. Cheap,
   and it leaves five spellings that a reader must still line up by
   hand.
3. **Leave it, and say so.** Five is small and the seams are named
   here. This is the honest option only while the list is written down
   somewhere a person adding a sixth will read — which is what this row
   is for.

Not urgent, and not M10-9's to take: the door's own five spellings are
consistent and measured
(`editor-core/tests/m10_9_pins_interval.rs`, the K row in
`docs/K-REPORT.md`).
