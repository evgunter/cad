---
id: subtract-v-notch-from-seam-aligned-cylinder-escalates-stale-halfedge
kind: issue
title: subtract(cylinder, V-notch tool) with the prism seams under the notch's ridge fails with an internal Euler(StaleKey) instead of a result or a typed refusal
status: open
opened: 2026-09-26
---


Found by CONTACT-3's fix pass, which was building the reviewers'
"V-cut" fixture through public doors. Reproduced by execution; the
failing stage has not been traced.

```rust
// sweep test context: prism, brick, pv, tol as in
// crates/sweep/tests/pis_arc_capped_poses.rs
let cut = |n: [f64; 3]| SplitPlane { origin: Point3::new(0.0, 0.0, 1.6),
                                     normal: Vec3::new(n[0], n[1], n[2]) };
// the notch tool: a brick cut down to the region above BOTH planes
let mut tool = brick((-3.0, 3.0), (-3.0, 3.0), (0.3, 4.0), tol());
for n in [[0.5f64.sin(), 0.0, 0.5f64.cos()], [-(0.5f64.sin()), 0.0, 0.5f64.cos()]] {
    let SplitPart::Body(t) = split(&tool, &cut(n), tol()).unwrap().above else { panic!() };
    tool = t;
}
// a unit cylinder of height 2.5 whose seams are at azimuths ±π/2,
// where the tool's ridge (the line x = 0, z = 1.6) meets the wall
let cyl = prism(vec![pv(0.0, -1.0, 1.0), pv(0.0, 1.0, 1.0)], 2.5, tol());
topo::subtract(&cyl, &tool, tol())
// => Err(Euler(StaleKey { key: HalfEdge(..) }))
```

An `EulerOpError::StaleKey` is corruption vocabulary. The operands are
healthy, so the boolean either has a construction path that loses a
key, or it should have refused typed. The same notch with the seams at
azimuths 0 and π refuses typed (`CurvedSectorSideUnsupported`), as does
the V built as a union of two cut halves (`CurvedPierceUnsupported`).
Owner by path is unclear (`boolean/ops.rs` is reach's, `boolean/join.rs`
zip's), so this is filed here.
