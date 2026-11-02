---
id: a-rod-union-refuses-at-interval-on-a-poison-ruled-region-span
kind: issue
title: A rod union refuses at Interval on a poison bool_ruled_region_span margin, so the self-intersection tee row has no Interval body
status: open
opened: 2026-10-30
priority: P2
cost: M
---


## Finding

Found by RIMSCALAR (CLEAR), which builds every row of
`crates/sweep/tests/self_intersection_proves_injectivity.rs` at
`Body<Interval>` by the same generic builders as at `Body<f64>`. The row
`a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone`
builds rod `A` (`sweep::test_support::prism_at`, radius 1, `z ∈ [−3, 3]`)
less the slab tilted `−π/4` about `y`, then unites it with rod `B`
(`z ∈ [0.2, 3]`) turned `π/2` about `y`. At `f64` both operations build.
At `Interval` the subtract builds and `topo::union` refuses:

```
Err(Escalated { decision: Containment { decision: None, escalation: Margin },
  diag: Indeterminate { margin: MarginDiag(Invalid), terminal_sliver: false,
  band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("bool_ruled_region_span") } })
```

The margin is `Invalid` (poison), not in-band, so this is an enclosure
that did not evaluate, at `topo::boolean::ruled_region`'s
`bool_ruled_region_span` site, rather than a decision refinement could
settle.

The row pins this refusal as the tee's known divergence: the Interval
injectivity verdict has no body to run on until the union builds.

## The refusal moves with ε

Found by EPSROWS (CLEAR). At `CAD_TOLERANCE_EPS=1e-12` the same union
refuses earlier, at a different site:

```
CurvedPierceUnsupported { operand: A, face: FaceKey(3v1), edge: EdgeKey(11v1),
  band: Band { zero: 1e-12, escalate: 1e-11 } }
```

At the default ε and at `1e-6` it is still the poison
`bool_ruled_region_span` above. So a fix for the poison span alone may
leave the union refusing at `1e-12`. The tee row now pins only that the
union refuses typed (`topo::test_support::offer_key`'s defect flag clear),
so it goes red when the union builds at any ε.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)
