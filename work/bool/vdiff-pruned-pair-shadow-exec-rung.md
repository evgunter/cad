---
id: vdiff-pruned-pair-shadow-exec-rung
kind: issue
title: Banked — verdict-recovery rung for pruned-pair vanish diagnoses (shadow-exec on demand)
status: open
opened: 2026-07-29
github: 134
refs: [BOOL-7]
---

## From GitHub issue 134

opened 2026-07-29, 0 comments.

Banked at the PR 8 rulings (Ev, in-session 2026-07-29, option (a) on the N5 diagnosis question): the C10 sweep legitimately prunes candidate pairs, so their predicates record no verdicts (−51% of the log on the corpus), and a vanish whose flip evidence lived on a pruned pair diagnoses to the documented evidence-free fallback (RecipeEdit{NodeChanged}) instead of PredicateFlip. Recording pruned-pair pseudo-verdicts is out (reintroduces the quadratic in space).

The rung: when the vdiff engine hits an empty pair population on a vanish, SHADOW-EXECUTE exactly the vanished pair's predicates from the prior evaluation's context (both sides as needed) and diff those — bounded work, paid only at diagnosis time, recovers the full PredicateFlip diagnosis. PERF-PLAN's shadow-exec scalpel is the same pattern.

Not scheduled; a candidate M5-adjacent or M6-era unit. N5's amended text (PR 8) references this issue as the front door that does not exist yet.

## Home

S-BOOL: `crates/editor-core/src/resolve/vdiff.rs` is in the program's `paths` territory, and the issue is already scheduled there as the unit `BOOL-7` under Ev's 2026-09-01 assignment (M10 dormant).
