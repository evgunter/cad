---
id: touch-cone-readings-are-levered-directions-not-face-distances
kind: issue
title: The census's touch analysis decides a face's side of a candidate plane by a unit-direction reading times a lever, not by the face's own distance from the plane — an obtuse sector reads a dipping face as on the plane
status: open
opened: 2026-09-26
priority: P1
cost: H
---


Filed by CONTACT-1's last review pass. The census's touch analysis
(`crates/topo/src/census.rs`, `touch_lever`, `Cone::within`,
`touch_verdict`) decides every sign as a dimensionless reading of unit
directions — a ray's side of a candidate plane, a face normal's
alignment, a dihedral — times a lever (`Margin::levered`). A face's
far points leave a candidate plane by the face's own tilt times its
extent; a reading equals that only when the reading IS the face's
tilt.

**Four rounds of one class** (each found by review, each fixed at the
site it was found):
1. every direction levered at the pair's shortest arm (the dual
   review's dipping sliver: a 0.8 m edge dipping past escalate read on
   the plane);
2. the half-space identity test ("is this candidate my face plane")
   levered at the pair's shortest arm (a candidate tilted 2.5e-8 rad
   passed as the wall);
3. the wedge's in-face rays levered at the edge's length;
4. vertex fans' rays, bisectors and dihedrals levered at chord length
   (the delta review's P1/P5) — closed by `touch_lever`, a face's
   farthest vertex from the touch point;
5. and now `sin α`: a ray's side reading is `sin θ · sin α` for a face
   tilted `θ` about its OTHER bounding ray, `α` the sector's angle. In
   an obtuse sector near 180° (and at a thin corner of a non-convex
   face) `sin α` is small, and a face whose far vertex leaves the plane
   by many times the band reads zero at both its rays.

**Witness.** `census::tests::an_obtuse_sector_is_read_through_its_rays`
(`crates/topo/src/census.rs`): a prism over
`(0,0), (1,0), (1,1), (−1,1), (−1,δ)`, `δ = 0.01`, its bottom sheared
so the far edge dips 30× the zero threshold below a floor slab; the
corner `(0,0)`'s vertex-on-face touch reads Rest. The review's probes
(`q1_obtuse_sector_dip`, `q1b_obtuse_sector_synthetic`, not
committed) read Rest at 30×, 60× and 200× on geometry and at 500× on a
synthetic fan; the same at `fa9360c`.

**Root cause.** Levering a unit-direction reading by a length assumes
the reading equals the face's distance from the plane per metre of
lever. It does not: the reading is a direction's, and a direction
inside a face's plane is not the face.

**Proposed design.** Decide a face's side of a candidate plane by its
boundary vertices' signed distances from the plane through `p`, in
metres (`Margin::of`), with the face's orientation from its outward
normal: exact for a planar face, and no lever at all. A cone lies in a
closed half-space iff every face at `p` does (its vertices' signed
distances all non-negative, or on the plane where they read zero) and
the material is on the far side (read from the outward normals as
today). Dihedrals likewise from the far face's vertices against the
near face's plane.

**Masking.** In every pose measured the pair-level reading (the
edge-in-face wedge along the dipping face) refuses the pair, and no
end-to-end wrong clear has been shown; the witness row pins both halves
so the redesign moves the local half deliberately. Cost H.
