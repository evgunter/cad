---
id: tess-budget-doc-note-finding-rule
kind: issue
title: the note-vs-finding rule has a fourth home in docs/TESS-BUDGET.md, carrying only its short form
status: open
opened: 2026-09-07
---

Filed by the style-review fix pass on `meter/join-gated-voice` (PR 2111,
`tess-lint-face-ordinal-join`), which is where the rule was extended.

**The rule.** *A re-key is a FINDING where it can cost a measurement,
judged per SCENE; where the scene carries no Hessian-sized face it is a
NOTE, because rule 1 still runs over its total and no comparison was
lost.* PR 2111 added the argument behind it — rule 5's harness-register
precedent, and the note's conversion property together with what that
conversion does NOT promise.

**It has four homes and one of them is stale.**

- `tools/tess-lint/src/lib.rs` module docs — the long form, the one
  that now carries the argument.
- `tools/tess-lint/src/main.rs:132-138` — recourse item 4, the operator
  form.
- `tools/tess-lint/src/lib.rs` on `Report` — the criterion the split is
  decided by.
- `docs/TESS-BUDGET.md:398-400` — *"Where the scene carries no
  Hessian-sized face the same event is a NOTE rather than a finding —
  rule 1 still runs over its total, so no comparison was lost."* The
  short form only. It is not wrong, but it is now the one statement of
  the rule with no pointer at the argument, and `tests/cli_contract.rs`
  asserts the CLI quotes this document, so the two are already coupled.

**Not fixed on 2111, and deliberately.** `docs/TESS-BUDGET.md` is held
by a live lane (PR 2114); editing it from a second branch is a merge
conflict by construction. The fix belongs at that lane's seam.

**What the fix is.** The cure this tree already states for the rule
roster next door: one home, everything else a pointer. Replace the
short form with a pointer at `tools/tess-lint/src/lib.rs`'s module
docs, or accept it as an operator-facing restatement and say at the
site that the argument lives there — but not a fourth independent
statement that a future extension has to remember to visit.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
