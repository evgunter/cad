---
id: tess-budget-doc-identity-column-list
kind: issue
title: docs/TESS-BUDGET.md's two identity-column enumerations still name the sizing-block entry tess-lint no longer carries
status: open
opened: 2026-09-07
---


Disclosed by METER unit 1 (`D213`/`D214`, branch
`meter/d213-d214-sizing-block`). It was filed because unit 2 held
`docs/TESS-BUDGET.md` for the wave; **unit 2 has since merged, so that
reason has expired and the file is free.** What still keeps the edit
out of unit 1 is the fence: `D213`'s is `scripts/gates/` less two,
`tools/` and `docs/K-REPORT.md`, and `docs/TESS-BUDGET.md` is in
none of them. The item is schedulable now by whoever takes the
document next.

`tess_lint::IDENTITY_COLUMNS` lost its `"the sizing block"` entry in
that unit: with `parse` now refusing either half of the lane pairing
without the other, block presence is a function of `chart`, `chart` is
entry zero, and the block entry could never be the first disagreement
rule 4 reports. The list is seven entries — chart, the four trim-box
edges, `nu`, `nv`.

`docs/TESS-BUDGET.md` enumerates that list twice and both copies still
carry the eighth:

- `:120-123` at `origin/main` after unit 2 merged — *"the identity
  columns its per-face join checks itself against — chart, trim box,
  the whole-patch divisions, and whether the row carries a sizing block
  at all"*, in the paragraph that makes `--sizing-only` sound.
- `:543-546` — *"`tess-lint` checks at each ordinal that both sides
  describe one face (chart, trim box, whole-patch divisions, and
  whether the row carries the sizing block at all)"*, under **A
  re-keyed face is read before it is re-cut**.

Both were re-checked against `origin/main` on 2026-09-08 (unit 2 landed
in between and moved the line numbers; neither sentence changed).

**A third copy was found and is CLOSED in this unit's own PR**, not
deferred: `local-scripts/ci-local.sh`'s hosted-mirror comment on
`tesslint_gate` said the gate JOINS on *"`chart`, whether the row
carries the sizing block at all, and `u0`-`v1` / `nu` / `nv`"*. That
file is held by nobody and is what a reader reproducing CI locally
reads. The class is therefore three copies, two of which remain here.
The tracker prose that restated the same list — `work/meter/C15.md`
and `work/meter/D201.md`, both open — was corrected in the same PR.

Neither is merely stale in its list: the second sentence describes what
the gate WOULD announce, and a reader expecting a re-key naming *"the
sizing block"* now gets a parse refusal in the harness voice on the row
that carries it, which is a different verdict, a different exit code
and a different recourse. Both sentences want the seven-entry list and
a clause pointing at the parse refusal for the case they used to cover.

`docs/TESS-BUDGET.md`'s own standing rule applies to the fix — the
document is not the census's home, so name the list, not a count.
