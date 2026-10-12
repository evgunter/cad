---
id: two-second-order-conic-readers-certify-one-fact-differently
kind: issue
title: The splitting lane's second-order conic reading is a bare sagitta while the boolean's conic_side is certified: one fact, two certifications
status: closed
opened: 2026-10-10
priority: P2
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, 4575]
branch: contacthold/one-certified-conic-side
pr: 4826
closed: 2026-10-12
---


## Finding

From the dual review of PR 4575. Two lanes read which side of a plane a
conic leaves on when its tangent lies in the plane:

- the boolean, `sectors::conic_side`: a certified lower bound on the
  lift (`2·sin²(κ_min·l/2)/κ_min`, capped at `π/(2·κ_max)`), less the
  first-order residue, against the band;
- the splitting lane, `splitting/neighborhood.rs` at
  `split_conic_departure`'s Zero, through `geom_brep::enters_material_order2`
  (`crates/geom-brep/src/enters.rs`): a bare sagitta `½·(κ·n̂)·extent²`
  at the edge's extent, with no residue term and no curvature bound.

One fact, two certifications, and the splitting one can read a side the
boolean's would refuse: the sagitta at the extent overstates the lift
where the arc turns past `π/2` or its curvature varies, and the residue
the first-order Zero left is not subtracted. The splitting reading wants
the certified one, or both want one home (`geom_brep`).
