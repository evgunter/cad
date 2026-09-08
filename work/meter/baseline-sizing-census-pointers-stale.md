---
id: baseline-sizing-census-pointers-stale
kind: issue
title: Six sites still point at tools/tess-lint/tests/baseline_sizing_census.rs, which no longer exists; one line citation had already moved
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
(`the_committed_baseline_sizes_this_much`). Five sites still name the
deleted file, and one of them additionally frames the guard as TWO
FILES rather than as two censuses. Citations re-taken after merging
`origin/main` at `ef3edaa3f`, which moved the two document ones:

- `docs/TESS-BUDGET.md:262` — *"The executable home is
  `tools/tess-lint/tests/baseline_sizing_census.rs`"*.
- `docs/TESS-BUDGET.md:274` and `:277` — the two-file framing:
  *"that pair is the neighbouring `baseline_census.rs`'s"* and *"the
  two test files together are a guard that runs unasked"*. The claim
  survives the fold — the two censuses still assert disjoint figures,
  and both still run unasked — but the sentence's mechanism does not.
- `docs/TESS-BUDGET.md:519-521` — *"`…/baseline_census.rs` (the
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
- `work/meter/report-header-column-phrases-unqualified.md:125`
  (closed) — *"`baseline_sizing_census.rs` asserts it"*, of
  `opt_cells`. Still asserted, in the other file.

`work/meter/plan.md:117` also names the folded file as
`tests/baseline.rs`; unit 10 kept the name `baseline_census.rs`
instead, because 24 live mentions name that path and five name the
other, so renaming both maximised the breakage rather than minimising
it. The plan line is the orchestrator's.

**A closed item is not exempt.** `baseline_census.rs`'s own module
docs state the rule this row is an instance of: a dated record may
keep the FIGURES it reported on the day it reported them, because
that is what makes it a record, but *"a pointer that no longer
resolves has stopped being a record of anything"*. Two of the six
rows above are dated records whose pointers need to follow.

## Why not in the unit that found it

Unit 10's fence was the two test files, its own item and new items it
files. `docs/TESS-BUDGET.md` was held by no lane but is outside that
fence, and the two `work/meter/` items are other units' files —
one-file-one-item makes editing them from this branch a merge
conflict by design.

## Sweep

`grep -rn "baseline_sizing_census\|baseline_census"` over `*.rs`
`*.md` `*.toml` `*.py` `*.yml` `*.yaml` `*.sh`, plus
`grep -rn "two census\|both census\|census files\|two test files\|both
tests"` for the framing that names no path. Re-run after merging
`origin/main` at `ef3edaa3f`, which added one hit
(`report-header-column-phrases-unqualified.md:125`) and moved two.
**What the pattern could not match**: a pointer that paraphrases the
path without the stem (e.g. "the sizing census's file"), a pointer in
a non-text artefact, and a pointer written after this sweep.

NOT stale, and listed so a later reader does not re-file them: there
are still two census tests, so `work/meter/plan.md:94` and
`work/meter/log.md:226` (*"both census tests re-derive"*),
`work/meter/log.md:323` (*"both census-touching"*) and
`docs/TESS-BUDGET.md:530` (*"passes both tests silently"*) all still
say something true.
