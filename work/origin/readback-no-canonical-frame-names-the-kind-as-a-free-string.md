---
id: readback-no-canonical-frame-names-the-kind-as-a-free-string
kind: issue
title: ReadbackError::NoCanonicalFrame carries the carrier's kind as a free string beside geom's CurveKind and SurfaceKind
status: open
opened: 2026-10-02
---


Found by the kind-namer sweep of TQUERY's
`one-kind-mirror-per-geometry-enum`, which moved `CurveKind` and
`SurfaceKind` down to `geom` with one `name()` each and retired
`step-export`'s own namers.

`crates/topo/src/readback.rs` `ReadbackError::NoCanonicalFrame`
carries `carrier: &'static str`, filled at three sites with its own
spellings: `face_pose` writes `"nurbs surface"` and
`"approximating surface"`, `edge_pose` writes `"nurbs curve"`. The kind
is already known at each site (`Surface::Nurbs`, `Surface::Approx`,
`Curve3::Nurbs`), so the payload could carry the typed kind
(`geom::SurfaceKind` / `geom::CurveKind`, or an enum over the two) and
the Display could read `name()`. Not changed in that unit: the field is
public, `editor-core`'s interrogate door passes the string through
(`crates/editor-core/tests/lib_u5_interrogate.rs`, `msolve9_from_face.rs`
assert on `"nurbs surface"` / `"nurbs curve"`), and a typed payload is a
signature change on another program's ground.
