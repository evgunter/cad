---
id: lane-keeping-at-rest-doors-skip-the-m7-8-class
kind: issue
title: The lane-keeping at-rest doors make no check-2 claim about an M7-8 edge, at every scalar
status: open
opened: 2026-09-05
refs: [H5, 1877]
---

## What

`topo::validate_pseudomanifold`, `topo::contact_marks` and
`topo::validate_geometric_structural` are bounded `T: PropsQuadLane`, which
says a scalar can certify a body at rest and says **nothing** about the C9
ring the plane × NURBS (M7-8) certificate lives in. So none of them can name
`geom_brep::plane_nurbs_limbs`, and check 2 skips an M7-8 edge rather than
reporting a refusal that would be about the caller. **The skip is
whole-edge**: `recertify_via` is one call and, without the lane, the
description resolver refuses `Unimplemented` before the endpoint, interval
and chart-image checks run — so what goes unmade on such an edge is every
check-2 verdict, a drifted endpoint included, not only the plane × NURBS
limbs. This holds at `f64` exactly as at a dual: what decides is the BOUND,
not the scalar.

## What is already closed, and what is left

CERT-M3's fix pass closed the consumer half. The certified twins
(`validate_pseudomanifold_certified` and its certificate form,
`contact_marks_certified` and its declared form) supply the lane at
`PropsQuadLane + CertifiedBounds`, and every production consumer that
reached a body ONLY through a skipping door now takes one:
`AtRestPolicy`'s `f64`, `Probe`, `Interval` and `Sym` arms (hence
`editor_core::assembly`'s `gate_at_rest_declared`) and `step-import`'s
aggregate `gate3`. `topo/src/cert_m3r1_probes.rs` pins the whole door table
on a corrupt M7-8 wall.

**What is left is the doors called BY NAME at a certifying scalar.** They are
public, and each is a real caller today:

- `pncad-py`'s `Value.validate_pseudomanifold` / `Body.validate_pseudomanifold`
  (`crates/pncad-py/src/py/value.rs:310`) — the Python API's own tier-3′ door.
- `demos/tour` (`main.rs:326,336`, `letterforms.rs:232`, `probe.rs:67`), which
  renders through the public door a user would call.
- `topo`'s own suites, and any external consumer of the `pncad` prelude
  (`crates/pncad/src/prelude.rs:260`).

Each of those gets the pass without check 2's M7-8 arm, and nothing tells the
caller so except the door's own docs.

## The shape a fix would take, and why this unit did not take it

Two candidates, neither free:

1. **Route the remaining named callers through the twins.** Mechanical and
   free per site (one identifier), but `pncad-py` is another track's ground
   and the Python door's semantics — "the tier-3′ pass at this scalar" —
   would change under a name users already call.
2. **Make the certified form the DEFAULT name**, as `validate_geometric` /
   `validate_geometric_structural` are one tier down, and rename the
   lane-keeping form. That removes the residue outright and is the shape the
   tier-3 door already has. It also evicts `Body<Dual64>` from the name
   `validate_pseudomanifold`, which is what `H-R3` forecloses in its own
   words, so it is a ruling to ask for and not a refactor to do.

Option 2 is the one worth putting to Ev, beside `H5`'s other two questions.

## Fence

`crates/topo/src/validate.rs` (the door names), `crates/pncad-py/src/py/`
and `demos/tour/src/` for the call sites. The ruling half is `H-R3`'s and
belongs with `H5`.
