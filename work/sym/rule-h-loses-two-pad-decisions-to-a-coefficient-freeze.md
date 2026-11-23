---
id: rule-h-loses-two-pad-decisions-to-a-coefficient-freeze
kind: issue
title: rule H loses two carrier_on_surface decisions on R2's rounded pad: its linearised multiple-angle form of a rounded pi/2 crosses the coefficient bound at a square the polynomial form fits
status: open
opened: 2026-10-30
priority: P2
cost: M
---


Found by INTENT's explanation of G's slow-row moves
(`work/intent/g-moves-four-main-reds-messages.md`). Rule H is
`SymRules::angle_sum` (`crates/geom-core/src/sym/turn.rs`). It landed with
INTENT stage 3 G (PR 5954, merge `cdef4469`, first parent `bc30e8a3`).

## What was measured

The document is R2's rounded pad with its first leg spelled as an angle
(`m10_8_r2_probes_interval::pad_first_leg_by_angle` at `10·ε`), driven as
one whole-box leaf. That is the leaf `sym_9_retry_interval`'s
`sym_9_the_drive_writes_the_ladders_receipt` pins
(`crates/editor-core/tests/sym_9_retry_interval.rs`). Both readings were
taken on `cdef4469` at the default ε row.

| dials | `[symbolic, numeric, retried]` |
| --- | --- |
| shipped, no ladder | `[1427, 1416, 0]` |
| `SymRules::without_angle_sum()`, no ladder | `[1429, 1414, 0]` |

The second reading is what `bc30e8a3` reads, so rule H accounts for the
whole move. The shipped ladder's `[1463, 1380, 34]` → `[1461, 1382, 34]`
is the same two decisions, and no retry recovers them.

The two decisions are one `carrier_on_surface_1` and one
`carrier_on_surface_2`, each going from 194/22 to 193/23. Every other
predicate's split holds. They are decisions 1712 and 1754 of the leaf
replay (`m10_8_arc_family_interval::replay_retried` over
`ParamBox::of(analyzed)`), and both go from `Symbolic` to
`NumericZero`.

## The mechanism

- The leg's `Angle(π/2)` reaches the tier as the f64
  `884279719003555·2^-49`. That is a rational number of radians, so rule
  H classes its `sin`/`cos` atoms as symbolic (`turn`'s module docs, "The
  canonical argument"). This is correct: the rounded value is not `π/2`.
- The product rewrite then linearises products of those atoms into the
  multiple-angle atoms of `2θ`, `3θ` and `4θ`. The residuals carry
  `cos(-884279719003555·2^-47)`, `sin(2652839157010665·2^-49)`,
  `sin(884279719003555·2^-48)` and their kin. Rule B's
  `sin² = 1 − cos²` keeps the same quantities as a polynomial in
  `sin θ` and `cos θ`.
- In both residuals the early walk freezes one `Powi ^2`. Its base has 36
  numerator terms (degree 4) and 18 denominator terms (degree 3). The
  profile's freeze site gives the cause as `Coefficient`, meaning an
  integer past `rational::COEFF_BITS` (256). With rule H shut that
  freeze does not occur.
- The other square in the residual (`Powi ^2 #3bc23967`, a 6-term base)
  builds. So the residual reads
  `2^62·X + 2^62·X·sqrt(1 + select(…)) − …`, where X is the frozen node.
  No form cancels that, and the numeric channel answers `Zero` inside
  the band.

## Why it is a defect

Rule H is an identity rewrite. The row for it in `SymRules::shipped`'s
table claims gains on the bracket, the link and the literal D-tab and
says "every other split on the eight documents unmoved". That was
measured at the nominal. On the pad's whole-box leaf it loses two
decisions, and DECIDE-3's exemption from never-loses names
`carrier_matches_mapped_source` on the bracket alone.

## Leads, unmeasured

- Linearise only pairs of constant-class atoms. A ring's step needs
  only those, but symbolic pairs are what this document's residual
  linearises.
- Or keep a node's pre-rewrite form wherever the rewritten one later
  freezes on the ring. That is harder, because the freeze is downstream
  of the node that rewrote.

Either change re-measures rule H's cost-table row.

## Same mechanism elsewhere, no decision lost

The `sweep` thread-count table's `sym_arc_loft validate_geometric
chk=99/104` → `97/104` at ε 1e-12 is the same kind of event. Rule H's
form of a `Mul`'s kid crosses the coefficient bound at that row only,
and two contradiction checks read the frozen node. No decision moves
there. The explanation is at that table's pin
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`,
`expected`).

## Closing it

Once the pad's two decisions discharge under the shipped rules, re-pin
`sym_9_the_drive_writes_the_ladders_receipt`'s two receipts. Both cite
this row.
