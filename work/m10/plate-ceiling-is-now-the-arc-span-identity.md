---
id: plate-ceiling-is-now-the-arc-span-identity
kind: issue
title: with the rim identity registered, the plate's ceiling is bounded by carrier_endpoint_end — the arc's SPAN identity, a different theorem
status: open
opened: 2026-09-06
---


**Measured by M10-9** (the registered-identity door, PR on branch
`m10/m10-9-registered-identity`), on the tour's two-hole plate at its
real study (±0.05 mm spacing, σ = 0.01 mm radii; `demos/tour` stop 1).

M10-9 registers the swept arc's rim identity `‖q − c‖ = r` at the site
that guarantees it (`crates/sweep/src/swept.rs`,
`register_rim_identity`, called from `placed_segment_spec`'s arc arm
and from `crates/sweep/src/extrude.rs`'s cylinder wall). It works: at
the plate's nominal `carrier_endpoint_start` goes from 32/16
(theorem/numeric) to 32 theorem + **16 registered** + 0 numeric, and
the whole-box replay just past the ceiling no longer refuses on it.

**And the ceiling does not move by a digit.** With
`carrier_endpoint_start` discharged, the first refusal beyond the
plate's widest whole-certifying box is `carrier_endpoint_end`, at the
same width and with an enclosure that agrees to eight digits:

| | ceiling (bisection bracket) | first refusal beyond it | enclosure |
| --- | --- | --- | --- |
| door OFF (M10-8) | `[7.791e2, 7.844e2] · ε` | `carrier_endpoint_start` | `[0, 1.2465807171e-9]` |
| door ON (M10-9) | `[7.791e2, 7.844e2] · ε` | `carrier_endpoint_end` | `[0, 1.2465807158e-9]` |

(at the default ε = 1e-9, band zero = 1e-9, escalate = 1e-8;
`editor-core/tests/m10_9_evidence_interval::m10_9_ceilings_with_and_without_the_door`.)

## Why the same registration does not reach it

The two endpoint predicates are NOT the same identity.

- `carrier_endpoint_start` pins `carrier.eval(0) = q_from`. For a
  circle carrier `carrier.eval(0) = c + u_ref·r` with
  `u_ref = (q_from − c)/‖q_from − c‖`, so the residual is
  `(q_from − c)·(r/‖q_from − c‖ − 1)` — componentwise zero exactly when
  `‖q − c‖ = r`, which is what the door states.
- `carrier_endpoint_end` pins `carrier.eval(θ) = q_to` with
  `θ = arc_span(bulge) = 4·atan|b|`
  (`crates/sweep/src/swept.rs`, `param_end`), so the residual is
  `c + r·(cos θ·u_ref + sin θ·(axis × u_ref)) − q_to`. Even with
  `‖q − c‖ ≡ r` recorded — which turns `r·u_ref` into `q_from − c` —
  what remains is the ROTATION identity
  `cos θ·(q_from − c) + sin θ·(axis × (q_from − c)) = q_to − c`, and
  the rendered form carries the atom `cos(4·atan(1·abs(1)))` verbatim
  (the whole-box replay's shape report, same probe). That is a
  different theorem of the sagitta construction — the arc's SPAN
  identity — and no registration of `‖q − c‖ = r` reaches it.

## What is owed

- A decision on whether the SPAN identity is a second registrant. It
  is the same shape as E12's own example ("a typed 'built as
  `carrier.eval(t0)`' token"): the constructor that builds the arc
  carrier could register the node it would produce at `param_end`
  against `q_to`, and content hashing makes the same-object condition
  testable. M10-9's spec scoped the unit to two registrants and said
  "not shipped: any registrant outside these two", so this was NOT
  built — it is filed rather than taken.
- With it, the plate re-measured: whether the ceiling then moves, or
  whether `carrier_on_surface_*` (which needs the SQUARED identity
  `v·v = r²`, a third node) is next.
