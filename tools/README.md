# `tools/` — the instruments, and the cross-column rule

Three crates that MEASURE the kernel rather than build it, each its
own cargo root outside the workspace:

| Crate | What it is |
| --- | --- |
| `tess-meter` | the tessellation-budget meter's consumer half: the CSV schema, the counterfactual schedules, the split optimizer |
| `tess-lint` | the gate over that CSV |
| `k-lint` | the gate over the kernel-probe sweep |

`tess-meter` writes what `tess-lint` reads, and the two share no code
by design — separate cargo roots, no import, no shared constant, so a
drifting sweep fails as a PARSER rather than parsing into the wrong
columns. What `tess-lint` and `k-lint` share is not code either. It is
one rule about their reading boundaries, and this page is its home:
here rather than inside either lint, because its subject is both of
them and a rule hosted inside one of its two consumers is the shape
that drifts.

## The rule

Each lint polices its input with a private `Admissible` enum and a
table of `(column, Admissible)` pairs, so that the block the parser
polices and the block the header declares can be compared to each
other. That table is **per column**: an entry sees one field of one
row and nothing else. Several of the producer's guarantees are not of
that shape, and the class is large enough that it gets one statement
here and citations from every site rather than a re-derivation at
each.

The clauses are `CC1`–`CC5`. Every site that carries one cites it by
this path and by id.

## `CC1` — at the reading boundary, and only there

A cross-column admission is checked in the one function that turns
text into readings: `tess_lint::parse`, and `k_lint::lint_csv`, which
parses and scans in one pass because k-lint keeps no row type of its
own. **Those are the same place, not two answers**: each is its
crate's reading boundary, and the gate's only input is a file, so
every reading the GATE makes came through one of them.

**What the boundary buys is not a type, and neither crate pretends
otherwise.** `tess_lint::Row`'s fields are `pub` and `compare` takes
`&[Row]`, so a hand-built `Row` can hold a pair `parse` refuses — the
type says so at itself. `k-lint` has no row type at all: it exports
`lint_sample` over raw scalars, and its own unit tests call it with
hand-written band thresholds that never met `CC3`'s relation. What the
clause buys is that the check has ONE site on the path CI takes. A
cross-column check placed at a consumer instead holds for that
consumer alone, and the next consumer cannot see that it is missing.

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
and before the row is built. A whole block's all-or-none presence is
the same clause over a block rather than over a pair.

**No roster of the live instances lives on this page.** Each site
cites this clause where it stands, which is where a reader meets it; a
list here would be a second roster with nothing to keep it honest, and
it would rot in the direction that reads as completeness.

**Several such checks at one boundary run in the order their blocks
are read, and a row that drifted in two of them reports the first.** A
harness message names *a* reading that broke, never the only one —
which is the right contract for a voice whose whole claim is that the
file cannot be read.

## `CC4` — a producer-side entailment is not a disposition

There is no fourth place to put the check. A cross-column property the
producer's code makes unreachable is checked anyway, because the
boundary cannot see the producer's code.

**The temptation is concrete, and this clause exists because this page
once stated its opposite as a clause.** `tess-lint`'s
`Admissible::Extent` admits a trim-box edge as finite and nothing
more; the box's own non-degeneracy — `u0 < u1`, `v0 < v1` — is beyond
what an entry over one edge can say, and it looked entailed:
`span_opt_cells` is an `Admissible::CellCount`, floored at one, and
the accumulator that writes it skips analysis cells outside the trim
box, so a degenerate box accumulates zero and the column is refused.

That entailment is true about the PRODUCER and says nothing about a
row. A row carrying `u0 = u1 = 0` and `span_opt_cells = 25` satisfies
every per-column entry, and the degenerate box reaches rule 4's
`identity` as a face-identity reading;
`tess-lint`'s `a_degenerate_trim_box_is_harness_breakage` is that row,
and it went red at the check that now refuses it.

The distinction: an entailment may be relied on only when it follows
from **what this row's own columns were admitted to say**.
"`span_opt_cells ≥ 1` implies `u0 < u1`" is not a fact about the row.
It is a fact about how `tools/tess-meter` computes that column — in
another crate, in another cargo root, invalidated by whoever edits it
there with nothing to tell them — and relying on it is exactly the
assumption these instruments exist not to make. The producer says so
from its own side: *"A parse guard bounds what a column may SAY; it
cannot know whether the producer measured it"*
(`tools/tess-meter/src/lib.rs`, the counterfactual-column paragraph).

The same clause is why `patch_cells = nu·nv` and
`opt_cells ≤ patch_cells` are checked at `parse` rather than read off
`tess_meter::columns`, which states both in three lines of code.

## `CC5` — the voice, and the test for whether a check is owed

Every clause above exits in the **harness voice** — `EXIT_HARNESS`,
the same exit a renamed column or an unknown chart tag gets. A
cross-column admission failure is the producer and the consumer
disagreeing about what the file says, so what broke is the reading,
not the geometry. Each lint spells that constant privately in its own
`main.rs`; the voice is one thing and the constant is two, for the
same reason nothing else crosses these cargo roots.

The test for whether a check is owed at all is **not** "does it span
columns", and this page does not restate it: it is the one
`tess_lint::Report` states, and it is stated there. What this page
owes is what falls out of it for a cross-column reading.

- **An admission refuses what the producer could not have written; it
  never refuses what the instrument exists to measure.** `cap_bands`
  counts a subset of `bands`, so a larger `cap_bands` is arithmetic
  that did not happen. The optimality relations look identical and are
  not: `grid_cells` against `span_opt_cells` is the `split` ratio the
  report is FOR, its denominator is an unconstrained optimum, and the
  committed baseline carries rows on both sides of one. Refusing that
  would be the instrument refusing to report its own subject.

  **The boundary between the two is where the equality case lives, so
  it is chosen and not defaulted.** `cap_bands == bands` on all 64
  sized rows of the committed baseline: every emitted band bound by
  the cap is a reading, the equality case IS the dataset, and a `>=`
  written where `>` belongs reds the whole gate rather than a corner
  of it.

- **A relation across ROWS is a join, not an admission**, and its
  voice is decided by the same test rather than by this page.
  `tess_lint::parse` refuses a repeated `(scene, face)` in the harness
  voice, because a mis-join is a reading nobody could make; rule 4
  announces a re-key as a FINDING, because what it costs is a
  comparison the gate claims to have made.

- **A check can be owed with no discriminant in the row**, and that is
  not a disposition. `tess-lint`'s `Admissible::Certificate` reads
  `worst_cert = 0` as a face whose triangles are exact, and
  `mesh::budget` says the same zero also spells a face that refused
  before the emit pass and certified nothing. The refusal is owed by
  the test above; nothing in the CSV separates the two states. What
  that owes is an open finding against the PRODUCER — filed, as
  `work/meter/tess-lint-zero-certificate-two-meanings` — and never a
  clause here saying the reading is fine.

`tess-lint`'s rule 5 stays the standing counter-example to the whole
shape and is not an exception to it: it speaks in the harness register
and exits `EXIT_FINDINGS`, because format agreement is perfect there
and what is missing is a reference the author supplies by folding.

## Ratification

**This page is not ratified.** It is a lane's writeup of a rule that
six code sites already cite as clause law, and `docs/DESIGN.md`'s
companion table carries no row for it.

Ev ratifies it, as Ev ratified `scripts/gates/README.md`. The row in
that table is what puts a page to Ev — the gates page's landed reading
*"RECOMMENDATION — awaiting Ev"* and was flipped to *"Ratified"* by
the commit recording the answer — and it is owed at that moment, not
at any program's close.
