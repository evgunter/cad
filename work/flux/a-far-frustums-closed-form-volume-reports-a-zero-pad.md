---
id: a-far-frustums-closed-form-volume-reports-a-zero-pad
kind: issue
title: A far frustum's closed-form volume reports volume_pad 0 though its translated volume moves by rounding
status: closed
opened: 2026-10-20
priority: P3
cost: E
closed: 2026-10-21
branch: flux/far-frustum-pad
pr: 5576
---

Found by PR 5156's review (germ, `circular-axes-band-gap`), disclosed
as off that PR's target; measured in its fix pass.

`topo::mass_properties` on the `sweep` `near_circular_sections`
frustum of scale 100 (`r0 = 100`, `r1 = 50`, height 100, revolved
about `y`) returns `(1832595.714594046, volume_pad 0.0)` as built and
turned about a generic axis, and `(1832595.7145940464, 0.0)` carried
`(700, −500, 400)` off: the two readings differ by an ulp, so at least
one of them is not the exact volume its zero pad claims. A zero pad
means "closed form, exact" (`fillet_h7_transverse_cap.rs` `volume`
asserts it), but a closed form evaluated at far coordinates carries
their rounding.

Owed: either the closed-form lane states its rounding as the pad
(scaled to the body's coordinates, not its size), or the zero pad's
contract is restated as "closed form, rounding not stated" and the
rows that read it as exact are audited.

## Resolution (FLUX #2, PR 5576)

The zero pad is restated, not widened: `volume_pad == 0.0` means every
face took a closed form, and the value carries its closed forms' rounding,
which no pad states (`topo::MassProperties`, GUIDE §2.4). The far
frustum's move was the rigid motion rounding its vertices, which changes
the stored geometry; it was not the closed form's rounding. A reader that
decides on a zero-pad value re-derives it or decides on an enclosure. The
backstop's readers that don't are filed on CLEAVE
(`the-backstops-magnitude-arm-meters-the-f64-margin-after-the-interval-has-cleared-it`,
`the-backstop-reads-an-unpadded-f64-volume-as-exact-where-it-passes-or-switches-off-a-bound`).
The frustum row
`near_circular_sections::a_frustums_zero_pad_carries_its_closed_forms_rounding`
bounds the error, as built and turned, at `4·u·|V|`.
