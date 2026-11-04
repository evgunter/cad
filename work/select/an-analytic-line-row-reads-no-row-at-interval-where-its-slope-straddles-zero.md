---
id: an-analytic-line-row-reads-no-row-at-interval-where-its-slope-straddles-zero
kind: issue
title: An analytic line row reads no row at Interval, where a fixed channel's slope encloses zero
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by the band-meridian lane (2026-11-04), from R1's re-review of PR 6514.

**The defect.** `geom_brep::analytic_shared_row` returns `None` at `Interval` for every span of a 30° cone's generator against the plane y = 0, the plane that holds it, while `f64` states and certifies the row. The fixture is `cone_generator` in `crates/geom-brep/tests/a_blend_section_is_a_shared_row_of_its_bands.rs`; its test runs at `f64` only for this reason.

**The likely cause.** `description::harmonic_row` (`crates/geom-brep/src/description.rs`) admits a fixed channel only where its coefficients are exactly zero (`geom_core::is_exact_zero`). The plane's image of the generator holds `v` by the slope `sin α·cos α − cos α·sin α`. That slope is exactly zero at `f64`, and at `Interval` it is an enclosure about zero, so the plane's image reads as no row.

**Done when** a line row whose fixed channel's slope encloses zero is either admitted on a certified reading or refused for a stated reason, and the cone test runs at `Interval` too.
