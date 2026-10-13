---
id: a-seamless-cylinder-band-measures-in-closed-form-and-refuses-once-it-carries-an-ellipse
kind: issue
title: A seamless cylinder band measures in closed form and refuses in the quadrature once a rim is an ellipse
status: closed
pr: 4839
branch: flux/cylinder-band-quad
opened: 2026-10-10
priority: P3
cost: M
closed: 2026-10-13
---


## What

A cylinder face with no seam, bounded by two loops that each wind the
cylinder once (a band), measures in closed form: `geom_brep::props::curved`'s
`cylinder_chart` reads `−Σ ∮ v du`, a global 1-form, decides the face's
loops wind it zero times together (`props_chart_loops_closed`), and
`cylinder_holes` passes over a ring that winds (`props_ring_contractible`).

The same band with an ELLIPSE for a rim (a pipe cut by an oblique plane,
stated without a seam) routes to the quadrature, which reads `Σ ∮ u dv`.
That form is not anchor-free on a winding loop, and the lane refuses
each such loop at its first junction a period off
(`quad::require_no_period_jump`, `PropsError::ChartPeriodJump`, added by
`flux/ellipse-ring`): honest, and a stored-data defect by its Display,
though the band is valid input that the kernel's own writers avoid by
splitting at a seam (`step-import`'s `normalize::band_seam` re-mints
one). Found by review R2 of PR 4557 (S7); no body was built.

## The shape of a fix

The quadrature reads `−Σ ∮ v du` instead, as the closed form does: the
harmonic channels are symmetric in `u` and `v`, so the composite rule
over `v·u'` is the same rule, and a band's loops then need only the
closed form's zero total winding. Until then the refusal should read
as unbuilt, not as a defect, for a loop that winds.
