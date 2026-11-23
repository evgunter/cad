---
id: chart-lane-domains-carry-a-half-extent-only-checked-and-keyed
kind: issue
title: Chart-lane SSI domains carry a half-extent that is only checked and keyed
status: open
opened: 2026-11-23
priority: P4
cost: E
---


`SsiDomain::half_extent` has three readers in `geom-brep`: `check()` (positive and finite), `ImplicitSection::pass_key` (its bits) and `SsiDomain::slab()`. `slab()` is called only by the ℝ³ marches (`march_cylinder_sphere`, `march_cylinder_cylinder`, `implicit_pair_r3`, `march_seeds`, `finish_r3`, `idealized_trace_r3`). `ImplicitSection::of_charted` and `Section::of_charted` never call it: the wall's rectangle bounds the search (the field's own doc: "on a chart lane, which a wall's rectangle bounds, only checked"). So a chart-lane caller computes a number that nothing reads but a positivity check and a cache key.

Measured by the `sizes-are-read-back-out-of-extent-levers` unit (PR 7628), which re-sized `topo::offset_derive`'s two chart-lane slabs from a lever to the walls' nets' reach:

- 24 plane × wall reads (6 processes) moved from 5.58–8.00 to 2.00–3.00.
- 42 pair reads (8 processes) moved from 1.0 or 1.0857 to 2.06 or 2.19.
- The check agreed, old and new, on all 66.

Three mutants survive at those sites, run over every test that reaches them (49): the plane slab a million times too small, the pair slab a million times too small, and the pair slab read as the greater of the two reaches. A NaN slab is red, which shows the tests reach the sites.

**Wanted:** decide what a chart-lane domain carries. Either its half-extent says what it is (only a key), or the chart lane stops building one, or it reads one. Until then, every chart-lane caller computes a slab whose size no behaviour can confirm or refute. Nothing in `SsiDomain` was changed by the unit that found this.
