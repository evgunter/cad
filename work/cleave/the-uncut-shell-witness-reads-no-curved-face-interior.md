---
id: the-uncut-shell-witness-reads-no-curved-face-interior
kind: issue
title: The uncut-shell witness reads no curved face's interior, so a shell whose vertices and edges all lie on the other boundary and whose other faces are curved refuses ShellWitnessExhausted
status: open
opened: 2026-10-01
priority: P3
cost: M
---



## What

`crates/topo/src/boolean/shell_witness.rs`, tier 3: `face_plane`
answers `KindUnsupported` for a curved face, and the tier passes over
it. Suppose every vertex and edge midpoint of a shell lies on the other
operand's boundary, and every face of it that is off that boundary is
curved. The shell then refuses `ShellWitnessExhausted`, although a
point inside one of those curved faces would decide it. The refusal is
honest: its prose names the flat-face limit.

## Measured

No fixture reaches it yet. The PR 3655 review tried tangent-cylinder
configurations aimed at this shape. Each refused earlier, at
`CurvedPierceUnsupported`, before the witness ran. A flush-capped
cylinder inside a block (`crates/sweep/tests/contained_flush_cylinder.rs`)
does not reach it, because its straight seam edges decide at tier 2.

## What a fix would be

A certified interior point for the curved kinds that
`curved_face_containment` already places. For example, a chart-space
candidate inside the face's trim, evaluated on the surface and accepted
only when that door answers `In`.

## Both callers (CLEAVE `cleave/ladders`)

Section-loop role resolution (`boolean/join.rs`
`resolve_roles_geometric`) now reads the same ladder (`complex_side`)
over each loop's region faces. A curved region face is passed over
there too. A loop whose regions are all curved, with every vertex and
edge midpoint on the other boundary, reads undecided, and the other
loop decides. If neither loop decides, the join refuses with
`JoinDesync` ("neither section loop's regions hold a decisive
witness"). No row reaches that refusal.

