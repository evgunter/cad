---
id: a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face
kind: issue
title: A convex corner kissing a cube's face ships results with an undeclared vertex-on-face (8 runs, identical on main)
status: open
opened: 2026-10-04
priority: P1
cost: M
refs: [a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms]
---


## What

Found by PR 4026's review r2 (N1) and re-measured by its fix pass on
main `45dc18f9`, in release. The battery is `join_pierce_r2_probes.rs`
`r2_shapes_battery` on branch `join/pierce-two-out-runs-review-r2`
(one pose: `R2_SHAPE=Lcvx R2_TAG=<tag> … r2_one_pose`).

The shape `Lcvx` puts the L-prism's convex corner on a cube's face,
undeclared, over a grid of directions. Eight results build with tier 2,
the certificate and the exact volume, but tier 3′ refuses
`UndeclaredContact { VertexOnFace }`. For example `g4.6 cp U` at
`(2, 1, 1)`. The runs are `g4.6`, `g4.7`, `g5.7`, `g5.8` and
`nf0_1e-2k0s1` (cp U), and `g16.3`, `g17.2`, `g17.3` (cp S).

The lines are identical on main and on PR 4026's head; nothing in that
PR's diff reaches a single-run corner. The result keeps a vertex of one
operand lying on a face of the other and records nothing for it. The
same battery's ten `StaleContactDeclaration` results are evidence on
`a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`.

## The shape to give

Read one pose's contact records through `remap_contacts`. Either the
kissing vertex's record is dropped, or none was minted for a contact
the result keeps. Fix it at the record's mint or its remap.
