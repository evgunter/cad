---
id: the-shared-row-contact-class-is-levered-by-the-edges-length
kind: issue
title: A shared row's contact class is levered by the edge's length, where the witness lane and tier 3 lever by the faces' folded arm
status: open
opened: 2026-11-04
priority: P3
cost: M
design: true
---


Filed by the band-meridian lane (2026-11-04), from R1's re-review of PR 6514.

**The disagreement.** An analytic shared row's contact class is decided on sines levered by the edge's own extent: `run_checks` in `crates/geom-brep/src/certify.rs` passes `edge_extent_inf` to `row_contact::read_contact`. So a short enough edge reads any shallow angle as touching:
- A 1e-3 rad crossing over 1 µm of row certifies **Touching**, at `f64` and at `Interval`.
- A cylinder crossing a sphere at 1e-3 rad over 1e-7 rad of the circle reads **Touching** at `f64`.

The witness lane and tier 3's wedge decide the same pair's contact on the faces' folded arm, a length of the surfaces and not of the edge. So on one short edge the structural class and tier 3 can disagree.

**The question (design).** Which lever should the structural contact class use: the edge's extent, the faces' folded arm (as the witness lane and tier 3 do), or both?

**The other end of the same lever.** At `Interval` the three in-span readings' harmonic widens as the span shrinks. The stadium junction over 1e-5 rad escalates `shared_row_touch`, while `f64` still reads it Touching. That limit is pinned where it stands today by `a_stadium_section_over_ten_microradians_escalates_its_touch_at_interval` (`crates/geom-brep/tests/a_blend_section_is_a_shared_row_of_its_bands.rs`).
