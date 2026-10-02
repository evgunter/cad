---
id: selector-has-no-sharp-edge-atom-so-a-kind-pair-takes-tangent-seams
kind: issue
title: "select: no convexity or sharp-edge atom (GS-Q2), so a kind-pair description of a plate's creases also takes its profile fillets' tangent seams"
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
---


## Finding

Demand evidence for the reserved `GeomPred::Convex` / `Reflex` slot
(`crates/editor-core/src/names/geompred.rs`, "Reserved, unbuilt:
convexity (GS-Q2)"; `docs/SELECT-DESIGN.md` GS-Q2). GS-Q2 deferred it
because "the demand evidence is a COMMENT, not a call site"; this is a
call site.

The tour's `rocker` (`demos/tour/src/rocker.rs`, `crease_narration`)
extrudes a plate whose outline is filleted in the profile and whose
keyhole is filleted on the solid. The keyhole's creases are, by
description, "the lines between a cylinder and a plane" — and on a
plate with profile fillets that description also matches every
vertical seam where a fillet arc meets a straight side: 8 edges, 6 of
them tangent. `fillet_edges` on the 8 refuses `TangentialEdge`, as it
should. What a user means is "the SHARP ones" (or "the convex ones"),
and neither seat can say it:

- the body seat (`topo::query`) has kind and adjacent-kind predicates
  only, so the scene scopes the description to the keyhole loop's
  struts (`Extruded::walls`);
- the document seat (`select_where`) localises with `datum_distance`
  to an axis datum at the keyhole's centre
  (`crates/pncad-py/tests/test_north_star.py`, `TestRocker`).

Both work; both say WHERE the creases are rather than WHAT they are,
and every filleted plate whose creases sit beside profile fillets
meets the same over-selection.

## What the taker owes

The GS-Q2 design question re-opened with this call site: a decided
dihedral atom (the blend battery's `fillet3_convexity_sign` margin is
the existing comparand, `sweep::blend::battery::convexity_at`), its
answer on a tangent edge (neither convex nor reflex — the `Zero` the
battery already refuses), and its body-seat twin in `topo::query`.
A design fork, so it goes to Ev before it is built.
