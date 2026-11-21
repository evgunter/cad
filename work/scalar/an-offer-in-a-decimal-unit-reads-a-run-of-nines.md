---
id: an-offer-in-a-decimal-unit-reads-a-run-of-nines
kind: issue
title: An offered bound in a unit whose factor is not a power of two reads a run of nines: is the side that works the typed float's or the decimal's
status: closed
opened: 2026-11-20
priority: P1
cost: M
parent: patherror-display-renders-float-noise
rides_with: a-number-in-a-unit-is-rounded-once
closed: 2026-11-21
---


`crates/geom-core/README.md`, "A number told to a person", says an
offered bound is "the shortest spelling on the computed grid that
lies on the side that works when typed". `geom_core::Told` reads
"when typed" as the text door's arithmetic: the decimal parsed to a
float and multiplied by the unit's factor
(`crates/geom-core/src/told.rs`, `toward` and `typed`). In a unit
whose factor is not a power of two that product is rounded, so a
round number of the unit can type back one float past the bound, and
the offer is then the grid point before it:

- `at most 0.00013 m` reads `0.1299999 mm`;
- `at most 0.30004 m` reads `300.0399999 mm` (`300.04 * 0.001 >
  0.30004`), pinned in `told::tests::a_quantity_reads_in_the_readers_units`.

**Measured** by PR 7229's review over round metre bounds: 20% of
`at most` and 6.5% of `at least` bounds read this way in millimetres;
centimetres are similar; the inch and the degree have no such class.
`told::tests::a_round_metre_offer_in_millimetres_reads_its_neighbour_below_a_fifth_of_the_time`
is the register row for the millimetre rate.

## The question

Is "the side that works when typed" the side of the **float the text
door produces** (today: never offers a number that types back past
the bound, at the cost of the nines), or of the **decimal as a real
number** (`300.04 mm` is exactly the bound's own decimal, and whether
its typed float lands a unit in the last place past the bound is the
door's rounding, which a tolerance-sized margin absorbs)? The first
is what a certified bound needs if the kernel compares the typed
float against the bound exactly; the second is what a person reads.
A third: offer in the unit only where the round number types back on
the side, and step by a whole grid step otherwise.

## Decided

Ev, PR 7261 (2026-11-21): settled by `a-number-in-a-unit-is-rounded-once`, which this row rides with and which builds the answer.

## Closed

Built by `a-number-in-a-unit-is-rounded-once`: `300.04 mm` now types back as the float nearest 0.30004 m, so `at most 0.30004 m` reads `300.04 mm` and `at most 0.00013 m` reads `0.13 mm`; `told::tests::a_round_metre_offer_in_millimetres_reads_its_own_decimal` pins that a round metre bound offered in millimetres is its own decimal there, both sides.
