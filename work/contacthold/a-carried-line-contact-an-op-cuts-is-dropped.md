---
id: a-carried-line-contact-an-op-cuts-is-dropped
kind: issue
title: An edge-in-face line contact carried into an intersect or subtract that cuts it comes out unrecorded at tier 3′
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [3955]
---

## What

Found by PR 5187's per-op tier 3′, in `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs`, `a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`. The row pins the finding, so a fix shows.

**The setup.**

- Two prisms touch along the line through (1.5, 1): the sector prism's edge lies in the triangle prism's side face.
- Their union carries its contact records (`contacts.carried()`) into an op against the lying cylinder, whose top that edge pierces.

**What happens.** The op builds sound: tier 3, closed-form volume, material. But tier 3′ finds `UnrecordedContact { contact: EdgeFaceOverlap, .. }` at (1.5, 1.0, 0.735), on the line between the two prisms, below the cylinder's top. It happens in:

- the intersection, either operand first;
- the pair less the cylinder;
- over both cylinder spans.

The union, and the cylinder less the pair, record it.

**Unmeasured:** whether the carried edge-face row is dropped by `ops::remap_carried` once the op cuts the edge at the cylinder's top (the family of `a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped`), or never mapped at all.
