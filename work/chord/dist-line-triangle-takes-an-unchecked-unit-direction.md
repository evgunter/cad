---
id: dist-line-triangle-takes-an-unchecked-unit-direction
kind: issue
title: mesh/cert.rs dist_line_triangle takes a unit direction it does not check — the surface's stored axis, so no caller holds the witness yet
status: open
opened: 2026-09-15
priority: P3
cost: E
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
§RATIFIED). The class: a function whose doc or parameter name asserts
a unit-vector precondition it does not check. `geom_core::UnitVec3<T>`
now exists to carry that fact across a function boundary — minted by
the normalizing constructor (`UnitVec3::new(v, site, band)`: decide the
length under the band, divide), by exact negation, by `sin_cos`, and by
`geom-core`'s `frame.rs` ladders. A function in the class takes the
witness the day its caller holds one; until then the precondition
stays prose, and this row is where that is recorded rather than in a
merged PR body.

The geometry CARRIER fields (`Line.dir`, `Plane.normal`, the conic
axes) stay bare under `geom/src/lib.rs`'s at-rest rule by the ruling;
a parameter that is read straight out of such a field is in the class
but its caller holds no witness, so the take waits on either a
decision at the read (a `UnitVec3::new` under a name the reader owns)
or the carrier rule changing, which is not this row's call.

## The site

`crates/mesh/src/cert.rs`, `dist_line_triangle` (`:99`): "project the
triangle onto the plane through `o` orthogonal to the unit direction
`d`". Its one caller (`:115`) passes the surface's stored `axis`, so
this is the carrier case. The sweep's grep did not match this site —
its signature spans several lines and the `Vec3<` is not on the `fn`
line — so it is here from the ruling's survey; that blind spot is
stated in the PR.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.
