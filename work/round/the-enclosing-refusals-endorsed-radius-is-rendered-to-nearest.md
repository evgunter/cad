---
id: the-enclosing-refusals-endorsed-radius-is-rendered-to-nearest
kind: issue
title: EnclosesLegCarrier's endorsed largest_tangent_radius is rendered to nearest, so typing it back can land on the refusing side
status: open
opened: 2026-11-10
priority: P3
cost: E
---

Found by the refused-corner build unit's sweep (PR 6547's rendering
rule, "each value is rendered strictly on the fitting side of the exact
value").

`CornerReason::EnclosesLegCarrier`'s `Display` arm
(`crates/profile/src/path.rs`, the `Some(bound) => write!` arm under
`Self::EnclosesLegCarrier`) renders `largest_tangent_radius`, the
existence bound `(R₁ + R₂ − d)/2` its sentence endorses, through
`path::num`, which rounds to the nearest spelling on its grid. That
number is a radius an author types back, and at the bound itself the
tangent circle is the knife edge of the offset carriers meeting, so a
spelling that rounds UP lands back in the refusal: the hazard the
anchor-fit refusal's `radius_at_anchor` is now rendered against by
`path::num_toward(.., Toward::Below)`.

The sentence states a fact ("the largest circle tangent to both has
radius X m"), which a spelling below X would make slightly false, so the
repair is not a one-word swap: either the clause becomes a lever
("use a radius below X m") rendered by `num_toward`, or the fact stays
and the recourse names a fitting-side value beside it. Owed: that
choice, and a row that types the rendered value back into the corner
it came from.
