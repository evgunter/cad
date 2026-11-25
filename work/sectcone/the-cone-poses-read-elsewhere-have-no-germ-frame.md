---
id: the-cone-poses-read-elsewhere-have-no-germ-frame
kind: issue
title: The cone poses the value reads elsewhere answer NoArm at the germ frame: a sphere on a cone's axis, a coaxial torus, two cones of one aperture on parallel axes
status: open
opened: 2026-11-24
priority: P2
cost: M
refs: [cone-germ-pairs-against-a-curved-face-have-no-section-frame]
---


Filed by the join's cone-pair unit (`the-join-reads-the-cone-pairs-on-the-value`).

The swept cone poses ride the carried lane: the section table holds every
cone × cylinder, cone × cone and cone × sphere pair the value classifies,
and `topo` `join.rs` `rides_carried` carries them. Three cone poses the
value reads `ConeRead::Elsewhere` (`pair_section/cones.rs`) or reads on
their closed form (`KtStance::Coaxial`) are held by no table, so a germ on
them asks the frame, and `join.rs` `conic_section_frame_at` answers
`FrameError::NoArm`, which the Boolean refuses `GermFrameUnsupported`
(`join.rs` `frame_refusal`). The row
`join.rs` `a_cone_pair_read_elsewhere_refuses_its_frame_typed` pins the
first and the third.

- **A sphere on a cone's axis** (`ConeRuledPose::decide_sphere` reads
  `Elsewhere`): the section is circles about the common axis
  (`cone_sphere_section`). Every circle turns about the cone's axis, so
  the frame is that axis line, the loops sharing it, as plane × cone's
  axis-normal circles do (`pk_germ_frame`). A ball sunk into a
  countersink is this pose.
- **A torus coaxial with a cone** (`ConeTorusPose`, `KtStance::Coaxial`):
  likewise circles about the common axis (`cone_torus_section`).
- **Two cones of one aperture on parallel axes**: the difference of their
  forms `Q(p − k₁) − Q(p − k₂)`, `Q(w) = cos²α|w|² − (d·w)²`, is linear,
  so the section lies in the plane `n·p = c` with
  `n = cos²α·Δ − d(d·Δ)`, `Δ = k₁ − k₂` (never zero: `M = cos²α I − ddᵀ`
  is invertible and the common apex is read before). The section is that
  plane's conic on either cone, the shape `pk_germ_frame` reads, but the
  plane is computed: its margins need the rounding of `n` and `c` charged,
  and an open conic refuses `GermSectionOpenConic` as plane × cone's does.
  Two countersinks of one tool, overlapping, are this pose.

What a fix supplies: each pose's frame from its closed form, read where
the frame is (`conic_section_frame_at`), and the loop a germ takes decided
where it is minted (`germ_branch`) for the shared-axis circles. Witness:
a ball in a countersink under every op, and two overlapping countersinks.
