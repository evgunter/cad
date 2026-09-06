---
id: curved-single-host-rim-refuses-at-the-half-band-gate
kind: issue
title: fillet: a closed rim whose HOST is one CURVED face carrying every arc can be authored through topo's kef and refuses at the half-band gate on both routes
status: open
opened: 2026-09-05
---


The answer to `docs/FILLET-H5-SPEC.md` §Out of scope's standing
question — "a rim whose HOST is a curved single face carrying several
arcs (state whether the shape can arise; if it can, file it)" —
measured by the FILLET-H5 R2 review on the frozen head `e44f1a7fe`.

## It can arise, and only one way

**Not through any sweep or boolean door.** A full revolve of a
pole-touching profile mints two half-band faces per segment; nothing in
`sweep` merges two faces of a CURVED surface, and
`merge_coplanar_faces` is planar by name and by gate (it refuses a
group mixing planar and curved members).

**It arises through `topo`'s public `kef`.** Kill one of a curved wall's
two seam meridians and the two half-bands become ONE face carrying both
of the rim's arcs — the curved twin of what a coplanar merge does to a
cap. That is a public certified Euler operator, so the shape is
authorable by any consumer holding a body, and it is a legal tier-2
solid.

Built at
`crates/sweep/tests/fillet_h5_r2_probes.rs::a_curved_single_face_carrying_both_arcs_refuses_at_the_half_band_gate_on_both_routes`.

## It refuses, on both routes, and never carves

The rim then has a curved single host and a mate side that is still
half-bands. Both of `resolve_rim`'s routes reach the same answer:

- with a PLANAR support on the other side, the host rule picks the
  plane as host, so the CURVED single face is the MATE — and the mate
  half-band gate refuses it, "a seam-split rim's support does not carry
  exactly its own rim arc";
- with no planar support at all, the rim falls through to
  `HostSide::Seams` and the same gate refuses on the host side.

`HostSide::Struts` is never chosen for it, by construction:
`resolve_rim` reaches `Struts` only from the branch that has already
fixed ONE PLANAR face as the host of every link. That is why the mode
is passed in rather than derived from the body inside the resolver —
deriving "one face carries every arc" from the body would route this
shape into the strut arm, which has no business with it.

## Why it stays refused

The strut foot is `scaled(host carrier).eval(t)` — the rim's own circle
frame scaled to the host TRIMLINE. On a plane host the trimline is a
concentric circle of the rim's frame, which is what makes the foot the
rim's own azimuth at a different radius. On a curved host the trimline
is a latitude circle at a different STATION as well, and the strut
would no longer be a chord of the host's own chart in the way the
ladder's fence describes. That is a real derivation, not a gate to
widen.

## The ask

Nothing, unless a consumer arrives. This file is the durable record
that the question is answered — the shape arises, it refuses, and it
refuses at a gate that is about the right property. The statement lives
at `blend::surgery::HostSide`'s doc, and the row above is what keeps it
true.
