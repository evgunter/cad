---
id: cut-line-commit-names-no-baseline-change
kind: issue
title: The tess-budget cut line stamps the sweeping tree's HEAD, so the commit it names need never have touched the baseline
status: open
opened: 2026-09-07
refs: [2151]
priority: P3
cost: D
---


## What

The committed baseline's provenance line reads
`# tess-budget-cut: aba2625f8f84 2026-09-04T03:16:35-07:00`, and
**`aba2625f8f84` is a commit that never touched
`docs/tess-budget-data/tess-budget-baseline.csv`.** A reader chasing
"what re-cut this baseline" through `git show <cut>` finds a commit
whose diff says nothing about the rows.

This is not a bug in the stamp. It is exactly what
`scripts/tess_budget_cut.sh`'s third arm specifies — a freshly written
sweep is stamped with HEAD, *"the tree that produced it"* — and the
script argues that case at length in its header. The cut dates the
**tree the sweep ran on**, which is the quantity rule 5 needs when it
asks whether a scene arrived after the cut or was already outside the
gate. It is honest and it is the right quantity.

## Finding

**The defect is that nothing says so where the value is READ.**
`tools/tess-lint` parses the line, prints it beside every verdict, and
its `Cut` type carries the commit as the answer to "when was this
baseline taken" — but neither the parse site nor the printed verdict
tells a reader that the commit is the sweeping tree's HEAD rather than
the commit that landed the rows. The distinction only exists in
`scripts/tess_budget_cut.sh`'s header, one cargo root and one language
away from the consumer that shows the number to people.

Two ways it misleads, both cheap and both silent:

- a reader who `git show`s the cut concludes the stamp is wrong, or
  that the baseline was hand-edited;
- a reader who wants the commit that changed the rows — the one whose
  PR body would say WHY the budget moved, which is the thing the
  re-cut discipline exists to produce — cannot get it from the
  baseline at all, and there is no pointer saying where else to look.

**Relation to `cut-prefix-three-unpinned-spellings` (unit 4).** That
item is about the three spellings of the prefix and the fact that the
reader does not check the SHAPE the writer's regex constrains. This is
the same seam and a different defect: the reader does not check, or
state, the MEANING. A lane landing on unit 4 is already in both files
and should decide whether to take this as a rider or leave it; it is
filed separately rather than folded in because the two would be fixed
by different edits and only one of them is a pin.

**Confidence:** sure that `aba2625f8f84` does not touch the baseline
path (checked with `git log` over the path); sure that the third arm
specifies HEAD (read in `scripts/tess_budget_cut.sh`'s header); unsure
whether any reader has actually been misled, which is why this is an
issue and not a unit.

## Was

Reported out-of-fence by the unit 0 lane (`meter/join-gated-voice`,
2026-09-07) while dating the baseline for its sweep, and filed here by
the orchestrator per `docs/prompts/implementer-discipline.md` §6 —
the lane reports, the party with the whole board places it.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## Refs at METER's sweep (2026-09-09)

METER closed and its item files left the tracker (`docs/DOC-LEDGER.md`, sweep 10); `cut-prefix-three-unpinned-spellings` is now cited by its closing PR 2151.

## A third way it misleads, worse than the two above, measured on the orchestrator (2026-09-16)

The two modes this row lists both end with the reader knowing something
is wrong — the stamp looks broken, or the wanted commit cannot be
reached. **There is a third that ends with the reader confidently
holding the wrong numbers**, and it cost a live adjudication today.

`git show <cut>:docs/tess-budget-data/tess-budget-baseline.csv` does
not yield the baseline that cut produced. It yields **the baseline as
it stood at that commit — the PREVIOUS cut's file**, which parses
cleanly, carries the same shape, and sums to different totals. Measured:

| read | cut line inside the file | `opt_cells` | `span_opt_cells` |
|---|---|---|---|
| `git show 3f55f361b22e:…baseline.csv` | `aba2625f8f84` | 94,154 | 44,446 |
| the file STAMPED `3f55f361b22e` (committed at `715977e6a`) | `3f55f361b22e` | 93,066 | 44,162 |

So the natural spelling of *"let me check the figures against the cut
the document names"* silently answers a different question, and returns
a plausible answer. Nothing errors; the two files differ only in
values.

**What it cost.** INSTR unit 1 froze a passage in `docs/TESS-BUDGET.md`
and labelled it with the cut it reads. The orchestrator checked the
label by the spelling above, got 94,154 / 44,446 against the document's
93,066 / 44,162, and sent the unit a correction instructing it to
re-point the label at a later cut — which would have been **wrong**:
that later blob (`448275c8d`) agrees on the four sums but carries 316
named rows against 286, so re-pointing would have broken the same
passage's `name` figure. The lane re-summed, found the stamp-versus-tree
distinction, and refused the instruction. The correct attribution
survived because a lane checked an orchestrator, not because the
tooling said anything.

**This sharpens what the fix owes.** The row already asks that the read
site say the commit is the sweeping tree's HEAD. It should also give
the reader the thing they actually want at that moment: **the commit
whose blob holds the rows**, which is not derivable from the stamp by
any spelling. Unit 1's own repair — every cut citation written as two
parts, the stamp AND the commit to sum, with the re-sum command beside
it — is the shape that works in prose; the instrument has no equivalent
yet, and `tess_lint::Cut` is where it would live.

**Whoever takes this row is warned**: the hazard reaches the people
holding the whole board, not only a passing reader, and it did so on a
slate that already carried this row.
