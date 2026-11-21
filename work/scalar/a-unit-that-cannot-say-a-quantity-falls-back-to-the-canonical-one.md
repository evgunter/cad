---
id: a-unit-that-cannot-say-a-quantity-falls-back-to-the-canonical-one
kind: issue
title: A quantity its notation's unit cannot say is told in the canonical unit, which the README does not provide for, and it happens at ordinary magnitudes
status: closed
opened: 2026-11-20
priority: P1
cost: M
parent: patherror-display-renders-float-noise
rides_with: a-number-in-a-unit-is-rounded-once
closed: 2026-11-21
---


`geom_core::Told` renders in the reader's notation, and where the
notation's unit cannot say a quantity under its role's rule it is
told in the canonical unit under that unit's word
(`crates/geom-core/src/told.rs`, `Quantity::said`; the module docs,
"Whose notation"). `crates/geom-core/README.md`, "A number told to a
person", does not provide for this: it says a person working in
millimetres reads a refusal in millimetres.

The cases: a **given** number no number of the unit types back to
(the multiply by the factor steps over floats); an **offered** bound
no spelling in the unit lands inside the window of; a **compared**
pair no precision in the unit separates, or whose members are of two
dimensions or one of which names the tolerance; a value the factor
carries out of the format.

**It is reached at ordinary magnitudes**, measured by PR 7229's
review: 2.3% of 12-digit given lengths near 0.1 m in millimetres,
8.5% of given angles in degrees, 3% of ordinary distinct compared
pairs.

## The question

What does a reader in millimetres read when millimetres cannot say
it exactly? The candidates: the canonical unit, as now (always true,
and a sentence in two units); the nearest number of the reader's
unit, marked as not exact; for a given number, the exact decimal of
the unit at more than seventeen digits, which no `f64` door reads
back; or a ruling that "exactly what was received" is a statement
about the canonical value and a given number in another unit is told
to seventeen figures.
`two-quantities-of-one-sentence-told-apart-can-read-in-two-units` is
what the fallback does to a sentence.

## Decided

Ev, PR 7261 (2026-11-21): settled by `a-number-in-a-unit-is-rounded-once`, which this row rides with and which builds the answer.

## Closed

Built by `a-number-in-a-unit-is-rounded-once`: a number in a unit is its decimal times the unit's exact factor rounded once (`quantity::UnitDef::canonical`), every finite float has a decimal of at most seventeen figures in every unit that reads back as it (`UnitDef::shortest`), and `geom_core::Told` spells decimals of the unit, so no role, pair or position falls back (`told.rs`, `Quantity::spelled_in` returns `None` only for a reading that does not read). A value whose number of the unit is past the float format is said in the unit (`1e306 m` reads `1e309 mm`).
