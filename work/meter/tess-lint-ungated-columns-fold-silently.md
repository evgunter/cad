---
id: tess-lint-ungated-columns-fold-silently
kind: issue
title: A re-cut folds in every movement of the columns no rule reads, and nothing anywhere reports it
status: open
opened: 2026-09-08
---


Found by METER unit 6, by diffing its re-cut against the baseline it
replaced.

**Seven of the CSV's columns are read by no rule**, and a re-cut folds
whatever they now say into the committed file with nothing anywhere
reporting the movement. Unit 6's re-cut moved, on a corpus whose rows
and scenes did not change at all:

| column | rows moved | largest relative move |
|---|---|---|
| `muv` | 15 | 38.10% (`lofts/nonuniform_loft` face 4) |
| `mvv` | 14 | last-digit |
| `mu1` | 12 | last-digit |
| `mv1` | 13 | last-digit |
| `worst_cert` | 16 | last-digit |
| `worst_dev` | 12 | 1.11% (`lily/lily_leaf_a` face 4) |
| `realized_aspect` | 8 | last-digit |

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

Cheapest cure, and it is a report and not a gate: `tess-lint` already
holds both files when `--baseline` is passed, so it can print what
moved in the ungated columns beside the verdict, exactly as it prints
the constraint-activity line. A threshold on a certified bound would
be the wrong shape — the meter never gates on geometry — but a lane
re-cutting the baseline should be told what it is folding in.

Fence: `tools/tess-lint/*`, METER's.
