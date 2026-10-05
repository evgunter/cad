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

## Under the `On` verdict (FUSE, branch `fuse/on-verdict`)

The uncut-shell caller no longer ends at `ShellWitnessExhausted` when
every witness read `OnBoundary` with none in band: it then asks the
`On` question (`shell_witness::on_verdict`). This row's shape reaches
it, and where the off-boundary curved face has no settled coincidence
pair it refuses `CoincidentShell { orientation: Unpaired { face } }`
instead. Still a refusal, still unreached by a
fixture; the fix above is unchanged. The join caller is untouched.

## Evidence (2026-10-03, `reach/arc-from-pairing`): the role read reaches it

The join's arc now comes from the germs, so three poses whose section
lies along a revolved ball's seam edges — the seam great circle in a
plane face of the other operand — get past the join's chords and stop at
the role read, `Join(SectionLoopUndecided)`: both section loops' region
faces are the ball's two hemispheres, and every witness the probe holds
for them lies on the plane face (measured on the pip: `Tally {
on_boundary: 4, in_band: 0 }` for each loop).

- the cube against the `y`-poled ball(0.3) at `(0.5, 0.5, 1)`, every op
  (`crates/sweep/tests/tilted_sphere_pair.rs`,
  `a_pip_with_its_seam_in_the_cubes_top_stops_at_the_role_read`;
  `germ_coplanar_conic.rs`, the y-poled pip);
- the slab `[−2, 2]² × [0, 1]` less the unit ball at the origin
  (`m5_pr9c_sphere_doors.rs`,
  `the_die_pips_shape_stops_typed_at_its_section_roles`);
- the cylinder of `review_ring_clearance_r1_probes.rs`
  `r1_diag_cylinder_pierces` against a `y`-poled ball(0.16) on its top
  cap (recorded, not asserted, there).
