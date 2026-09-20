---
id: tint-plan-states-the-withdrawn-reviewer-independence-rule
kind: issue
title: tint's plan states the withdrawn reviewer-independence rule beside the ruling that withdrew it
status: open
opened: 2026-09-19
---



## Finding

- **Where**: `work/tint/plan.md`, in the reading list, in one bullet:

  > `memories/review-and-dependency-policy.md` — retirement is always
  > permitted, and a reviewer suite's **independence is worth keeping**
  > where it pulls its weight; **reviewer tests are ordinary tests**
  > (Ev's ruling of 2026-09-04, in `work/tcost/log.md`'s seam).

- **Why it is wrong**: the bullet states the withdrawn reading and the
  ruling that withdrew it as though they were two compatible clauses of
  one memory. *"Independence worth keeping"* is verbatim one of the
  three phrases the memory names as retired — *"An earlier version of
  this memory made reviewer suites a protected class ('promoted as-is',
  'independence worth keeping', 'never simplify to match shipped
  fixtures'); that reading was withdrawn"* — and **the ruling cited in
  the second half of the same sentence is the one that retired it**.
  The surviving clause is per row, not per author: keep your own code
  *"only where a row's claim needs its own derivation (a general
  test-design question, not a question of who wrote the row)."*
- **Importance**: high. This is worse than a stale sentence: a lane
  reading it sees the ruling named and correctly dated, and reasonably
  takes the clause beside it to be part of that ruling. The date is
  right and the sequence is inverted.
- **Confidence**: sure.
- **Raised by**: the S-DUP lane for the withdrawn-no-simplify unit,
  2026-09-19, found by enumerating every tracked citation of
  `memories/review-and-dependency-policy.md` — 30 occurrences in 27
  files — and classifying each by what it claims the memory says.

## What the remedy is, and why S-DUP did not apply it

Deleting the withdrawn half leaves the bullet accurate. But this bullet
is in the plan's reading list, so what it says is a standing
instruction to every S-TINT lane about when a reviewer suite may be
folded — and that is S-TINT's call, not a visiting lane's. Two of this
program's own slate rows turn on the answer.

`work/tcost/plan-states-the-withdrawn-reviewer-independence-rule.md` is
the same defect in S-TCOST's plan, which states it in its **exit
criteria** as well as its reading list. They are filed separately
because one file is one item.
