---
id: profiles-validation-refusals-print-two-margins-exactly
kind: issue
title: ProfileError's two scene-resolution arms print a MarginDiag through Display, exact and without a unit
status: open
opened: 2026-11-20
priority: P4
cost: E
---


Found by the told-quantity build's second sweep pass
(`the-told-quantity-type-and-profiles-refusals-on-it`), shaped at what
its gate cannot match: a scalar reaching a sentence through a bare `{}`.

`ProfileError::ArcBelowSceneResolution` and
`ProfileError::DirectionBelowSceneResolution`
(`crates/profile/src/validate.rs`, `impl fmt::Display for ProfileError`)
write `{margin}` and `{headroom}`, each a `geom_core::MarginDiag`
through its own `Display`: every digit of the float, and no unit word.
They are the only scalars `crates/profile` still tells outside
`geom_core::Told`. `MarginDiag::readable` tells a computed length; the
build did not move these two onto it because the sentences name no
unit, so which dimension the two margins are was not established.

`scripts/gates/told-quantity-fields.sh` covers `crates/profile` and
passes this: a `MarginDiag` field is not a bare scalar and `{}` is not
`{:?}`. This is the first instance of the class the per-crate sweep
(`patherror-display-renders-float-noise`, unit 3 onward) meets in every
crate: `MarginDiag`'s `Display` and `LowerExp` in a sentence.
