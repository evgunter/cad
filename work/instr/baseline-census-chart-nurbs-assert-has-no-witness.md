---
id: baseline-census-chart-nurbs-assert-has-no-witness
kind: issue
title: baseline_census's chart-is-nurbs assert can only red on a tag no committed blob carries
status: open
opened: 2026-09-16
priority: P3
cost: D
---



## Finding

In `tools/tess-lint/tests/baseline_census.rs`, inside
`five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows`:

```rust
for r in &sized {
    let n = r.nurbs.expect("filtered to sized rows");
    assert_eq!(r.chart, "nurbs", "{} face {}", r.scene, r.face);
```

`sized` is filtered on `Row::is_sized`, and `parse` admits the sizing
block only under `SIZED_CHART_TAGS` — `["nurbs", "approx"]`. So the
only reading that reds this assertion is a sized row tagged `approx`.

`tess_lint::SIZED_CHART_TAGS`' own doc records that no such row can be
produced from the corpus: *"**`approx` has no corpus witness.** No row
of any committed blob of
`docs/tess-budget-data/tess-budget-baseline.csv` carries that tag"*.

**Failable in principle, with no witness reachable without a producer
change.** That is one step short of the class INSTR unit 4 closed —
an assertion no bug could break — and it is the nearest neighbour to
it in this file. Unlike the deleted partition assert it is not forced
by its own construction, so deleting it is not obviously the repair;
what it owes is either a fixture that supplies the missing witness, or
the disclosure this file gives at three other sites for exactly this
("undiscriminating against a change in the DATA, and it is said rather
than hidden").

## Was

Disclosed by INSTR unit 4's style review (`instr/u4-census-partition-assert`).
