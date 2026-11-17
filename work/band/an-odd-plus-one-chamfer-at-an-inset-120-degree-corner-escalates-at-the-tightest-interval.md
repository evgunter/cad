---
id: an-odd-plus-one-chamfer-at-an-inset-120-degree-corner-escalates-at-the-tightest-interval
kind: issue
title: An odd-plus-one chamfer at an inset 120° corner escalates at Interval at ε 1e-12, its trimline's endpoint certificate a hair wider than the band
status: open
opened: 2026-11-16
priority: P3
cost: E
---


Found by the bracket step 3, cut 2 lane (PR 7038).

At `Interval` at ε 1e-12, the chamfer of the inset 120° corner's odd edge with its second pair edge refuses `Op { site: "trimline mef", Certification { Escalated { check: EndpointEnd, predicate: "carrier_endpoint_end" } } }`. The fixture is `band_mixed_corner::inset_at(2π/3)`, the edge the one from the origin toward `(−sin φ, −cos φ)`. The enclosure is `[0, 1.12e-12]` against a zero of `1e-12`. The same request builds at f64 at every ε, and at `Interval` at 1e-9 and 1e-6. It is pinned as that escalation in `crates/sweep/tests/band_mixed_corner_odd_plus_one.rs::the_odd_edge_with_one_pair_edge_chamfers_at_a_non_right_plan_angle` (`assert_endpoint_escalation`).

The same edge escalates in other requests too:
- the pair alone (its mitre) escalates at the same site;
- the all-three corner, chamfer and fillet, escalates at `mixed host arc mef` (`carrier_matches_mapped_source`).

So it is the fixture's coordinates, read through `sin`/`cos` into an `Interval`, against the tightest band. It is not the odd-plus-one rule. The fix is either a fixture with exact coordinates at 120° or a narrower enclosure for the endpoint read. The latter is the one a user would feel: a blend that builds at f64 and escalates at `Interval` is a certified-scalar gap.

P3: only at the tightest ε, only at `Interval`, and refused typed rather than built wrong.
