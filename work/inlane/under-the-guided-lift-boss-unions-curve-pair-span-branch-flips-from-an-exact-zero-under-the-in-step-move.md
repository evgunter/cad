---
id: under-the-guided-lift-boss-unions-curve-pair-span-branch-flips-from-an-exact-zero-under-the-in-step-move
kind: issue
title: Under the guided lift, boss_union's curve_pair_span_branch margin is exactly zero at the nominal and pi at p1, so a 1e-7 in-step move flips it (#5749) and curve_pair_span_end_side decides another margin after it
status: open
opened: 2026-10-17
priority: P2
cost: M
---


Found by the shared replay session unit (`inlane/sym-replay-session`), guided lift, in-step move (every variable by the same relative 1e-7).

- **#5749 `curve_pair_span_branch`:** margin 0.0 exactly at p0, 3.141592653589793 at p1. The sign flips: a branch angle sits exactly on its cut at the nominal and jumps a half turn.
- **#5752 `curve_pair_span_end_side`:** p0 −0.7330382858376184, re-valued at p1 −1.8325958978536172, p1 build 0.3665191795707234. #5753: p0 1.4660765716752366, re-valued 2.565634256995064, p1 0.36651917957072355. After the flip the builds decide this predicate over different margins: two margins under one name, not a laundering.
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=boss_union cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

The gating row `revalue_boss_union` pins exactly this, through its `flips` entry. The flipping predicates are {`curve_pair_span_branch`, `curve_pair_span_end_side`}, and the predicate re-valued away after them is {`curve_pair_span_end_side`}. The row goes red if the flip goes, so the entry comes out with this fix, and also if any other decision flips.

**Owed:** decide `curve_pair_span_branch` away from its cut at a held configuration, by name, or at a margin that is not exactly zero at the nominal. Then take the entry out.
