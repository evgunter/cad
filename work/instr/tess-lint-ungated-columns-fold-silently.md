---
id: tess-lint-ungated-columns-fold-silently
kind: issue
title: Six CSV columns reach the gate unparsed and unrefused, and a re-cut folds every ungated column's movement in silently
status: closed
opened: 2026-09-08
closed: 2026-09-17
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
The sizing census — `the_committed_baseline_sizes_this_much`, in
`tools/tess-lint/tests/baseline_census.rs` — reads totals only, so
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

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## What unit 5 closed, and what it did not (2026-09-16)

**Arm 1 is closed.** The constructed case the row asserts was run
first, against the real gate rather than against a fixture: one sized
row of the committed baseline with `muu` set to `banana`, linted as the
fresh file against that same baseline, parsed clean and printed
*"clean — no scene grew and no face's sizing got wastefuller"* at exit
0. `parse` now reads `muu`, `muv`, `mvv`, `mu1` and `mv1` through a
`BOUND_COLUMNS` table under one `Admissible::NonNegative` — finite and
non-negative, which is what a sup of a norm is; `mesh::nurbs_cert`'s
`nurbs_cell_grid` refuses a face whose bound is non-finite in any of
the five, and `tess_meter::split_scan` asserts the sign for the three
Hessian sups — and `cells` through the `usize` read `dev_samples` and
`triangles` take.
The values are DROPPED after they are admitted: no rule reads them, and
storing a number no rule reads is arm 2's defect rather than a cure for
it. The same `banana` row is now harness breakage naming the column, at
exit 1.

Zero is admitted in all six, deliberately and per `CC5`: a ruled
direction's `sup ‖S_uu‖` is zero, and `mu1 = 0` is a 3-D-degenerate
direction `nurbs_cert` handles rather than refuses. For `cells` the
warrant is narrower and is stated as such at the constant — no
producer path that leaves the `Vec` empty was found and the baseline's
minimum is one, so zero is admitted not because it was observed but
because every argument for refusing it runs through the producer's
code, which `CC4` refuses to lean on.

The admissions table was also collapsed: `Certificate`, `Count` and
the `Sup` this unit first added were three spellings of
`finite && >= 0.0`, permutable among themselves with the whole suite
green, and `Target`/`Aspect` were two spellings of
`finite && > 0.0`. They are now `NonNegative(what)` and
`Positive(what)` — one variant per policy, the quantity's name as
data — and every harness message is byte-identical to before.

**The class is closed too, not just the instance.** The six columns
were invisible to the bracket assertions, each of which says its own
block's neighbour is where it expects it to be — exactly as true with
an unread run between two blocks.
`every_header_column_is_claimed_by_exactly_one_site` covers the header
instead, so a column that arrives claimed by nothing fails rather than
folding in.

**Arm 2 is not this unit's and is already scheduled.** `name` has no
in-band value to refuse — an admission is not what it lacks — and what
it lacks instead is a rule that reads it, which is unit 18 (`C15` +
`c15-is-dischargeable-now-that-a-sized-scene-carries-names`, which
re-keys rule 4 over the rows that carry a name). Nothing was filed for
it: a second row would duplicate that unit.

**Arm 3 is filed as `tess-lint-re-cut-folds-uncompared-columns`**, with
the movement table moved there, because what it needs is a report that
does not exist and a decision about who it is for — a gate run and a
re-cut arrive at `tess-lint` identically today.

**What `C15` cites this row for now lives in the crate.** `C15` names
this row as *"the statement of record on which ungated columns reach a
reader"*; that roster is now in `tools/tess-lint`'s module docs, beside
the code, where it survives this program's directory being deleted.
