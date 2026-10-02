---
id: a-rim-offset-half-the-zero-band-builds-in-one-member-order-only
kind: issue
title: A dome rim offset half the zero band off the tube builds in one member order and refuses in the other
status: open
opened: 2026-10-02
---

## What

The tube of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
with its radius set to `R + 0.5 · band.zero()`, unioned with
`dome_on_the_cap()`, the discs declared `Rest` (measured on branch
`tang/abutting-rim`, PR 3823, `Tol::witness()`):

- order 0 (tube ∪ dome) builds: `V = 6.971041473977074`, census
  `(5, 8, 5)` faces, edges, vertices, one shell;
- order 1 (dome ∪ tube) refuses `Merge(Pcurve { source: Certify {
  error: Escalated { check: Envelope, .. } } })`, on `pcurve_envelope`
  with margin `1.000000082740371e-9`, which is the band's zero.

The crossing layer decides the rim exactly on (`LiesOn`) in both orders,
since the offset is inside the zero band. What differs is the merge
stage's pcurve certification of the seam it then builds, which reads
the half-zero offset as an in-band envelope in one order only.

## Where

`crates/topo/src/merge_faces.rs` and the pcurve certification it calls,
outside PR 3823's diff. Filed on TANG because the rim lane is what
reaches it; the owner of the merge stage (FUSE) may take it.

A body that one member order builds and the other refuses breaks the
op's symmetry. Either both orders should build (the zero band says the
rim is on) or both should escalate.
