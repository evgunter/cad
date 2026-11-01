---
id: registered-is-spelled-five-times-and-pinned-once
kind: issue
title: the discharge vocabulary is spelled five times across two crates and only one pair is pinned
status: closed
opened: 2026-09-06
priority: P1
cost: E
closed: 2026-09-21
---

**Found by M10-9's fix pass** (the registered-identity door, branch
`m10/m10-9-registered-identity`), filed as a CLASS item: nothing here
is wrong today, and the shape is the kind that goes wrong quietly.

A discharge outcome is spelled **five** times, in two crates:

| spelling | where | what it is | pinned by |
| --- | --- | --- | --- |
| `SymCounts::registered` | `crates/geom-core/src/sym.rs` | the session receipt's column | `sym::discharge_pins::every_discharge_kind_increments_one_receipt_column_of_its_own` |
| `Discharge::Registered` | `crates/geom-core/src/sym.rs` | how the tier answered | all four rows — it is the side every seam shares |
| `ShapeOutcome::Registered` | `crates/geom-core/src/sym/report.rs` | the shape report's row | `sym::discharge_pins::every_discharge_kind_records_a_report_row_of_its_own` |
| `SampleOutcome::Registered` | `crates/geom-core/src/k_stats.rs` | the K sample's token | `geom-core`'s `k_stats_doors::every_discharge_kind_retags_its_sample_with_a_token_of_its_own` |
| `Scan::registered` | `tools/k-lint/src/lib.rs` | the linted CSV column | `k-lint`'s `tests/outcome_vocabulary.rs` (pinned before this row) |

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

## Re-homed at M10's exit sweep (2026-09-13)

Here because four of the five spellings are the tier's own (`SymCounts::registered`,
`Discharge::Registered`, `ShapeOutcome::Registered`, and `SampleOutcome::Registered`
in PROPS' `k_stats.rs`). The fifth, `Scan::registered` in `tools/k-lint`, is INSTR's
and is the one pair already pinned across the workspace boundary; a sixth discharge
kind moves all five together, so the row travels with the tier that would mint it.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## CLOSED (2026-09-21, DECIDE-2)

**The ruling: shape 2, a pin per seam** (the orchestrator's,
2026-09-21). Not shape 1 — one enum projected would make `k_stats`
depend on `sym`, a dependency the tier's design keeps absent, and a
`From` impl pins only where the compiler's exhaustiveness reaches,
which it does not across `tools/k-lint`'s workspace boundary. Not
shape 3 — the door's five edits were made by hand and the order they
were made in is why one pair was pinned and three were not, so the
next discharge kind should red somewhere. Five spellings stay five
types by design; what changed is that a sixth kind reaching four of
them cannot compile-and-pass.

The three rows are named in the table above. What they assert is not
totality — a new `Discharge` variant already forces an arm at every
`match` — but INJECTIVITY: each kind reaching a receipt column, a
report row and a K token of its OWN, with the members of the other
side that no discharge reaches named in each row as excluded
(`numeric`, `frozen`, `registrations_refused`,
`registrations_contradicted`; `Definite`, `NumericZero`,
`Indeterminate`, `Invalid`; the definite signs, `Indeterminate`,
`Invalid`). Each was demonstrated to red under a sixth discharge kind
planted in one side of its seam and then reverted; the plant table is
in the unit's PR.

`enum Discharge`'s doc now names the three rows, so a person adding a
sixth kind reads where it has to be spelled.
