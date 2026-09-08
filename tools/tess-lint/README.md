# `tools/tess-lint/` — the reading boundary, and the cross-column rule

What this lint reports and what it gates on is `src/lib.rs`'s module
docs. This page is about the step before either: the boundary where a
CSV becomes rows, and the one rule that governs it in **both**
instruments — `tools/tess-lint` and `tools/k-lint`.

Each instrument polices its input with a private `Admissible` enum and
a table of `(column, Admissible)` pairs, so that the block the parser
polices and the block the header declares can be compared to each
other. That table is **per column**: an entry sees one field of one
row and nothing else. Several of the producer's guarantees are not of
that shape, and the class is large enough that it gets one statement
here and citations from every site rather than a re-derivation at each.

The clauses are `CC1`–`CC5`.

## `CC1` — the boundary, and only the boundary

A cross-column admission is checked in the one function that turns
text into the crate's rows: `tess_lint::parse`, and `k_lint::lint_csv`,
which parses and scans in one pass because k-lint keeps no row type of
its own. **Those are the same place, not two answers**: each is its
crate's reading boundary.

Past the boundary the row type is the contract, and every rule in each
crate is written to be readable on rows that came through it. A
cross-column check placed at a consumer holds for that consumer alone,
and the next consumer inherits a type whose guarantee it cannot see.

## `CC2` — widen the admission when one column is prior

When the invariant is **one column's policy conditioned on another the
boundary has already read and validated**, the discriminant becomes an
argument to `admits` and the table stays the single home of both the
policy and the message.

k-lint's `Admissible::Margin` is the case: a `NaN` margin is the
poison `Decide for f64` reports as `invalid` and drift on any other
outcome, so `admits` takes `outcome`, which `lint_csv` has already
checked against its allow-list.

The cost is that every column's `admits` then sees the discriminant.
It is worth paying when the discriminant is one column, already
validated at that point, and genuinely prior — a policy the other
column's value cannot change.

## `CC3` — check beside the table when neither column is prior

When the invariant is a **relation between columns that each admit
alone**, no entry can own it: neither field is the other's condition.
It is checked once, at the boundary, after both readings are in hand
and before the row is built. Four live instances:

| Relation | Site |
| --- | --- |
| `band_zero < band_escalate` — `Band::new`'s nonempty open interval | `k_lint::lint_csv` |
| `worst_dev`'s `NaN` against `dev_samples` — an unresampled sweep, or a sample that came back `NaN` | `tess_lint::parse` |
| `chart` against the sizing tail — the block is owed by exactly `SIZED_CHART_TAGS` | `tess_lint::parse` |
| `cap_bands`, `snap_bands` against `bands` — each counts a subset of the bands the schedule emitted | `tess_lint::parse` |

The sizing tail's own **all-or-none** presence check is the same
clause over a whole block rather than a pair.

## `CC4` — state the entailment where a per-column policy already refuses it

A cross-column property that **no row surviving the per-column table
can violate** is stated at the column that entails it and checked
nowhere. A second check would be a second home for one rule, which is
what this page exists to prevent.

`Admissible::Extent` is the instance: the trim box's own
non-degeneracy, `u0 < u1` and `v0 < v1`, is beyond what an entry over
one edge can say — and it is entailed by `span_opt_cells` being an
`Admissible::CellCount`. That accumulator skips analysis cells outside
the trim box, so its floor of one is geometric: a degenerate box
overlaps no cell, and a face with a degenerate box has no triangles
and no row at all. The entailment is the load-bearing part, so it is
written out at `Admissible` and at `Extent`, where a reader tempted to
add the check will meet it.

## `CC5` — the voice is the harness voice, and the test is not "cross-column"

Every clause above exits in the **harness voice** — `tess-lint`'s
`EXIT_HARNESS`, the same exit a renamed column or an unknown chart tag
gets. A cross-column admission failure is the producer and the
consumer disagreeing about what the file says, so what broke is the
reading, not the geometry.

The test that decides this is not "does the check span columns". It is
the one `tess_lint::Report` already states: **would the gate otherwise
assert something false about a MEASUREMENT.** Two consequences a
reader hitting a new invariant needs:

- **An admission refuses what the producer could not have written; it
  never refuses what the instrument exists to measure.** `cap_bands`
  counts a subset of `bands`, so a larger `cap_bands` is arithmetic
  that did not happen. The optimality relations look similar and are
  not: `grid_cells` against `span_opt_cells` is the `split` ratio the
  report is FOR, its denominator is an unconstrained optimum, and the
  committed baseline carries rows on both sides of one. Refusing that
  would be the instrument refusing to report its own subject.
- **A relation across ROWS is a join, not an admission**, and its
  voice is decided by the same test rather than by this page.
  `tess_lint::parse` refuses a repeated `(scene, face)` in the harness
  voice, because a mis-join is a reading nobody could make; rule 4
  announces a re-key as a FINDING, because what it costs is a
  comparison the gate claims to have made.

Rule 5 stays the standing counter-example to the whole shape and is
not an exception to it: it speaks in the harness register and exits
`EXIT_FINDINGS`, because format agreement is perfect there and what is
missing is a reference the author supplies by folding.
