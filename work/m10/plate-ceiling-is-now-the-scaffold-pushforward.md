---
id: plate-ceiling-is-now-the-scaffold-pushforward
kind: issue
title: with both of the arc carrier's same-object identities registered, the plate is bounded by carrier_matches_mapped_source — the carrier against the scaffold pushforward, two independently built objects
status: open
opened: 2026-09-06
---


**Measured by M10-9 under amendment A1** (the registered-identity door;
branch `m10/m10-9-registered-identity`), on the tour's two-hole plate
at its real study (±0.05 mm spacing, σ = 0.01 mm radii; `demos/tour`
stop 1).

The swept arc carrier's builder now registers BOTH of the same-object
identities it guarantees (`crates/sweep/src/swept.rs`):

- `register_rim_identity` — `‖q_from − c‖ = r`, discharging
  `carrier_endpoint_start`;
- `register_span_identity` — `carrier.eval(param_end) = q_to`
  componentwise, discharging `carrier_endpoint_end`.

Both work. **And the ceiling still does not move**: at the default ε
the plate certifies whole in `[7.787e2, 7.817e2] · ε` door-open and
door-shut alike, and the first refusal beyond it walks one predicate
further each time.

| door | first refusal beyond the ceiling | enclosure |
| --- | --- | --- |
| shut (M10-8) | `carrier_endpoint_start` | `[0, 1.2459e-9]` |
| open, rim only | `carrier_endpoint_end` | `[0, 1.2466e-9]` |
| open, rim + span | `carrier_matches_mapped_source` | `[0, 1.2986e-9]` |

(band `zero = 1e-9`, `escalate = 1e-8`;
`editor-core/tests/m10_9_evidence_interval::m10_9_ceilings_with_and_without_the_door`,
gated by `m10_9_pins_interval::m10_9_the_rim_registrant_discharges_the_plates_endpoint_identity`.)

## Why the arc carrier's builder cannot register this one

`carrier_matches_mapped_source` is the fenced SCAFFOLDING residual
(`crates/geom-brep/src/certify.rs`, the `Resolved::Scaffold` arm):

```
Margin::of( spec.carrier.eval(t_i).distance( mc.eval(s_i) ) )
```

with `t_i = sample_param(param_start, param_end, i)`,
`s_i = i / (CERT_SAMPLES − 1)` and `mc` the `MappedCurve::PlacedSegment`
pushforward of the sketch segment through the placement.

It IS a theorem of the same construction — the arc carrier is that
pushforward, re-expressed in the circle's own frame — but it is **not a
same-object identity**, and that is exactly the line ERROR-DESIGN E12's
reserve draws ("exact and simple but only for same-OBJECT identities;
the cosurface case is an expression identity"). Two independently built
objects are involved: the `Curve3::Circle` and the `MappedCurve`. The
door aliases NODES, so a registration would have to name the node the
consumer asks about — and there is no such node until the certifier has
chosen a sample. To state it the constructor would have to enumerate
`CERT_SAMPLES` and reproduce `sample_param` and `i/(N−1)`, i.e. carry
the funnel's sampling schedule inside a construction site. That is a
coupling this unit's spec forbids in its own terms ("no edits at any
funnel site"; "not shipped: any registrant outside the arc carrier's
builder and the fillet"), and it would be a registrant per SAMPLE
rather than per identity.

M10-9 therefore stops here, as amendment A1 directs: the first bound
that is not a same-object identity of the arc carrier is named with its
predicate and its enclosure, and not widened into.

## What is owed

- A decision on the shape, not on the site. Two are visible and both
  are design conversations rather than implementation:
  1. **Have the constructor state the pushforward relation once, at the
     level of the two curves rather than at samples** — which needs a
     form-level equation, not a node alias, and M10-9's spec forbids a
     form-level axiom store.
  2. **Retire the scaffolding residual for arc carriers**, since the
     carrier and the mapped source are the same construction: D3's
     fence exists because a transient scaffolding edge has no surfaces
     yet, and the residual is what stands in for a description. That is
     a PCURVE/D3 question, not an E12 one.
- Either way the plate re-measured, and this row and
  `work/m10/plate-ceiling-is-now-the-arc-span-identity` (closed by the
  span registrant) re-cut against it.
- Note that the OTHER three documents are not bounded by this predicate:
  R2's bracket and pad are bounded by `line_span` and R1's annulus by
  this same `carrier_matches_mapped_source`. The bracket's and the pad's
  bound is the real-margin dependency-widening class
  (`work/m10/real-margin-dependency-widening`).
