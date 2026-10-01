---
id: coefficient-ring-width-is-not-monotone-in-reach
kind: issue
title: widening the coefficient ring can lose discharges: a frozen node matches itself as one opaque indeterminate, and the same node expanded may not close
status: closed
opened: 2026-09-14
priority: P0
cost: D
closed: 2026-09-24
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

**And it is not rule E's hazard twice.** Rule E's loss is demonstrated
at the SCALAR — `sym_rule_e_rows::rule_e_can_cost_a_theorem_to_the_coefficient_ring`,
a hand-built residual at a 200-bit coefficient — and no measured
document pays it. Rule F's is realised ON a measured document, at the
scale that document certifies whole at. Both reviews of SYM-8 read the
spec's Phase-1.3 stop clause as literally tripped by it; the SYM
orchestrator ratified the ship-on as a spec deviation on 2026-09-21
(`work/decide/SYM-8.md`). The guard that was missing is in:
`m10_9_pins_interval`'s `Study` now pins `symbolic_zero` beside
`registered` on all five documents, so the next four theorems to leave
red rather than living in a comment.


## What SYM-9 answered (2026-09-22, fix pass 2026-09-24)

This row's thesis — that the ring's width is not monotone in reach, and
neither is opening an atom — is what SYM-9 built a RETRY LADDER on
rather than a wider ring (`geom_core::SymRetry`, and the unit's PR).
The ladder cannot pay this row's cost, because a retry is asked only
where every rung of the first attempt declined: the first attempt is
identical with the ladder installed and without it, so `numeric` can
only fall and no discharge can be lost. That is the row's mechanism
turned into a design constraint rather than worked around.

**The measurement, per shape, as a RETRY at the nominal, on five of the
six documents** (the dev nominal replay; R2's rounded pad does not
return one on the measuring box —
`work/sym/the-pads-nominal-replay-is-not-takeable-on-a-four-core-box`):

| shape, as a RETRY | plate | annulus | boss | bracket | link |
| --- | --- | --- | --- | --- | --- |
| the ring at 512 bits | 0 | 0 | 0 | **6** (4.41x) | **8** (1.29x) |
| the ring at 1024 bits | 0 | 0 | 0 | **19** (11.57x) | 8 (1.28x) |
| rule A shut | 0 | 0 | 0 | **6** (1.11x) | 0 (1.04x) |
| rule G shut | 0 | 0 | 0 | 0 (1.25x) | **12** (1.14x) |
| rule A and rule G shut in ONE mask | 0 | 0 | 0 | 6 (1.05x) | **0** (1.07x) |
| rule E shut / rule F shut | 0 | 0 | 0 | 0 | 0 |
| the companion rewrite shut / the magnitude door shut | 0 | 0 | 0 | 0 | 0 |

The ratio is one whole nominal replay against the same replay with the
ladder off, a dev build on a four-core box. The pad is measured on the
OTHER instrument, one whole-box leaf in release at `1e2·ε`, where its
receipt is `[890, 6, 150, 907]` with the kept-atom ladder and without
it: nothing recovered, at 131.3 → 147.7 s.

**At 512 bits the ring recovers, predicate by predicate, no more than
the kept-atom attempts do**, at four times the cost on the bracket. At
1024 bits it recovers THIRTEEN more decisions on the bracket than the
kept-atom attempts (19 in all, `tangent_on_surface_2` `[9, 0, 0, 9]`
-> `[18, 0, 0, 0]` among them), at 11.57x — reach the ring has and the
kept atom does not, at a price no default pays.

**The two kept-atom shapes are not composable in one mask** — rule A
and rule G shut together keep the bracket's six and lose all twelve of
the link's — so `SymRetry` carries one mask per shape (the free const
`geom_core::sym::MASKS`). Why the joint mask loses the twelve is not
executed on the link; the working hypothesis is that they close
through rule A's `sqrt(X)² = X` once rule G has stopped re-keying the
atom, and the render of those residuals under both masks is what would
confirm it
(`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`,
shape 2).

**Rule E's scale step — the second mechanism above — is RECOVERABLE by
a retry at the scalar and costs no measured document anything a retry
recovers.** SYM-9's `geom-core/tests/sym_9_retry_rows.rs`
(`a_fewer_rules_retry_closes_the_rule_e_loss`) drives the hand-built
residual above through a retry with rule E shut and it closes as a
theorem; rule E shut as a retry recovers zero on the five documents.

**What ships.** `SymRetry::kept_atom`, ON by default, across the 1.6 s
line as a disclosed trade, the way rule E shipped: on the affordability
line's instrument the ladder acts only on documents already over the line
at one attempt per rung, adds 12.5–36 % there, changes no certification,
and its rule-G attempt returns the ten theorems the default rule G costs
R2's link (`editor_core::drive::DEFAULT_SYM_RETRY` carries the table;
the rule-A attempt is the weaker half, six registrations and no theorem).
The fix pass first set the default to none on the leaf line and the
delta review reversed that; the leaf numbers stand either way.

**What is left open here.** The pad's nominal replay is unmeasured
(its own row), and the ring's non-monotonicity is now a DESIGN
CONSTRAINT the ladder respects rather than a defect anything closes —
the row stays open as the record the next unit that reaches for
`COEFF_BITS` reads its own split against, which is what it was filed
as.

## Closed (2026-09-24)

Closed at SYM-9's merge (#3083, into `props/sign-hull`). The row's
second way to settle it — "the walk keeps an atom opaque where opening
it buys nothing" — is what shipped: a retry ladder that leaves the first
attempt exactly as it was and asks a kept-atom attempt only into its
silence, so the non-monotonicity this row measured can no longer lose
a discharge. The ring's width stays a dial (`SymRetry::ring`) with its
table above: at 1024 bits it reaches thirteen bracket decisions the
kept atom does not, at a price no default pays. A unit that reaches for
`COEFF_BITS` still owes the full per-document split and the bracket;
this closed row is the record it reads.
