---
id: a-merged-face-is-named-for-exactly-the-member-faces-it-holds
kind: unit
title: A boolean result face's parent is exactly the member faces whose area it holds (per-face exact), for union, intersect and subtract
status: review
opened: 2026-11-02
priority: P1
cost: H
design: true
pr: 6381
---

Filed by the REFS orchestrator (2026-11-02), from the fork `merged-parent-class-or-per-face` that Ev raised on PR 6342 (fork-log row 248). Both designers converged on **per-face exact**: a result face's name cites exactly the member faces whose area it holds, for the union, the intersect and the subtract. It replaces N2's class reading (PRs 3222 and 3734) and N3's union piece sentence. `design: true` stays until Ev rules on PR 6342.

**What the rule must state** (both designers):
- `Borders` is taken when a member face the result face holds is also held by another result face (the subtract's `in_merged` rule), never by a sibling count.
- "Holds area of" is read geometrically, counting a face that another member's coplanar face covers, or PR 3734's covered case returns.

**Parked on exact descent** (`a-union-whose-fold-glues-a-run-no-judgement-opened`): a record, per finished face, of which member faces' area it holds. Until that record exists, per-face would refuse more than the interim does: any glued face that a later member cuts.

**Done when:**
- the union's `Links`, its judgement clauses and `UnsupportedUnjudgedGlue` are gone;
- names README N2 and N3 state the per-face rule;
- the subtract's `name_boolean` and the union share one parent derivation;
- the pair-boolean row `pair-boolean-keys-a-merged-parent-by-the-exact-face-set` closes as not a defect;
- rows pin the designers' worked example:
  - tiles t1 and t2 flush with a bar g dividing their top, so L holds t1 and R holds t1 and t2;
  - adding a flush tile t0 that touches only L renames L alone;
  - moving g across x = 1 renames only the faces beside g.

Expect re-baselines of union fixtures in the merged-then-cut shape (PR 3222's `r1two`, `fam0**`, `r2endsg`).
