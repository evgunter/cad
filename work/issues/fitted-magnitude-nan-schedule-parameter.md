---
id: fitted-magnitude-nan-schedule-parameter
kind: issue
title: error payloads - FittedMagnitude::LastFootDistance carries t = NaN where there is no schedule parameter
status: open
opened: 2026-09-04
refs: [931, 925, 934]
---

## The finding

Found by FILLET-E3's sweep obligation (issue 934: grep `f64::NAN` in
non-test `src/`, not the diagnostic type). It is the same class 934
enumerates, in the file that holds the class's own good precedent.

`crates/geom-brep/src/pcurve_cache.rs:1183-1188` — the endpoint-foot
translation:

```rust
Err(crate::edge_nurbs::PlaneNurbsRefusal::FootPointInconclusive {
    last_distance, ..
}) => Err(PcurveCertifyError::FittedCertificate {
    limb: Some(SsiLimb::OnLocus),
    what: "an edge endpoint has no certified foot on this chart, so where its \
           image sits cannot be measured",
    magnitude: Some(FittedMagnitude::LastFootDistance {
        t: f64::NAN,
        last_distance,
    }),
}),
```

`FittedMagnitude::LastFootDistance`'s `t` is documented at
`pcurve_cache.rs:658-666` as "The schedule parameter" — the parameter
the projection gave up at. This call site has no schedule: it is a
chart-foot query at an EDGE ENDPOINT, and the endpoint is not a
schedule position. `f64::NAN` is standing in for "there is no such
parameter" — a structural absence wearing a measurement's costume,
which is exactly #925's shape and row 1 of #934's table.

The rendering says it out loud
(`pcurve_cache.rs:881-883`):

```
(the projection's last distance was 1.2e-9 m at t = NaN)
```

## Why it is not a poison propagation

The other `f64::NAN` sites the sweep turned up in this crate are the
sanctioned poison channel — `implicit.rs`'s `poison<T>()`,
`ssi/certify.rs:841`'s zero-speed guard, `offset_meters.rs:439`'s
explicit re-poisoning of a `max` fold — where a NaN scalar is the
fail-loud value and every downstream comparison is false by design.
This one is not computed and not propagated: it is written into an
error payload as a placeholder, and nothing downstream tests it.

## The fix, when its file next opens

Invariant 1 of #934: a payload that measured nothing carries `None` or
its own variant. Either

- `t: Option<f64>`, `None` at the endpoint door, with the Display arm
  dropping the `at t = …` clause when there is none; or
- a sibling variant — `EndpointFootDistance { last_distance }` — since
  "the projection gave up at schedule parameter t" and "an edge
  endpoint has no certified foot" are two situations, and the second
  one's `what` string already says which.

The second reads closer to the precedent the enum was built on: each
`FittedMagnitude` arm names one measured thing, and this door measures
a distance and nothing else.

## Home

`crates/geom-brep/src/pcurve_cache.rs` is PCURVE's ground (closed), the
same home #934's third row (`edge_nurbs.rs`) has, so this lands under
`work/issues/` beside it and goes to whichever unit next opens the
file. FILLET-E3 did not touch it: its brief scopes it to
`crates/sweep/src/blend/*` and names `geom-brep` explicitly out of
scope.
