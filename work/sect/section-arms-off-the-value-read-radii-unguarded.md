---
id: section-arms-off-the-value-read-radii-unguarded
kind: issue
title: The C5 arms the section value does not hold yet, and the frames over them, read radii not guarded definitely positive
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Found by `the-section-of-a-surface-pair-is-one-object` (2026-10-27),
applying `cylinder-sphere-tangency-is-decided-twice-…`'s radius ruling.

## Measured

The ruling — a radius not definitely positive refuses
`DegenerateOperand` — holds on the section value's one arm
(`geom_brep::CylinderSpherePose::decide`, `cs_cylinder_radius`,
`cs_sphere_radius`) and on `cone_cylinder_section`
(`coc_cylinder_radius`). The arms the value does not hold yet read a
stored radius unguarded:

- `geom_brep::plane_cylinder_section`, `plane_sphere_section`,
  `sphere_sphere_section` and `cylinder_cylinder_section`: no radius
  question before their pose trileans.
- `topo::boolean::join::cylinder_pair_axes` levers
  `bool_germ_frame_axes_parallel` on the signed `r1.max(r2)` (the
  frame's `FrameExtent::Radii` and `Wall` arms), so a negative stored
  radius shrinks the lever.
- `topo::boolean::section_cert::classify`'s sphere, sphere pair and
  cylinder pair arms read radii as stored.

Bodies at rest hold `r > 0` against ZERO (tier-3 check 1), so a radius
in `(0, band]` validates and reaches these arms.

## What a fix has to supply

Each arm's radius guard, the section value's posture, as the arm moves
into the value (`plan.md` steps 2 and 3), with the frames and the
classifier's arms retiring behind them.

Moved from SECTRUNG to SECT on 2026-11-02, by `git mv`, when SECTRUNG merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.

## 2026-11-02: a cited site retired (SECTARMS stage 3)

Branch `sectarms/cylinder-join-reads-value` retires `topo::boolean::join`'s
`cylinder_pair_axes`, `cylinder_pair_frame`, `crossing_cylinder_reading`
and `parallel_radical_plane`. The join now reads two cylinders on the
section value (`join::cylinder_read`), and the parallel lane takes its
plane from `geom_brep::CylinderCylinderPose::rulings_plane`. The hit
above that cites one of those sites no longer exists. The rest of the
row stands as filed.

## 2026-11-02: the rulings plane retired too (SECTARMS stage 3b-i)

Branch `sectarms/cylinder-carried-closed-form` retires the parallel
lane's plane: `geom_brep::CylinderCylinderPose::rulings_plane` and
`topo::boolean::join`'s `GermLane::Rulings`, `rulings_are_straight` and
`rulings_normal` are gone. A parallel cylinder pair now rides its
carried rulings (`chord_join::carried_chord_spec`'s closed-form arm).
The note above that names the rulings plane no longer points at live
code. The rest of the row stands as filed.
