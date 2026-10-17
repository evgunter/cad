---
id: an-offset-faces-dihedral-could-be-read-from-its-description
kind: issue
title: Weigh reading an Approx (offset) face's dihedral from its intensional description, so tier 3 can judge its edges instead of leaving them unjudged
status: open
priority: P3
design: true
opened: 2026-10-15
---


Raised on [ev] PR 5150 (D2's prefer-intrinsic check). Ev: "i feel like i
lack the context to know if we'll eventually want this. i suppose we can
unambiguously add an item to weigh it."

## What happens today

Tier 3's check 4 reads each edge's dihedral, transverse, smooth or in
band, from the two faces' implicit-form gradients
(`geom_brep::classify_dihedral_reported`). A NURBS or `Approx` face has
no implicit form, so an edge next to one is skipped by kind and left
unjudged: no pass, no refusal (`nurbs_adjacent`,
`crates/topo/src/validate.rs:7097`; O5 in `crates/geom-brep/README.md`:
"`Approx` faces inherit the NURBS-adjacent exemption from dihedral
marks").

An `Approx` face is not only a fit, though. It carries its intensional
description, today always `SurfaceDescription::Offset { base, d }`
(`crates/geom/src/surfaces/approx.rs`), and tier 3 already re-derives
the fit's certificate from that description on every call
(`OffsetFitLane::recertify`). The offset's unit normal at a point is the
base's unit normal there, so an edge between an offset face and an
analytic face has a dihedral that could be read from the description.
Two offset faces of one base meeting along an edge are similar.

## What to weigh

- Whether a dihedral read from the description belongs in check 4 for
  `Approx`-adjacent edges. It would narrow the exemption to plain NURBS
  faces, which have no description to read.
- How it is certified. The normal is the base's, which is a NURBS whose
  derivatives `patch_bound.rs` already encloses. The point where the
  edge's station lands on the base needs a foot-point lane, the shape of
  the plane × NURBS lane's certified foot residual. Check whether one
  exists for a general NURBS base.
- What would change at rest: which shell or offset results would then
  refuse `TransverseNotIntrinsic` or `MixedDihedral`, and whether any
  door mints an `Approx`-adjacent edge with a conventional description
  that the rule would then call wrong.
- Whether anything needs it now. No known symptom: the exemption leaves
  these edges unjudged, which fails no body. The value is that tier 3
  judges more of what it validates.

A design question first; build nothing until it is weighed.
