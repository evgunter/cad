---
id: a-ring-of-n-whose-member-turns-a-rational-cosine-leaves-seats-unproven
kind: issue
title: A ring of N whose step's cosine is rational at some member (N = 9, 24) leaves seats unproven: the exact-turn lane folds cos(turn/3) to −½ before the symbolic tier, so a constant angle has two canonical forms
status: closed
priority: P1
refs: [patterns-are-index-variables]
opened: 2026-10-28
pr: 6009
closed: 2026-10-29
---

Found by G's rule H measurement (`patterns-are-index-variables`; the
symbolic tier's angle sums, `crates/geom-core/src/sym/turn.rs`). D10
`[repetition]`, as sharpened on #5905, says a ring whose member spans
`turn/N` closes for any `N`; it does not for every `N`, so this is a gap
against ratified text.

A ring of 9 leaves four side seats unproven, `(2, 3)`, `(3, 4)`, `(5, 6)`
and `(6, 7)`: the seats either side of its members at a third and two
thirds of a turn. A ring of 24 leaves 16 of its 24. Pinned as a limit by
`g_ring_closure_measure::a_ring_of_nine_leaves_the_seats_beside_its_rational_members`
(`crates/editor-core/tests/g_ring_closure_measure.rs:247`), and measured
for 24 by that file's `measure_ring_closure`.

The cause: a member turned by a rational turn whose cosine is rational
reaches the tier with that cosine already folded to a rational value
(`trig_at_rational_pi`, `crates/geom-core/src/sym.rs:3386`, and the
exact-turn lane before it), so `cos(turn/3)` arrives as `−1/2` while its
neighbours arrive as angle atoms. A constant angle then has two
canonical forms, and the product it stands in is not an angle sum rule
H meets.

## Repair shape

One of two, the row saying which and measuring it:

- one canonical form for a constant rational-turn angle: keep it an
  angle atom until after linearisation, and fold it to a rational value
  only at the end; or
- the cyclotomic reduction: reduce constant `sin`/`cos` of `q·turn` in
  the cyclotomic field, so the two forms meet.

The limit row then asserts every seat proven for 9 and 24, and the
budget is re-measured as rule H's was.
