---
id: no-tighten-offer-quotes-a-value-below-the-runs-resolution
kind: issue
title: A zero-band margin's tighten offer quotes a value far below anything a run can resolve ("tighten the tolerance below 2.71e-28 m"), at a build and at rest
status: open
opened: 2026-10-08
priority: P3
cost: M
design: true
---


(Filed by the ENCL fix-pass implementer on PR 4331, `encl/adoption-at-rest-eps-in`. At that PR's import door a size within the file's ε_in is offered only together with declaring the file's uncertainty below it, so the same tiny value can appear there too. The class below is base behaviour at every other reading, and out of its scope.)

## What

A sized decision's band-decided Zero arm offers "if this {size} is intended, tighten the tolerance below m/K" for any nonzero margin m in the zero band, however small m is. `ftc11_uref_off`'s tangent-plane zero has a margin of about 2.71e-27 m at ambient 1e-9. Read at a build or at rest, its ending is therefore "Recourse: move the geometry so the surfaces cross at a clearer angle, or, if this angle is intended, tighten the tolerance below 2.71e-28 m".

No run can take that offer:
- D4 ¶4 fixes the model's units in metres with a documented size range.
- D4 ¶1 sizes the default ε at about 1e-9 m for micron-to-kilometre coverage with "~4 orders of f64 headroom at km scale" (`docs/DESIGN.md`, D4 ¶1).
- An ε nineteen decades below that resolves nothing inside the session box: f64 spacing at 1 m is about 2.2e-16 m.

So the offer is a number, not a recourse. The margin is rounding noise of an exact coincidence, and the honest Zero-arm ending is the lever alone. The decision's `at_zero` note is not the repair: it is written for a margin of no size, where no smaller tolerance decides it (`SizedWords::otherwise`'s contract), and a nonzero margin is not that case.

## Where

- `crates/geom-core/src/predicate.rs`, `MarginDiag::tightens_below` (predicate.rs:1400): it returns `band.tolerance_deciding(m)` for every nonzero point margin short of the band's far edge. A zero-band margin has no floor.
- The same file, `MarginDiag::sized_recourse` (predicate.rs:1457), which quotes that value, and `Band::tolerance_deciding` (predicate.rs:393), which is `|m|/K`.
- Readers: every `SizedDecision::recourse` at `Reading::Build` and `Reading::AtRest` (`crates/geom-brep/src/recourse.rs`), and so every certification Zero arm, topo's validator endings (`crates/topo/src/validate.rs`, `WedgeCheck::ending`) and the boolean's sized refusals.

## Repair shape

Withhold the offer where the value it quotes lies below what any admissible ε could resolve. That needs a resolution floor named once in geom-core, from D4 ¶4's size range and the f64 headroom, and it has to stay a sentence choice inside `sized_recourse`, within the reporting-margin fence (`real.rs`'s `Bounds` clause 2). The floor's definition is a design question, so weigh it before implementing.
