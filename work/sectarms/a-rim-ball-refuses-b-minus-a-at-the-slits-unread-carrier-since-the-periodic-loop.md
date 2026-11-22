---
id: a-rim-ball-refuses-b-minus-a-at-the-slits-unread-carrier-since-the-periodic-loop
kind: issue
title: Since PR 7052 a ball across the drum's rim refuses B ∖ A at the slit, on a section edge that ends on the cut plane at both ends and is clear of it between
status: open
opened: 2026-11-22
priority: P0
cost: M
design: true
---

Filed by GREEN's U2 lane (`green/rim-ball`), from
`work/green/main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face.md`.
`subtract` on ordinary geometry refuses where the body exists, so P0.
The refusal keeps the slit's contract; the certificate under it is too
weak (below).

## The red

`sweep`'s `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim`:
a ball r 0.1 at `(0.45, 0, 0.3)` against the drum cut by the plane
through `(0, 0, 0.5)` tilted by `TILT`. Five of its six runs build at the
oracle's volume; **B ∖ A** refuses

```
EdgeInFace { edge: EdgeKey(16v1), face: FaceKey(4v5), gap: CarrierUnread }
```

at every ε (nightly 44870651337, main `f5ef8f2ba0`). The row is pinned
at that stop (`pinned_red::row`, every ε) until this item closes; the pin
goes red the day B ∖ A builds or stops any other way, the edge and face
keys included.

## Bisected: PR 7052

Hosted CI, default ε, the gate's slow-set step on a throwaway branch per
side: `fa4e9ba921` (7052's first parent) PASS, run 44997693000;
`6c2430e888` (7052's merge) FAIL at the stop above, run 44997688528.
Nightly 44023697569 (11-17, `cdee746161`) passed it at every ε; 11-20's
was the first red.

## Probe: a conservative refusal from a certificate too weak

A print at `topo::boolean::slit::holding`'s unread arm, on the row's own
pose (main `1d14b92ca6`, CI run 44997911372, a throwaway branch):

- face 4 is the drum's **cut plane** (origin `(0, 0, 0.5)`, normal
  `(-0.2955, 0, -0.9553)`); edge 16 is the sphere × cylinder section
  edge, a `Curve3::Nurbs` (the clamped view over span
  `(0.2047, 0.8841)`, 510 control points, degree 3);
- `Names { ends: true, record: false }`; `walls_inside` is
  `Ok(Some(false))`, so `unplaced` returns the unread refusal;
- the edge's height over the plane at 33 equispaced parameter
  **samples**: `3.5e-13` at the start, then `0.010, 0.021, … 0.126` (the
  midpoint, `z = 0.213`, where `y` crosses 0) `… 0.010`, and `5.1e-13` at
  the end;
- its 510 **control points**: all on that one side, the interior ones
  from `1.35e-4` up to `0.126`, the first and the last at `3.5e-13` and
  `5.1e-13`, which are the edge's own ends on the rim.

So the edge lies off the plane along every piece of its length and
touches it only at its two rim vertices. The refusal itself is correct
under the slit's contract: `holding` takes a spline carrier not
certified off a face it is related to as lying in it, and
`CarrierUnread` says only that the reading is missing. What is too weak
is the certificate. `clears_plane` asks every control point to be
decided beyond the band (`RIDER_HULL_OFF`) on one side, and a clamped
carrier's end control points are its endpoints, which lie on the plane
here by construction (in f64 they are on the same side, but inside the
band). No carrier with both ends on the plane passes that test, however
far it stays from the plane between them. The row's expectation is
right: the body exists at the oracle's volume.

**Not a knife edge.** Six neighbours (`c.x` 0.44 and 0.46, `c.z` 0.29
and 0.31, r 0.099 and 0.101) all refuse B ∖ A the same way at all three ε.
Their A ∩ B all build at the oracle's volume, and the r 0.2 rim pose
builds both runs.

## Why 7052 moved it (a reading, not measured)

The midpoint of the edge is at `y = 0`, the ball's seam meridian. PR 7052
joins arcs of one periodic loop across a 2-valent vertex (`crease_at`
reads the stored carrier, so the vertex has no crease). Before 7052 the
section was most likely two edges, split at that seam, each with one end
on the cut plane, so `Names::ends` was false for both and the face was
never asked. After 7052 it is one edge with both ends on the plane. A
print of the same arm on `fa4e9ba921` would settle this.

## Done when

The row's B ∖ A builds at the oracle's volume at every ε, and its pin in
`crates/sweep/tests/cylinder_sphere_frame.rs` comes out. The likely
ground is the slit's plane reading (`clears_plane`, which `holding`
reaches through `carrier_on_surface`): clear a spline whose control
points are beyond the band (`RIDER_HULL_OFF`) on one side everywhere
except the end ones, when those ends are the edge's own vertices on the
face. On a clamped end span the basis functions of the next control
points are positive inside the span, so the height is positive past the
end vertex.

**The fork** (`design: true`): strengthen the certificate as above (the
slit's ground), or restore the split at the loop's seam so neither edge
has both ends on the plane (7052's ground, `edge_join`'s `crease_at`). The slit is
`topo::boolean`'s ground (onecopy, onehome and others hold
`slit.rs`), so coordinate with them there. SPLICE's
`work/splice/an-unread-rider-related-only-by-its-own-vertices-refuses.md`
is the same arm of `holding`, but its exemption is by lineage, for an
edge and a face of one operand. Here the edge is a section edge whose
ends lie on the other operand's rim, so that exemption would not cover
it.

Reproduce: `cargo nextest run -p sweep --test all -E 'test(=cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim)'`,
with the pin taken out.
