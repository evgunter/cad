---
id: a-declared-line-seam-stops-at-the-curved-lump-site
kind: issue
title: A seam declared along a line (the stadium) verifies and stops at the curved coplanar-lump site
status: open
opened: 2026-10-02
---


## What

The stadium of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
(`the_stadiums_plane_cylinder_seam_is_contradicted_as_a_tangent`):
the slab's top and bottom are tangent to the rod's wall along a ruling,
with aligned outward normals. Declared `Seam`, with the rod's end discs
declared continuations of the slab's sides, the seam VERIFIES along the
DEV-1 line locus (the `Tangent` lane with the sense bit reversed). The
slab ∪ rod union then refuses `CurvedBooleanUnsupported { kind: Cylinder }`.
The rod ∪ slab order escalates on `bool_contact_vertex`, the same
margin its undeclared row escalates on.

## Why

The curved coplanar-lump sites read a declared distinct-carrier
tangency as `Tangent` only. `recl::require_same` refuses any pair
`declares_tangent` does not cover, `sectors::tangent_lump` takes the
second-order verdict for a `Tangent` pair, and `recl`'s tangent-flank
germ short-circuit reads `declares_tangent` too. None of them reads a
seam. A seam's lump verdict is the same second-order question with the
material on the same side, so the arm needs the sense read from the
declared class. The rim seam (the sphere-capped tube) never reaches
these sites.
