---
id: coefficient-ring-width-is-not-monotone-in-reach
kind: issue
title: widening the coefficient ring can lose discharges: a frozen node matches itself as one opaque indeterminate, and the same node expanded may not close
status: open
opened: 2026-09-14
---


**Filed by SYM-3's measurement** (`work/sym/rule-d-reaches-the-unit-bulge-only`,
"What stands (SYM-3)"), from two local, reverted patches on the bulge
fixtures (`m10_10_evidence_interval`, `CAD_M10_10_DOC=r1_segment_boss`
and the D-tab variants).

## What was measured

A node the early walk cannot build is FROZEN: it becomes an opaque
indeterminate of its own (`sym.rs`, "Freezing: the budget, and why it
is sound"), keyed on its content hash. Two spellings of one quantity
that both freeze at the SAME node therefore differ by the zero form —
soundly, since one node is one value — and an identity between them
discharges without the ring ever seeing what the node was. Widen the
ring and the node no longer freezes: both sides expand into forms the
ring must now actually close, and where it cannot, a discharge is
LOST.

- On R1's boss (`bulge = 2`) with `rational::COEFF_BITS` 256 → 512
  and the shipped rules, `carrier_matches_mapped_source` goes 72/0/48/6
  → 72/0/54/0 (all six through the door) and the ceiling `8.26e2·ε` →
  `9.36e2·ε`, onto `line_span`; nothing is lost there. But with an
  `abs(X) = X` fold for a manifestly non-negative `X` (or `abs(X)² =
  X²` in rule A's walk) at 256 bits, the boss's
  `carrier_matches_mapped_source` goes 72/0/48/6 → 72/0/38/16: ten
  decisions the door had closed while `abs(R)` was ONE opaque atom on
  both sides fall numeric once the atom is opened into `(5/8)·sqrt(L²)`
  and the ring freezes on the products. With both patches (the fold and
  512 bits) the same predicate is 72/0/50/4 — four numeric where the
  ring alone left none.
- On R2's parameter D-tab, the two patches together move the ceiling
  DOWN, `3.52e2·ε` → `2.82e2·ε`, and the over-band set at ceiling + δ
  changes predicate: `witness_at_mid_parameter` 1/11 where the shipped
  tree has `carrier_matches_mapped_source` 1/89. At 512 bits alone it
  is unmoved.

So the ring's width is not monotone in what the tier discharges, and
neither is opening an atom: a unit that widens `COEFF_BITS` or folds an
`abs` owes the full per-document split, not the one predicate it is
after, and a ceiling that MOVED DOWN under a "stronger" tier is this
mechanism before it is a bug. The measurement's numbers are on the
item; the renders in `crates/editor-core/tests/m10_bulge_renders.txt`.

## What would settle it

Either the ring closes the expanded forms (the products the boss's
on-surface residual needs are past 512 bits once the abs is opened —
`carrier_on_surface_2` 16 of 90 still numeric under both patches), or
the walk keeps an atom opaque where opening it buys nothing — which is
a policy the early walk does not have today (it folds whatever a rule
reaches). Not this program's next unit; recorded so the dual that takes
the bulge family's freezes reads its own split against this.
