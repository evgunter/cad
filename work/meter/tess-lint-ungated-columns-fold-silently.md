---
id: tess-lint-ungated-columns-fold-silently
kind: issue
title: Six CSV columns reach the gate unparsed and unrefused, and a re-cut folds every ungated column's movement in silently
status: open
opened: 2026-09-08
---


Found by METER unit 6, by diffing its re-cut against the baseline it
replaced.

**The table below is what MOVED on one re-cut. It is not the class,
and the first version of this row presented it as one.** The class is
larger, and its worst arm is stronger than "read by no rule".

**Arm 1 — six columns `parse` never reads at all** (`tools/tess-lint/src/lib.rs`,
`parse`): `muu`, `muv`, `mvv`, `mu1`, `mv1`, `cells`, the whole block
between `nv` and `grid_cells`. `IDENTITY_MEASURES` covers `u0..nv`,
`SIZING_COLUMNS` covers `grid_cells..worst_dev`, `DEV_SAMPLES` and
`INDICATOR_COLUMNS` cover the rest, and nothing indexes 12-17. The
ONLY check they get is the all-empty/any-empty partition over
`f[IDENTITY_FIRST..]`, so a row carrying `muu=banana` parses clean and
reaches the gate. Nothing even refuses a garbage value — which is a
stronger defect than "no rule reads it", and none of the six is in the
table below except by accident.

**Arm 2 — one column parsed, stored and read by nothing**: `name`,
added by unit 6. `parse` puts `f[NAME]` into `Row::name`, no
`Admissible` polices it, and no rule reads it. (Unit 6's fix pass added
the one assertion that catches the value landing in the wrong field;
before that, nothing in the crate touched it after `parse` wrote it.)

**Arm 3 — parsed, policed, printed, and compared against nothing**:
`worst_cert` (`Admissible::Certificate`), `worst_dev`
(`Admissible::OptionalDeviation`, which reaches the report's `total`
factor) and `realized_aspect` (`Admissible::Aspect`, which reaches the
constraint-activity line). These are NOT ungated in arm 1's sense: a
garbage value is refused as harness breakage, and two of the three
reach a reader. What they lack is a comparison across a re-cut, which
is the fold hazard below and is the weakest of the three arms.

**What one re-cut folded in.** Unit 6's re-cut moved, on a corpus whose
rows and scenes did not change at all:

| column | arm | rows moved | largest relative move |
|---|---|---|---|
| `muv` | 1 | 15 | 38.10% (`lofts/nonuniform_loft` face 4) |
| `mvv` | 1 | 14 | last-digit |
| `mu1` | 1 | 12 | last-digit |
| `mv1` | 1 | 13 | last-digit |
| `worst_cert` | 3 | 16 | last-digit |
| `worst_dev` | 3 | 12 | 1.11% (`lily/lily_leaf_a` face 4) |
| `realized_aspect` | 3 | 8 | last-digit |

`muu` and `cells` did not move on THIS cut, which is why they are
absent from the table and why the table is not the class.

None of those is the unit's doing: every one is `m.<field>` copied
straight from `mesh::budget`'s measurement in `tess_meter::columns`,
and the unit's diff reaches only `opt_cells` and `span_opt_cells`,
which are the two columns computed through the split scan. What moved
them is the kernel, over the 442 commits under `crates/` between the
previous cut (`aba2625f8f84`, 2026-09-04) and this one — `mesh/12` and
`props/budget-faces` among them.

**The hazard is not that they moved; it is that a re-cut is the only
event that ever reads them, and it does not report.** `tess-lint`'s
rules read triangle counts, the slack ratio and the identity columns;
`worst_dev` reaches the report's `total` factor and `realized_aspect`
the constraint-activity line, and neither is compared against
anything. So a certified bound that moves on a face the gate covers is
invisible until somebody re-cuts, and then it is invisible again
because the fold has no diff. This is `docs/TESS-BUDGET.md`'s
*"Coverage restored is not coverage verified"* on a scene the baseline
DOES cover, which that passage does not reach.

**A second consumer is affected and it is not obvious from here.**
`tools/tess-lint/tests/baseline_sizing_census.rs` reads totals only, so
all seven movers above are invisible to it; its "fourth thing a re-cut
can be" says a re-cut whose only movers are `opt_cells` and
`span_opt_cells` is an instrument-resolution change and never a
schedule regression, and the census cannot tell whether "only" holds.
That is stated at the census as of unit 6's fix pass, and it is one of
the things arm 1's cure would settle.

Cheapest cure, and it is a report and not a gate: `tess-lint` already
holds both files when `--baseline` is passed, so it can print what
moved in the ungated columns beside the verdict, exactly as it prints
the constraint-activity line. A threshold on a certified bound would
be the wrong shape — the meter never gates on geometry — but a lane
re-cutting the baseline should be told what it is folding in.

Fence: `tools/tess-lint/*`, METER's.
