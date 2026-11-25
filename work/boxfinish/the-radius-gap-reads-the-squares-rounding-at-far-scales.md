---
id: the-radius-gap-reads-the-squares-rounding-at-far-scales
kind: issue
title: geom_core::radius_gap reads one ulp of the squared norm over twice the radius where norm − radius is exact: f64 decisions move at far scales
status: dispatched
opened: 2026-11-24
priority: P1
cost: M
branch: boxfinish/signed-radius
---


Found by both reviews of PR 7686 (the signed radius, 2026-11-24),
demonstrated; the function is on main since PR 7370 (the radial
readers), so it is that unit's defect too.

`geom_core::radius_gap` spells a measured norm less a locus's radius as
`(‖w‖² − r²)/(‖w‖ + r)`, so that a vector built at the radius's length
reads the zero form at the symbolic scalar with nothing known of the
radius's sign. At `f64` the numerator keeps the squares' rounding: one
ulp of `‖w‖²` over `2r`, about `r·2⁻⁵²`, where the root form
`‖w‖ − r` is exact near the locus (and exactly 0 where the radius is
the measured norm). Its doc's `f64` sentence ("`norm − radius` to
rounding") claims more than that.

**Measured on PR 7686's head, where arcs read through it:**
- `profile`
  `a_far_arcs_lens_reads_its_own_width::a_far_arcs_lens_past_ten_eps_wide_validates`
  is green on the base and red at ε 1e-12
  (`ArcBelowSceneResolution { check: OnCarrier, margin 6.1e-11 }`, a
  lens of radius 1e6); one line back to `rim − radius` greens it.
- At the default ε, 71 of 5,040 generated `f64` documents change stage,
  each an arc of radius about 4e6 or more: the path door refuses where
  the base stored the loop.

**Not measured:** main's own readers through it (PR 7370's cylinder and
sphere wall distances and the pcurve cache) at far scales.

**Ruled (the orchestrator, with PR 7686's fix pass):** a reading
re-posed for the symbolic tier does not cost the numeric channels
their accuracy. One generic spelling at the one home that is the root
form's accuracy at `f64`, no wider than the difference at `Interval`,
and the zero form at `Sym`; candidates are measured before one is
chosen, and if no single spelling does all three it becomes a design
question. Being fixed on `boxfinish/signed-radius`. (BOXFINISH
orchestrator)
