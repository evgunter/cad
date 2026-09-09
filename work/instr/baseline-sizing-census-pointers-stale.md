---
id: baseline-sizing-census-pointers-stale
kind: issue
title: Seven mentions of tools/tess-lint/tests/baseline_sizing_census.rs survive the file, one of them in code; and one line citation had already moved
status: open
opened: 2026-09-08
---


## Was

disclosed by METER unit 10, the fold of the two baseline census files,
and filed in the PR that carries it. The fold deleted
`tools/tess-lint/tests/baseline_sizing_census.rs`; every site outside
that unit's fence that names the path is now a pointer to nothing.
**Unit 10's first pass at this row missed one of them and its own
sweep is why — see below.**

## Finding

The sizing census now lives beside the face-identity one in
`tools/tess-lint/tests/baseline_census.rs`, under the same `#[test]`
name it always had (`the_committed_baseline_sizes_this_much`). **Seven
mentions across six files name the deleted path**, and one further
site frames the guard as TWO FILES without naming either. Citations
taken from the tree at `origin/main` merged at `ef3edaa3f`:

1. `tools/tess-lint/tests/report_columns_pin.rs:40` — *"`baseline_sizing_census.rs`
   asserts what the COMMITTED baseline's four cell sums are"*, inside
   the paragraph that argues that pin does NOT overlap the sizing
   census. The argument survives; the path does not. **This is the one
   in code, in the same `tests/` directory as the fold**, and it is
   the site unit 10's own sweep missed.
2. `docs/TESS-BUDGET.md:262` — *"The executable home is
   `tools/tess-lint/tests/baseline_sizing_census.rs`"*.
3. `docs/TESS-BUDGET.md:520` — *"and `…/baseline_sizing_census.rs`
   (the sizing census) each read the committed file"*. The list of
   what neither census reads, immediately below, is unaffected.
4. `work/meter/tess-lint-ungated-columns-fold-silently.md:79` (open) —
   *"reads totals only, so all seven movers above are invisible to
   it"*. True of the census; the path is gone.
5. `work/meter/tess-budget-doc-finding-block-stale.md:202` (closed) —
   *"is the executable home, on the `baseline_census.rs` precedent and
   beside it."*
6. `work/meter/report-header-column-phrases-unqualified.md:125`
   (closed) — *"`baseline_sizing_census.rs` asserts it"*, of
   `opt_cells`. Still asserted, in the other file.
7. `work/meter/plan.md:117` — names the folded file as
   `tests/baseline.rs`, which is neither file's name and not the name
   unit 10 used.

And the framing that names no path: `docs/TESS-BUDGET.md:274` and
`:277` — *"that pair is the neighbouring `baseline_census.rs`'s"* and
*"the two test files together are a guard that runs unasked"*. The
claim survives the fold — the two censuses still assert disjoint
figures and both still run unasked — but its mechanism does not.

**Why the name went the way it did.** 24 mentions across 14 files name
`baseline_census.rs` against these 7 across 6, so folding into the
majority name stales 7 where renaming both would have staled 31. The
decision does not turn on the exact figure, but the figure is the
argument, so it is stated as counted rather than as remembered.

**A closed item is not exempt.** `baseline_census.rs`'s own module
docs state the rule this row is an instance of: a dated record may
keep the FIGURES it reported on the day it reported them, because that
is what makes it a record, but *"a pointer that no longer resolves has
stopped being a record of anything"*. Two of the rows above are dated
records whose pointers need to follow.

## The line citation, which was stale before the fold and is now stale by more

`work/meter/baseline-census-partition-assert-cannot-fail.md:12` cites
`tools/tess-lint/tests/baseline_census.rs:228-233` and quotes the
`constant.len() + discriminating.len()` assert. **That citation did
not resolve on `origin/main` either**: at `9ad3b0bc7` the assert is at
`:391-395` and `:228-233` is the inner loop of `indistinguishable_pairs`,
an unrelated function the swap-cost unit added above it. Unit 10's
fold moves it again.

Filed here rather than fixed because the item is another unit's file.
It is the instance worth naming of a hazard the standing discipline
states: **a citation into a file your own pass edited is stale by the
lines you inserted above it**, and here two passes in a row inserted
above it. A citation by the enclosing `#[test]` name plus the quoted
source — which that item already carries — cannot go stale this way;
the line range adds nothing it does not already have. **No replacement
range is given here on purpose**: this row would then be the third
pass to write one, and the fix is to drop the range, not to refresh
it.

## Why not in the unit that found it

Unit 10's fence was the two census files, its own item and new items
it files. `docs/TESS-BUDGET.md` and `tests/report_columns_pin.rs` are
both outside it — the second explicitly, as another lane's file — and
the four `work/meter/` rows are other units' files, which
one-file-one-item makes a merge conflict by design.

## Sweep, and how unit 10's first pass missed row 1

The pattern is `grep -rn "baseline_sizing_census\|baseline_census"`
over `*.rs` `*.md` `*.toml` `*.py` `*.yml` `*.yaml` `*.sh` from the
repo root, plus `grep -rn "two census\|both census\|census
files\|two test files\|both tests"` for the framing that names no
path.

**Unit 10 stated that pattern and then ran a narrower one.** Its
pre-merge sweep was the stated one and was complete against that tree.
After merging `origin/main` it re-swept with
`grep -rn "baseline_sizing_census\.rs" work/ docs/` — two directories
instead of the repo root — and `tools/tess-lint/tests/report_columns_pin.rs`
is a file the merge itself had just created. The narrower command
found six mentions and the row was written claiming the merge added
one hit; the stated pattern finds seven and the merge added two
(`report-header-column-phrases-unqualified.md` and
`report_columns_pin.rs`). **A sweep result is only as wide as the
command actually typed**, and re-running it after a merge that adds
files is exactly the case where the two come apart.

**What the pattern could not match**: a pointer that paraphrases the
path without the stem (e.g. "the sizing census's file"), a pointer in
a non-text artefact, and a pointer written after this sweep.

NOT stale, and listed so a later reader does not re-file them: there
are still two census tests, so `work/meter/plan.md:94` and
`work/meter/log.md:226` (*"both census tests re-derive"*),
`work/meter/log.md:323` (*"both census-touching"*) and
`docs/TESS-BUDGET.md:530` (*"passes both tests silently"*) all still
say something true.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
