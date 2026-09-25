---
id: decision-read-triples-the-plate-pin-suites-wall-time
kind: issue
title: the decision read's deep enclosure runs at every select and min/max node, and the plate's pin suite goes 142s to 535s in the dev profile
status: open
opened: 2026-09-21
priority: P2
cost: D
---


## What was measured (DECIDE-3, 2026-09-21)

`SymRules::decision_read` asks `signed::decision` at EVERY `Select`
node and `signed::order` at every `min`/`max` node the early walk
reaches, and each call encloses both halves of the decision form in
the outward-rounded ring, descending through `sqrt`/`abs`/`min`/`max`
atoms to `ENCLOSE_DEPTH = 8`. Where the read SUCCEEDS it pays for
itself; where it DECLINES — the common case, because most decisions
carry an indeterminate no bracket is known for — the enclosure has
already walked the sub-tree before it can say so.

Measured on this box (dev profile, 4 cores, `--test-threads=2`):

| suite | base (`e88987915`) | DECIDE-3 | ratio |
| --- | --- | --- | --- |
| `m10_10_pins_interval` | 142.97 s | 534.93 s | 3.7x |
| `m10_8_pins_interval` | 106.39 s | 144.82 s | 1.36x |
| `m10_9_pins_interval` | 134.27 s | ~81 s (fewer refusals to report) | 0.6x |

On the ceiling-bisection instrument the per-probe costs are unmoved to
the digit at the plain tier and A0 alone (plate 0.18 s / 0.24 s,
bracket 0.61 s), because the read is early-walk only; the shipped
tier's bracket probe is where it shows.

## What would answer it

The cheap tests first, in this order, before any enclosure is built:

1. **The enclosability pre-pass rule C already has.** `signed::fold`
   refuses in one pass over the ids when the form carries an
   indeterminate that is neither a parameter nor π. The decision read
   needs the same test widened by one level — "a parameter, π, or an
   atom whose own arguments pass this test" — run as a cheap
   id-walk before `enclose_deep` allocates a single interval.
2. **Memoize the enclosure per form digest** inside the session. A
   frame's conditioning floor is asked at every component of every
   vector built on it, and the argument forms repeat exactly.
3. **A depth-1 attempt first**, widening only where it straddles.

None of these changes what the read DECIDES, so the pins hold across
them, which is what makes this a follow-up and not a blocker.

## Home

`crates/geom-core/src/sym/signed.rs` (`enclose_indet`, `enclose_deep`,
`decision`), `crates/geom-core/src/sym.rs`'s `combine`. Filed by
DECIDE-3's lane with the measurement above.


## SYM-9 read this row and left it where it is (2026-09-22, re-taken 2026-09-24)

SYM-9's A1 amendment made this row ride as far as that unit's cost
table: if the decision read dominated the retry ladder's cost on any
document, Phase 2 was to take this row's first cheap answer (the
enclosability pre-pass) and re-baseline the suites' wall times.

It does not, and that is now executed rather than argued. On the
affordability line's instrument (one whole-box leaf, release, fastest
of three takes), the shipped rules against `SymRules::without_the_reads`,
each with `SymRetry::kept_atom` and without it:

| document | read on: one attempt / ladder | read shut: one attempt / ladder |
| --- | --- | --- |
| R2's filleted bracket (`1e1·ε`) | 2.86 / 3.88 s | 2.82 / 3.85 s |
| R2's link (`1e1·ε`) | 17.28 / 19.71 s | 17.57 / 19.81 s |
| R2's rounded pad (`1e2·ε`) | 131.3 / 147.7 s | 132.8 / 148.6 s |

On this instrument and these three documents the read costs nothing
measurable, with or
without a ladder. This row's own measurement is a different one — the
pin SUITES' wall time in dev — and nothing here contradicts it; what it
says is that the leaf a drive pays is not where the read's cost lands on
these three documents. The row stays open at P2 with its three cheap
answers in the order it names them.
