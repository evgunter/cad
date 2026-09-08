---
id: baseline-sizing-census-pointers-stale
kind: issue
title: Pointers into the folded baseline census files: four to a deleted path, one to a line that had already moved
status: open
opened: 2026-09-08
---


## Was

disclosed by METER unit 10, the fold of the two baseline census files,
and filed in the PR that carries it. The fold deleted
`tools/tess-lint/tests/baseline_sizing_census.rs`; every site outside
that unit's fence that names the path is now a pointer to nothing.

## Finding

The sizing census now lives beside the face-identity one in
`tools/tess-lint/tests/baseline_census.rs`, under the same
`#[test]` name it always had
(`the_committed_baseline_sizes_this_much`). Four sites still name the
deleted file, and two of them additionally frame the guard as TWO
FILES rather than as two censuses:

- `docs/TESS-BUDGET.md:261` — *"The executable home is
  `tools/tess-lint/tests/baseline_sizing_census.rs`"*, and the
  paragraph around it (`:266-270`) turns on the two-file framing:
  *"those are the neighbouring `baseline_census.rs`'s"* and *"the two
  test files together are the guard, and neither is a subset of the
  other"*. The claim survives the fold — the two censuses still
  assert disjoint figures — but the sentence's mechanism does not.
- `docs/TESS-BUDGET.md:508-510` — *"`…/baseline_census.rs` (the
  face-identity census) and `…/baseline_sizing_census.rs` (the sizing
  census) each read the committed file"*. The list of what neither
  reads, immediately below, is unaffected.
- `work/meter/tess-lint-ungated-columns-fold-silently.md:79` (open) —
  *"`tools/tess-lint/tests/baseline_sizing_census.rs` reads totals
  only, so all seven movers above are invisible to it"*. True of the
  census; the path is gone.
- `work/meter/tess-budget-doc-finding-block-stale.md:202` (closed) —
  *"`tools/tess-lint/tests/baseline_sizing_census.rs` is the
  executable home, on the `baseline_census.rs` precedent and beside
  it."*

`work/meter/plan.md:117` also names the folded file as
`tests/baseline.rs`; unit 10 kept the name `baseline_census.rs`
instead, because ~20 live pointers name that path and four name the
other, so renaming both maximised the breakage rather than minimising
it. The plan line is the orchestrator's.

**A closed item is not exempt.** `baseline_census.rs`'s own module
docs state the rule this row is an instance of: a dated record may
keep the FIGURES it reported on the day it reported them, because
that is what makes it a record, but *"a pointer that no longer
resolves has stopped being a record of anything"*. The
`tess-budget-doc-finding-block-stale` row is a dated record whose
pointer needs to follow.

## Why not in the unit that found it

Unit 10's fence was the two test files, its own item and new items it
files. `docs/TESS-BUDGET.md` was held by no lane but is outside that
fence, and the two `work/meter/` items are other units' files —
one-file-one-item makes editing them from this branch a merge
conflict by design.

## Sweep

`grep -rn "baseline_sizing_census\|baseline_census" --include=*.rs
--include=*.md --include=*.toml --include=*.py --include=*.yml
--include=*.yaml --include=*.sh .` over the whole tree, plus
`grep -rn "two census\|both census\|census files\|two test files\|both
tests"` for the framing that names no path. **What the pattern could
not match**: a pointer that paraphrases the path without the stem
(e.g. "the sizing census's file"), a pointer in a non-text artefact,
and a pointer written after this sweep. The second grep's hits in
`work/meter/plan.md:94` and `work/meter/log.md:226` (*"both census
tests re-derive"*) are NOT stale: there are still two census tests,
now in one file.

## The line citation, which was stale before the fold and is now stale by more

`work/meter/baseline-census-partition-assert-cannot-fail.md:12` cites
`tools/tess-lint/tests/baseline_census.rs:228-233` and quotes the
`constant.len() + discriminating.len()` assert. **That citation did
not resolve on `origin/main` either**: at `9ad3b0bc7` the assert is at
`:391-395` and `:228-233` is the inner loop of
`indistinguishable_pairs`, an unrelated function the swap-cost unit
added above it. Unit 10's fold moves it again, to `:517-521`.

Filed here rather than fixed because the item is another unit's file.
It is the instance worth naming of a hazard the standing discipline
states: **a citation into a file your own pass edited is stale by the
lines you inserted above it**, and here two passes in a row inserted
above it. A citation by the enclosing `#[test]` name plus the quoted
source — which that item already carries — cannot go stale this way;
the line range adds nothing it does not already have.
