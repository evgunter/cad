---
id: a-sharp-plate-offset-over-a-rounded-one-refuses-unpaired-loose-ends
kind: issue
title: A sharp plate offset over a rounded one refuses its union Join(UnpairedLooseEnds) in both operand orders
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-08
---

Found by the dual review of PR 3846 (lanes r1 and r2) and measured on
that PR's branch after its merge of `origin/main`.

## Repro

The 6 × 4 × 1 sharp plate on the rounded one (`rounded(0.5)`, z 0 to 1;
`crates/sweep/tests/reach_continuation.rs`), the sharp plate moved
rigidly (`topo::transform_rigid`) by (dx, dy) in its own plane, every
flush finding declared (only the mating plane's `Rest` survives the
move; the walls are no longer one carrier):

| (dx, dy) | union, either order | A ∖ B, either order | A ∩ B |
|---|---|---|---|
| (0, ±1e-7), (0, ±1e-3), (±0.25, 0) | `Join(UnpairedLooseEnds { count: 8 })` | builds, exact | empty |

r2 also measured dx = −5.5 refusing `Join` in both orders on main and
on the branch. On `origin/main` before PR 3846 the sharp-first union
refused `CurvedPierceUnsupported` instead, so the two orders differed;
with the deferral both orders reach the same `Join` refusal.

## Not yet measured

Which stage raises it: the chord join's loose-end match, after the
declared-`Rest` zip (`boolean/rest.rs`) declined the configuration, is
the likely route but has not been instrumented. The sharp plate's
bottom edges now cross the fillets transversally (dy > 0) or miss them
(dy < 0), so this is a seam-matching question about a `Rest` patch whose
boundary mixes both operands' edges, not a tangency one.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: its repro reaches the chord join only after the declared-Rest zip declines; stage 4 retires that zip and moves its arms into the join, so the route is measured there. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-08, INTENT stage 4 A (`intent/s4-a-join`))

Every pose of the table builds in the join, in both operand orders, additive at tiers 3 and 3′: `reach_continuation.rs`'s `a_sharp_plate_offset_over_a_rounded_one_unions_in_either_order` ((0, ±1e-7), (0, ±1e-3), (±0.25, 0)), at the default, 1e-6 and 1e-12 rows; at 1e-12 the two poses 1e-7 off may refuse `Escalated` in the crossing layer instead (a margin of 1.07e-12 inside the band), and the row allows that and nothing else.
