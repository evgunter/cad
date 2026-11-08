---
id: a-bisectors-tie-is-read-on-its-leading-term-with-no-witness
kind: issue
title: A bisector's tie, and an edge's against a cone or a torus, decides its side on the leading term with no witness point
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by PR 6385's review (`a-leaves-verdict-carries-no-witness-point`).
`bound_side::Tie::witnessed` reads a witness for a tied **edge** (a
line's chord point, a circle's or an ellipse's arc point) against a
**sphere or a cylinder** (`link::Round`, `metric::Distance::off`). Two
cases decide on the parting's leading term alone, as before the witness:

- **A subdivision bisector** (`Reach::Bisector`). Its parting reads its
  sector face's normal section along it (`Reach::bisector`,
  `bend = n·section`), so its witness must be a point of that section,
  the face cut by the plane through the vertex spanned by `d̂` and the
  face's normal, at arc length `l`. The carrier's nearest point to
  `p + d̂·l` is not that point. It agrees on a sphere and parts at
  third order on a cylinder, a cone and a torus (about
  `l³·s³·c/2R²`). The review measured it on a unit cylinder with a
  bisector 45° off the axis: the foot read `+1.2e-4` where the section
  stood `−5.0e-9`. Needed: a certified normal-section point per carrier
  kind (a great circle on a sphere, an ellipse on a cylinder, a conic
  section on a cone, a spiric section on a torus), read by a distance
  door `link::metric` admits.
- **An edge against a cone or a torus.** `link::Round` holds a sphere's
  or a cylinder's material only, and `metric::Distance` has no
  constructor for a cone's or a torus's carrier.

Witnesses: the five gap rows on the witness row (bisectors on a
torus, a cylinder and a sphere) read `Unresolved::Gap` on their leading
term. `bound_side`'s `a_bisectors_tie_reads_its_partings_own_verdict`
pins the review's two cylinder poses at their leading-term verdicts.
