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
  So the fall is patch A's own, and its mechanism is this row's
  thesis: rendered at the fallen ceiling, the bounding decision's
  early form is `sqrt(?#…)` over a `Powi^2` node that is FROZEN even
  at 512 bits — the opened `abs²` product does not fit the ring, so
  the discharge the opaque atom gave is lost and the numeric channel
  decides at the box width (the delta review's render, `likely`).

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

## A second mechanism: SYM-5's rule E scale step (2026-09-14)

Found by SYM-5 PR-2's review R2, by execution. The ring's width is not
only reached by a document's own coefficients: a RULE can widen them.
Rule E (the quotient's common factor, `SymRules::common_factor`)
canonicalises a quotient's scale by multiplying both halves by `1/|s|`,
with `s` the denominator's coefficient at its smallest monomial — so
every numerator coefficient gains `s`'s odd part in ITS denominator,
up to `s`'s bits wider.

The demonstration, `geom-core/tests/sym_rule_e_rows`'s
`rule_e_can_cost_a_theorem_to_the_coefficient_ring`: with
`P = (x + |1/q|)/(x + 0.1)` and `q = 3^126` (200 bits), the unscaled
`P` lets rule A substitute and the product by `h` fit
`rational::COEFF_BITS`; scaled by `1/0.1` (52 bits) `P`'s coefficients
are 252 bits, the product needs 259, and BOTH sides freeze to two
different indeterminates. `without_rule_e` reaches the theorem;
`shipped` does not.

So rule E is monotone in TERMS and in DEGREE and not in the third
budget, and "no measured document lost a decision" is a measurement on
the documents SYM-5 names and never a structural fact.

**The next shape, not taken.** A scale choice that provably adds no
bits would close it: scaling only when the pivot coefficient is a
DYADIC UNIT (`±2^k`, whose reciprocal adds nothing to any
coefficient's odd part) keeps the canonicalisation for every quotient
whose pivot is a power of two and leaves the rest alone. That is a
narrower canonical form — two spellings whose pivots differ by a
non-dyadic factor would stop meeting — so it trades reach for width
and wants its own measurement before it is taken.

## A third measurement: SYM-8's rule F (2026-09-15)

This row's `abs` patches are re-taken, against the narrower predicate
SYM-8's rule F uses. Patch C folded `abs(X) = X` for a SYNTACTICALLY
NON-NEGATIVE `X` and lost ten decisions on R1's boss with patch A at
256 bits; rule F folds only where `X` is manifestly POSITIVE — every
term non-negative and at least one term strictly positive, with
`sqrt`/`abs` atoms counting as positive only when their own argument
is. The boss's atom is `abs((5/8)·sqrt(L²))`, whose `sqrt` is over a
BARE SQUARE: non-negative, not positive, so the predicate declines it.

Measured (the split at the nominal, whole, rule F off → on):

| document | rule F off | rule F on |
| --- | --- | --- |
| R1's segment boss at `bulge = 2` | the shipped table | **bit-identical**, every row |
| R2's D-tab, bulge a literal `0.4` | the shipped table | **bit-identical** |
| R2's D-tab, bulge a parameter | the shipped table | **bit-identical** |
| its whole-certifying ceiling (boss) | `9.3559e2 .. 9.3595e2 · ε` | identical to the digit |

So the narrowed predicate does not re-take the loss this row recorded:
rule F never fires on the bulge family. `sym_rule_f_rows`'s
`abs(sqrt(t²)) − sqrt(t²)` row is the pin that keeps it declining.

**But the class is alive, and rule F pays it once.** On R2's rounded
pad, at the scale it certifies whole at, the same 1953 decisions split
`symbolic_zero: 858, registered: 104, numeric: 991` with the rule off
and `854 / 128 / 971` with it on — 24 decisions into the door, twenty
out of `numeric` and FOUR out of `symbolic_zero`. `frozen` is 2750 at
both dials and no ceiling on any of the eight measured documents moves
by a digit. Those four are this row's mechanism exactly: the early walk
was cancelling OVER an opaque `abs` atom, opening it expands both sides
into forms the ring must now close, and where it cannot the theorem is
lost — here the registry re-takes them, so no decision is lost, but the
claim is weakened from a theorem the tier proved to an axiom a
constructor stated. A unit that folds an atom owes this reading, not
only the predicate it is after; SYM-8's is the third.
