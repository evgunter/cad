---
id: keep-out-names-a-track-that-owns-nothing
kind: issue
title: CIW's keep_out gives tools/* to code-quality Track K, which has no program; INSTR owns it
status: open
opened: 2026-09-11
---


## Finding

`work/ciw/program.md:12`'s first `keep_out` clause reads

```
scripts/gates/* and tools/* are code-quality Track K's
```

Neither half names the live owner any more:

- `tools/*` is **INSTR's**, by that program's own `paths`
  (`work/instr/program.md:11`, opened 2026-09-08), and INSTR's
  `keep_out` says so from the other side — it inherits *"the tools/*
  half only"* and names `scripts/tess_budget_cut.sh` and its two
  siblings as CIW's.
- `scripts/gates/*` is **GATES'**, which `work/ciw/plan.md`'s unit 3
  already records in prose (*"`scripts/gates/*` is GATES' program now"*)
  while the header it is supposed to describe still says Track K.

`work/code-quality/` is not a program at all: the directory holds
unclaimed rows (`D106`, `C13`, …) and carries no `program.md`, so
`scripts/work.py` resolves no owner from it and the clause resolves to
nobody.

## Why it costs something

A `keep_out` clause is what a lane reads before it decides whether an
edit is invited or taken. Pointed at a directory with no program, the
honest readings are both wrong: that the path is unowned, or that its
owner is a closed track whose orchestrator cannot be asked. The live
answer — INSTR, open, with an away channel — is one the clause
currently hides. The unit that anchored `CUT_RE`
(`cut-regex-unanchored-admits-a-line-the-lint-refuses`) crossed into
`tools/tess-lint/tests/cut_line_pin.rs` on an invitation written into
that file's own alarm string, and named the fence as Track K's from
this clause before `scripts/work.py territory --base origin/main`
answered `owned by instr`.

## Fix

Re-spell the clause to name GATES and INSTR, in the PR that next
touches `work/ciw/program.md`'s header. It is an orchestrator edit:
`program.md` is a program-level document and a unit lane rewriting it
races the orchestrator's own branch for a one-file-one-item conflict.
