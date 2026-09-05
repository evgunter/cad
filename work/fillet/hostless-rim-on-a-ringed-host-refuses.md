---
id: hostless-rim-on-a-ringed-host-refuses
kind: issue
title: fillet: a hostless-crossing rim whose one plane host also carries a RING refuses; the annulus band's host trim has no answer for that ring
status: open
opened: 2026-09-05
---


Found by the FILLET-H5 R1 review on the frozen head `e44f1a7fe`, and
the reason `FILLET3_ASSEMBLY_RECOURSE`'s closed clause states a
condition instead of promising the carve unconditionally.

## The shape

A revolve whose flat top runs inward to a dome, repaired the way every
boolean consumer must:

```rust
let body = revolved_about_y(
    vec![(0,0), (1,0), (1,1), (0.5,1) /* bulge tan(pi/8) */, (0,1.5)],
    Revolution::Full, tol);
body.merge_coplanar_faces(tol)?;
```

The merged flat top is ONE plane face and an ANNULUS: its OUTER cycle is
the two arcs of the cylinder's top rim at radius 1, and it carries ONE
RING, the two arcs of the dome rim at radius 0.5.

Its **top outer rim `(1, 1)`** is therefore everything the hostless
crossing needs — one plane host, the rim in that host's own outer cycle,
crossings trivalent, convex — and it refuses:

```
UnsupportedChain { detail: "a hostless-crossing rim's host face carries
                            rings of its own" }
```

Live at
`crates/sweep/tests/review_fillet_h5_r1_probes.rs::r1_a_hostless_rim_on_a_ringed_host_refuses_under_a_recourse_that_promises_it`.
The same body's BASE rim `(1, 0)` is the unit's shape with a ring-free
host and carves (`r1_the_bosss_base_rim_is_hostless_and_carves`), so the
ring is the whole difference.

## Why it is refused rather than carved

The band's host trim becomes the host face's NEW outer boundary, moving
inward from the rim by the trimline setback. Nothing in the hostless
arm says where a RING of that face then sits relative to that new
boundary — the trim could cross it, or enclose it. `ring_clearance_pass`
is the check that answers exactly this question, and it is scoped to
`RimShape::Ladder`: an annulus rim's trim circle IS the replacement for
part of its host's outer boundary, so the external-separation form the
pass uses would refuse every annulus rim on its own rim edge
(`surgery.rs`, the `let RimShape::Ladder { .. } = rim.shape else`
guard and the paragraph above it).

So carving here needs the clearance question answered for a shape the
pass was not written for. That is a numeric decision, not a routing one,
and H5 refused rather than take it: the gate is
`surgery.rs`'s hostless host gate, arm 1.

## What it cost, and what it bought

The first spelling of H5's recourse said the closed clause carries
"whether each support face carries one arc of the rim or one face
carries every arc" — UNCONDITIONAL — while this body satisfies that
description and refuses under that very sentence. The clause now reads
"either with each support face carrying one arc of the rim, or with one
ring-free face carrying every arc as its whole outer cycle", which is
true at every site the tag fires. The R1 row above is what caught it.

## The ask

Decide whether the ringed host carves. Two shapes the decision could
take:

- **Extend the clearance pass to the hostless annulus**: its host trim
  is a circle and its host's rings are circles, so the same closed-form
  circle-vs-circle margin the ladder arm already uses applies — with the
  containment form this issue's sibling
  (`ring-clearance-refuses-a-nested-trim-circle.md`) is separately about.
  The two are the same arithmetic and are probably one unit.
- **Keep the refusal** and say so at `HostSide`, which is what the head
  does today.

Either way the recourse sentence follows the decision; it must not move
ahead of it again.
