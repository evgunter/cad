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
- On R2's parameter D-tab the pair `abs(X)² = X²` in rule A's walk
  (patch A on the item) WITH the 512-bit ring moves the ceiling DOWN,
  `3.5218e2 → 2.8211e2 .. 2.8222e2·ε`, and the over-band set at
  ceiling + δ changes predicate: `witness_at_mid_parameter` 1/11 where
  the shipped tree has `carrier_matches_mapped_source` 1/89 —
  re-measured at the fix pass from the recorded diff, 16.8 s a probe.
  The pair `abs(X) = X` on a syntactically non-negative `X` (patch C)
  with the 512-bit ring leaves the bracket bit-identical to shipped
  (`3.5218e2 .. 3.5232e2·ε`, `carrier_matches_mapped_source` 1/89,
  14.4 s a probe), and so does the ring alone — though at the nominal
  the two pairs print the same table (`carrier_matches` 126/0/38/16,
  `carrier_endpoint_start` 24/0/12/0 with the rim identity closing).
  So the fall is patch A's own: a square substituted as a form over
  the box carries the box's width into a predicate the atom had kept
  out of it, and the wide ring then holds that form where the narrow
  one froze it.

**The ring's cost** (dev build, one whole-box probe of the bisection,
256 → 512 bits): boss 0.81 → 3.17 s (3.9×), parameter D-tab 1.45 →
13.2 s (9×), the `0.5` parameter control 7.9 → 26 s under a
fold-plus-512 pair; the literal D-tab 0.64 → 0.66 s (its forms freeze
on constants either way).

So the ring's width is not monotone in what the tier discharges, and
neither is opening an atom: a unit that widens `COEFF_BITS` or folds an
`abs` owes the full per-document split AND the bracket, not the one
predicate it is after, and a ceiling that MOVED DOWN under a
"stronger" tier is this mechanism before it is a bug. The measurement's numbers are on the
item; the renders in `crates/editor-core/tests/m10_bulge_renders.txt`.

## What would settle it

Either the ring closes the expanded forms (the products the boss's
on-surface residual needs are past 512 bits once the abs is opened —
`carrier_on_surface_2` 16 of 90 still numeric under both patches), or
the walk keeps an atom opaque where opening it buys nothing — which is
a policy the early walk does not have today (it folds whatever a rule
reaches). Not this program's next unit; recorded so the dual that takes
the bulge family's freezes reads its own split against this.
