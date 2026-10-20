---
id: a-far-frustums-closed-form-volume-reports-a-zero-pad
kind: issue
title: A far frustum's closed-form volume reports volume_pad 0 though its translated volume moves by rounding
status: open
opened: 2026-10-20
priority: P3
cost: E
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
