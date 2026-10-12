---
id: degree-1-partial-has-four-spellings
kind: issue
title: A degree-1 direction's constant partial is spelled four ways, one per consumer of KnotVector::derivative's None
status: closed
opened: 2026-10-11
branch: encl/degree-1-partial
pr: 4754
closed: 2026-10-12
---


Found by the review of PR 4616.

`KnotVector::derivative` is `None` at every degree 1, because the
derived vector would have degree 0 and `KnotVector::clamped` refuses
that by design. A degree-1 direction's partial is constant on each
span, and four consumers spell that constant for themselves:

- `crates/geom-brep/src/patch_bound.rs` `Second::Zero` (`enum Second`,
  `:508`; `Second::of`, `:524`): the second partial along a degree-1
  direction is the exact zero.
- `crates/mesh/src/nurbs_cert.rs`, the `second` closure (`:3675`,
  `None if degree == 1 => zero`): the same zero, read where
  `check_direction` has already admitted the direction.
- `crates/geom-brep/src/props/quad.rs` `Loose::Const` (`enum Loose`,
  `:1426`; read at `:1139`, `:1293`, `:1614`): per-span constants on
  a raw knot list, which also carries the multi-span case.
- `crates/geom-brep/src/offset_fit.rs` `Composite::build`, the `du`/`dv`
  closures (`TensorCoeffs::from_fn` over `diff.get(0, j)` and
  `diff.get(i, 0)`, `:2359` and `:2368`): the single span's constant as
  its degree-1 form, refusing a multi-span direction as
  `Degree1Crease`.

They agree today, but each is its own reading of one fact, and a
fifth consumer would write a fifth. A geom-core helper that hands back
the degree-1 partial as per-span constants would let all four read one
spelling. It was not built in PR 4616, which only made the `offset_fit`
spelling check its own premise. The first step is to decide whether
the helper's output is `Loose::Const`'s shape (per-span constants on
the raw knots), and whether `patch_bound` and `nurbs_cert`, which want
only the zero second partial, gain anything from it.

Also (from PR 4616's delta review): `offset_fit.rs` `Composite::build`'s `lift` closure re-spells `patch_bound::check_direction`'s degree-1 crease test rather than calling it.
