---
id: ssi-r3-slab-is-not-geometry
kind: issue
title: ssi: the ℝ³ lane ends an open branch at the caller's slab, which is not geometry, so slab-face flushes and slab-corner touches are artefacts of the box
status: open
opened: 2026-10-02
priority: P2
cost: M
design: true
---


(SSI implementer on PR 3862, building the boundary pass, 2026-10-02.
The finding is designer B's, from the contact fork's first round, "For
the orchestrator": "The ℝ³ lane has the same problem. In
`cylinder_sphere_ssi`, the slab is the caller's box, which is not
geometry, so slab-face flushes and slab-corner touches would be
artefacts of that box. Once the reach is cut to the bounded operand, a
branch meeting the slab is a domain that is too small, and should
refuse as such. This is not a contact.")

## What

The plane × NURBS lane now decides its own domain boundary, the wall's
knot rectangle, before marching (C3, `ssi/boundary.rs`), and its
branches end only at the crossings that pass certifies. The ℝ³ lane
(`cylinder_sphere_ssi`, `idealized_trace_r3`) was outside that design
and keeps the old mechanism, confined to it by type:

- `ssi/march.rs`'s `SlabExit` decides `ssi_branch_open_end` on the
  signed distance to the caller's slab and bisects the crossing with
  `push_boundary` (`SSI_SLAB_BISECTIONS`, a fixed 32);
- `march_both` (ℝ³ only, `LocalSystem<2, 3>`) re-marches a short trace
  and refuses `SsiError::TraceUnresolved`;
- `BranchEnd::Slab` / `BranchEnd::SlabInBand` label the ends.

The slab is the caller's box, not geometry. A slab face flush with the
locus, or a slab corner the locus touches, reaches the same refusals
the plane × NURBS lane had before the boundary pass (`TraceUnresolved`,
an open-end escalation, the floor), and a slab that clips a branch
(`m5_pr7_ssi.rs`'s `a_clipped_domain_ends_the_branch_on_the_boundary`)
returns that branch cut at a box face the user never named as geometry.

## Open

Whether a branch meeting the slab should refuse as a domain too small
for the intersection (designer B's reading, which retires
`SlabExit`, `push_boundary`, `TraceUnresolved` and the two `Slab` ends
outright and turns the clipped-slab row into a refusal), or the slab
should be cut to the bounded operand's box so that only a genuinely
unbounded pair meets it. Either changes what the ℝ³ door answers for a
clipped slab, which is a decision, not a fix.
