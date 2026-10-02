---
id: seam-description-reads-a-dihedral-at-the-seams-own-length
kind: issue
title: The boolean's seam description reads a seam's dihedral at the seam's own length, so a short seam between faces that part far away is left a scaffold at rest
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Filed by CONTACT-9.

`boolean/ops.rs` (the seam description, `classify_dihedral(surf1,
surf2, witness, extent, band)` at `:1475`) decides whether a result
edge's two faces cross transversally. It levers their angle at the
edge's own length. A seam 1 mm long between faces `5e-8` rad apart
reads Smooth, and the edge is left a `Scaffold` at rest. Tier 3 then
reports `ScaffoldAtRest`, and on the intersection also `LaminaWedge`.
But the two faces part by `500·ε` 10 m away.

Witness: `crates/topo/tests/contact9_side_codes.rs`,
`a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`.
It skips tier 3 for this reason. The same pose with a 1 m seam passes
tier 3. This is the lever class CONTACT-7 and CONTACT-9 closed at the
touch analysis and the side codes: a reading levered at a length that
is not the one the verdict is about. Here the dihedral should be read
at the faces' extent from the seam.

## Now a refusal (FUSE, 2026-10-02)

The boolean's result gate refuses a scaffold at rest
(`boolean::ops::gate`, from the FUSE unit
`a-boolean-result-gate-ships-a-scaffold-at-rest`). The poses this
lever leaves a scaffold no longer answer: in `contact9_side_codes`,
the tilted wedge's ∩ and − and the pierce row's `tool − block` refuse
`ResultInvalid` with `ScaffoldAtRest` alone, and those rows now pin the
refusal. Fixing the lever turns them back into answers; the corner and
volume checks they lost are in the rows' history.
