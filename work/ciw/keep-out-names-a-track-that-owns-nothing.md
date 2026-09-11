---
id: keep-out-names-a-track-that-owns-nothing
kind: issue
title: CIW's keep_out gives tools/* to code-quality Track K; INSTR has owned it since 2026-09-08
status: open
opened: 2026-09-11
---


## Finding

`work/ciw/program.md:12`'s first `keep_out` clause reads

```
scripts/gates/* and tools/* are code-quality Track K's
```

**Half of that is correct and stays.** `scripts/gates/*` really is
code-quality's again: GATES was opened for that half on 2026-09-06 and
**closed on 2026-09-08** (`docs/DOC-LEDGER.md`, "Sweep 7 — GATES leaves
the tracker"; `work/gates/` no longer exists), and
`work/code-quality/program.md:70-78` records the return in its own
words — *"`gates` closed 2026-09-08 … and its half of the fence,
`scripts/gates/*`, is this program's again"*, with `work/meta/program.md`
saying the same independently. `work/code-quality/program.md` exists,
is `status: open`, and carries `tag: (SMELL orchestrator)`, so that
half of the clause names a live program with a live owner.

**`tools/*` is the half that is wrong.** It is INSTR's, by that
program's own `paths` (`work/instr/program.md:11`, opened 2026-09-08),
which took the `tools/*` half of Track K when GATES took the other.
INSTR's `keep_out` says so from the other side: it inherits *"the
`tools/*` half only"* and names `scripts/tess_budget_cut.sh` and its
two siblings as CIW's, citing the two cut-script residues on this
slate by name.

## Why it costs something

A `keep_out` clause is what a lane reads before deciding whether an
edit into another program's file is invited or taken. On `tools/*` it
currently sends that lane to a program that does not own the path,
and past a live one that does — INSTR, open, with an away channel and
an orchestrator to announce to.

That is not hypothetical: the unit that anchored `CUT_RE`
(`cut-regex-unanchored-admits-a-line-the-lint-refuses`) edited
`tools/tess-lint/tests/cut_line_pin.rs` on an invitation written into
that file's own alarm, and named the fence as code-quality's from this
clause. `python3 scripts/work.py territory --base origin/main` is what
corrected it, answering `owned by instr`.

**`territory` will not catch the `scripts/gates/*` half either way**,
which is worth knowing before anyone leans on it here:
`work/code-quality/program.md` declares no `paths:` key at all, so the
resolver attributes nothing to that program and a diff touching
`scripts/gates/*` draws no warning. The prose clause is the only
notice a lane gets there, which is the argument for keeping that half
accurate rather than deleting it.

## Fix

Re-spell the clause so the two halves are separate: `scripts/gates/*`
is code-quality Track K's (live, `(SMELL orchestrator)`), `tools/*` is
INSTR's. It is an orchestrator edit — `program.md` is a program-level
document and a unit lane rewriting it races the orchestrator's branch
for a one-file-one-item conflict.

Two sentences elsewhere carry the same staleness and are cheapest to
fix in the same pass, both in this program's own files:

- `work/ciw/plan.md`'s unit 3 — *"**Fence, new as of 2026-09-06:**
  `scripts/gates/*` is GATES' program now, so widening
  `gate-roster.sh`'s scope is announced to GATES and drawn with it"*.
  GATES closed two days after that was written; the announcement now
  goes to code-quality.
- The same plan's "Fences" section — *"Track K keeps `scripts/gates/*`
  and `tools/*`"* — which is the clause above in its original form.

## Note on this item's id

The id says `names-a-track-that-owns-nothing`, which is the claim the
first draft of this finding made and the tree does not support:
code-quality owns `scripts/gates/*` and has a `program.md`. Ids are
stable (`work/README.md`), and this one was already in circulation
when the error was caught, so it stays and this paragraph is the
correction. The title and the body are what carry the finding.
