---
id: a-flush-union-at-the-session-boxs-wall-does-not-build-at-interval-at-the-tightest-eps
kind: issue
title: A flush union of two cubes built 700 units out does not build at Interval at eps 1e-12: the Boolean escalates at bool_contact_edge on an enclosure 1.5e-12 wide
status: open
opened: 2026-11-23
---


## Finding

From EMBED's lane `embed/corner-pieces`, 2026-11-23, on
`origin/embed/wave3-land` at `fdbcd20d8d`.

`topo::union` of two unit cubes, the lower with its low corner at
`(700.3, -600.7, 500.1)`, the upper standing on it moved over by
`(0.5, 0.25)`, built at `Interval` at `CAD_TOLERANCE_EPS=1e-12`:

```
Err(Escalated { decision: Containment { decision: Some(Loop(Boundary)),
escalation: Margin }, diag: Indeterminate { margin: MarginDiag(Enclosure
{ lo: 0.0, hi: 1.4779288903810084e-12, .. }), terminal_sliver: false,
band: Band { zero: 1e-12, escalate: 1e-11 },
predicate: Some("bool_contact_edge") } })
```

The same union builds at `f64` at that ε, and at `Interval` at the
default ε and at 1e-6. At the origin, at `(0.1, 0.2, 0.3)` and a hundred
units out it builds at `Interval` at every ε row. The enclosure is the
contact edge's own width at 700 units, about 13 ulps of a coordinate
there, against a band zero of 1e-12 m.

Not traced further: which reading at `bool_contact_edge` holds the
width, and whether it is an in-box coordinate chain the admission rule
covers.

## Where it shows

`crates/sweep/tests/self_intersection_proves_injectivity.rs`,
`a_flush_union_at_the_session_boxs_wall_holds`: at ε = 1e-12 the row
prints `SKIPPED` for its `Interval` half, tied to this predicate, and
asserts the `f64` body. It asserts both everywhere else.
