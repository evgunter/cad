---
id: a-plan-step-carries-its-coefficient-twice
kind: issue
title: A knot-algebra plan step carries its coefficient twice: exact ring ingredients (Ratio) beside the held-knot form (Coef)
status: open
priority: P3
cost: E
opened: 2026-10-18
---

Found by both reviewers of C6 S4 (PR 5309), and ruled to its own item by the INLANE orchestrator.

## Finding

Each `Step::Combo` in `crates/geom-core/src/spline/algebra.rs` holds its combination coefficient twice:
- `ratio: Ratio` (`algebra.rs:202`, the struct): its exact `f64` ingredients `(inserted − lo)/(hi − lo)`, which the ring applier `CurvePlan::apply_certified` forms as an outward-rounded quotient;
- `form: Form(Coef)` (`algebra.rs:212`): the same number as indices into the input knot vector (or elevation's `num`/`den`), which the appliers at a scalar (`apply_rational`, `apply_homogeneous` in `projective.rs`) form from the held knots.

Each constructor writes the two together, so they agree only by construction:
- insertion: `algebra.rs:701`;
- removal: `algebra.rs:1241`, `:1262`, where `Ratio.inserted = u` and `Coef::Knots { u: KnotAt::Index(r) }` agree because `knots[r] == u`;
- elevation: `algebra.rs:1381-1386`, `Ratio { inserted: num, lo: 0, hi: den }` beside `Coef::Constant { num, den }`.

The `lambda: f64` beside them is a third spelling, the `f64` reference applier's.

## Why it is two

The ring applier takes one channel and no knots, so it cannot read a `Coef`'s indices. Folding the two would mean handing the ring applier the input knot values (or storing them in the plan).

## Owed

One spelling of the coefficient: either the ring applier takes the input vector's knots and forms its quotient from `Coef`, or the plan keeps the knot values it was planned against. A row that would have caught the two disagreeing (a constructor that wrote one and not the other) goes with it.
