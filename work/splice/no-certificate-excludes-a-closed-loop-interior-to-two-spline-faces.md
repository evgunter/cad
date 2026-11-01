---
id: no-certificate-excludes-a-closed-loop-interior-to-two-spline-faces
kind: issue
title: "No certificate excludes a closed intersection loop interior to two spline faces: the interior-loop guard has no NURBS x NURBS arm"
status: closed
opened: 2026-10-28
closed: 2026-10-31
priority: P1
cost: H
refs: [nurbs-plane-section-has-no-component-arm, a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary]
---

## What

The boolean's interior-loop guard owes, for every curved face pair, that
the two faces meet only where the boolean found curves: no closed
intersection loop lies interior to both. For two spline faces nothing
proves it:

- `topo/src/boolean/ops.rs` `interior_loop_verdict` → `section_pairs` →
  `pair_verdict` reads each pair through `section_cert::classify`;
- `classify` has no `(Nurbs, Nurbs)` arm and falls to
  `Section::Intractable` (its catch-all), as `geom_brep::pair_section`
  routes `(Nurbs, Nurbs)` to `SectionRead::Elsewhere`;
- `section_cert::certify` reads `Intractable` as `Refusal::Reach`
  before it reads `evented`, so a pair with found crossings is no
  exception;
- the guard raises `CurvedPairUnsupported { site: InteriorLoopGuard }`.

Sound, and blocking: every boolean whose operands' spline faces overlap
in box refuses there once its edges certify. It is broader than the
sheath-on-blade graft:
any loft or sweep body against another.

## Measured (2026-10-28)

The sheath-on-blade sweep row (`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`),
blade ∪ sheath, with limb 3 temporarily stubbed to pass (not committed),
reaches this guard at ε 1e-9, 1e-6 and 1e-12. Every pair the guard reads
is evented and every one refuses `Reach`: the plane cap of each body
against the other's two walls, and all four wall × wall pairs. The first
in arena order, the one the refusal names, is a Plane × NURBS pair
(`operand: B, kind: Nurbs, other_kind: Plane`), whose arm is
`germtail/nurbs-plane-section-has-no-component-arm`. So the graft needs
both arms, that one first.

Stubbing only the one-arc walk is not enough to reach the guard: limb
3's own margin reading then refuses, `TubeStraddles { Zero(margin 0.0),
boxes: 16 }`, the tour's default-ε reading
(`ssiedge/a-tube-whose-graph-margin-straddles-zero-refuses-as-a-decided-zero`).

## What is open

An arm for the guard's section certificate on two spline faces: away
from a shared row, a certified exclusion of a closed loop interior to
both (a component count, or a witness per component, as the NURBS ×
plane row asks); beside a shared structural row, PR 5933's two
designers differ on whether a cover of the strip beside the row is
needed. It is a design row (C3's exhaustiveness for a spline pair), not
an implementation one.

2026-10-31: its design was settled by `[ev]` PR 5990 (fork row 217,
approved by Ev): C3's NURBS × NURBS lane with `Side` and `Corner`
regions built from a handed-in row or vertex, and the boolean reading
the pair through the section value, a component that reaches its
chart's boundary cleared by W1′ (`crates/geom-brep/README.md`).

2026-10-31: Review tier: dual (H: a guard that clears interior loops, on
shared SSI ground).

## Closed (2026-10-31)

The guard's `section_cert::classify_known` reads two spline faces through
`geom_brep::spline_pair_section`. That is C3's whole reading
(`ssi::Section::read_all`), handed the rows and corners the faces share
(`boolean::spline_known`). The pair lane builds the `Side` region (a row
cover through `four::row_cover` at a width) and the `Corner` region. The
witness rows are `topo`'s `section_cert_spline_pair_rows`. The residue is
on SPLICE's slate:
`splice/a-pair-walk-end-at-a-domain-corner-refuses-undecided` (evidence added).

