---
id: render-mm-overflows-to-inf-for-a-delta-the-door-accepts
kind: issue
title: render_mm renders a legal δ above 1.7977e305 as inf, and the row over it stops at a kilometre
status: open
opened: 2026-09-16
---


(VIEW) `DisplayTolerance::new` (`crates/viewer/src/scene.rs`) accepts
**any finite δ > 0** — the predicate is `delta.is_finite() && delta > 0.0`
and there is no upper bound. `DisplayTolerance::render_mm` is
`readout::number(self.0 * 1.0e3)`, and that product overflows for every
δ above about `1.7977e305`. So a δ the door accepted renders as `inf`,
which is not a δ the door accepts and is infinitely far from the value.

Measured (`rustc -O`, the module's own predicate replicated):

| δ (m) | `self.0 * 1.0e3` | `render_mm` |
|---|---|---|
| `1.7975e305` | `1.7975000000000001e308` | exact, reads back (after the third arm) |
| `1.7976931348623156e305` | `1.7976931348623155e308` | exact, reads back |
| `1.0e306` | `inf` | `"inf"` |
| `1.0e307` | `inf` | `"inf"` |

**It is a hole in the row, not only in the code.**
`no_delta_renders_as_a_number_a_delta_cannot_be`
(`crates/viewer/tests/display_budget.rs`) asserts exactly the property
this breaks — `DisplayTolerance::new(read * 1.0e-3).is_ok()` — and it is
green, because its sweep runs `1.0e-12` to `1.0e-1` and its named ends
are the two smallest positives and a kilometre. The top of the type is
outside the population, so the row states a universal it never tests
there. A sweep that stops short of an end is that fact recorded as an
absence (this program's own rule, `work/view/plan.md`).

**What a repair has to decide** is where the conversion's overflow is
answered: in `DisplayTolerance::new`, which would make the type refuse a
δ no tessellation could use anyway and give the render a total domain;
or in `render_mm`, which would have to say something true about a δ whose
millimetre value is not an `f64`. The first is a change to a door's
accepted set and is the larger claim; the second leaves the type holding
values its own render cannot spell. Either way the row above owes an end
at the top of the type, which is the cheap half and is independent of
the choice.

Found by `the-scientific-arm-rounds-out-of-the-type`, whose item argued
the carve-out was safe partly on the ground that *"a δ that large does
not survive the millimetre conversion"*. It does not survive it, and
that is this defect rather than a reason there is none.
