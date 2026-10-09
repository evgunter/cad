---
id: lane-free-doors-skip-the-analytic-rung3-limbs
kind: issue
title: Lane-free certification doors admit an analytic rung-3 edge on its samples alone, opposite to the M7-8 class's NurbsLaneNotSupplied
status: open
opened: 2026-10-08
priority: P2
cost: M
---


Found by `pcert/projected-image` (PR 4304) and its confirming review.

## What happens

An `Intersection` edge over a `Curve3::Nurbs` carrier between two
analytic surfaces states C2's limbs — limb 2 against each operand over
the edge's interval, and the uniqueness tube — in its own certificate,
through `edge_nurbs::analytic_rung3`, and only when the certifying door
holds the scalar's `NurbsLane` (`crates/geom-brep/src/certify.rs`, the
block after the plane × NURBS lane in `run_checks`). A door holding no
lane admits the edge on the nine schedule samples alone. The lane-free
doors:

- `EdgeCurve::certify` and `EdgeCurve::recertify`
  (`crates/geom-brep/src/certify.rs`), which pass `None`;
- every scalar with no certification rights — the dual (`Dual64`) has
  no `NurbsLane::certified`;
- the `_structural` at-rest doors (`crates/topo/src/validate.rs`, the
  door roster: "no plane × NURBS lane at check 2");
- the graft's `Bridge::Recertify`, which re-certifies every grafted
  edge through `EdgeCurve::certify`
  (`crates/topo/src/boolean/combine.rs:864`), reached from the REST
  lane (`crates/topo/src/boolean/rest.rs:367`) and the combine door
  (`crates/topo/src/boolean/finish.rs:316`).

The plane × NURBS class (M7-8) decides the opposite way: an edge of
that class at a door holding no lane refuses
`CertifyError::NurbsLaneNotSupplied`.

## Open

Which policy the analytic rung-3 class takes at a lane-free door —
refuse (as M7-8 does), or certify on the samples and leave the limbs to
the next lane-holding door (tier 3 at rest re-derives them) — is a
decision, not yet made. Until it is, these doors admit a carrier that
leaves a surface between the samples (the plane-limb row in
`crates/geom-brep/tests/analytic_rung3_certificate.rs` certifies with
the lane withheld). The graft's `Bridge::Recertify` is the door most
worth deciding first: it re-certifies during a boolean, and a lane-free
pass there is not followed by one with the lane until the body is
validated at rest.
