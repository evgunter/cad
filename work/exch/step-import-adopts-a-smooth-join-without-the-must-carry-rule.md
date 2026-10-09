---
id: step-import-adopts-a-smooth-join-without-the-must-carry-rule
kind: issue
title: the adoption ladder offers no conventional rung for an under-determined tangency between distinct surfaces, so a native body with one does not re-import
status: open
opened: 2026-10-09
priority: P2
cost: M
refs: [split-tangent-chord-mints-tangency-without-the-must-carry-rule]
---

## Finding

Found by the §5 sweep of the PR that routed the split's tangent
section chord through `geom_brep::must_carry_over_edge`
(`work/tang/split-tangent-chord-mints-tangency-without-the-must-carry-rule.md`).
That rule is the one home of what a definitely-smooth join stores:
jet-determinate ⇒ `TangentIntersection`, under-determined ⇒ a chart
image, in band ⇒ a typed refusal.

`crates/step-import/src/adopt.rs`'s edge ladder (the candidates loop
that ends in `StepImportError::Adoption`) does not ask it. It offers
`Intersection`, then `TangentIntersection`, then the conventional
`MappedCurve` rung only where the two surface records COINCIDE (the
M7-2 rung, which is right to refuse to offer a chart image wherever an
intersection refuses). A smooth join between two distinct,
non-coincident surfaces that the rule calls under-determined therefore
has no candidate that certifies, and the import refuses a body the
kernel built tier-3 valid.

## Measured

The rounded shoulder of `crates/sweep/tests/wedge_end_doors.rs`
extruded `h`, split by `shoulder_cut()` (`y = 1`), its `y < 1` piece
written by `step_export::step_string` and read back by
`step_import::import_step` with default options, at `Tol::witness()`.
Since the PR above, the piece's seam is a chart image on the section
plane wherever `h²/2 ≤ ε`:

| h | outcome |
|---|---|
| 1, 1e-3 | imports |
| 4e-5, 1e-5 | `Adoption { attempts: [Intersection: NotTransverse (Zero), TangentIntersection: NotSecondOrderSeparated (Zero, 8e-10 / 5e-11)] }` |

`import_step`'s own contract says files `step_string` writes from
finished kernel bodies import cleanly.

## Fix shape

The rung for a definitely-smooth edge between two distinct surfaces is
the rule's answer over the carrier: `must_carry_over_edge` read at ε on
the resolved surfaces, `Conventional` offering the chart image
(certified like any candidate, so a wrong surface still refuses), an
in-band verdict refusing typed rather than falling through. Which
adjacent chart, and how ε_in enters (D7: ε_in interprets, ε builds),
is this program's call.
