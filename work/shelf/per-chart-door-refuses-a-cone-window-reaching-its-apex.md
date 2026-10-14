---
id: per-chart-door-refuses-a-cone-window-reaching-its-apex
kind: issue
title: the per-chart offset door refuses ApexWindow on a cone chart whose faces reach their apex
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-of-a-cone-tip-refuses-at-the-nappe-decision]
---


`topo::replace_faces_offset` on the two cone half-faces of
`common::shell_operands::cone_tipped_vessel(0.5, 0.6, 0.4)` refuses at
either sign of `d = ±0.05`, `Tol::witness()`:

    ApexWindow { v_min: -0.6403…, v_max: 0.0, shift: ±0.04 }

`face_nappe` now decides a face that reaches its apex (on the nappe its
other corners stand on, the apex corner moving with the apex), and the
axial door (`offset_charts_together`) offsets the same chart at both
signs to the closed form, with the apex vertex on the moved apex. The
per-chart door's apex-window gate in `replace_face.rs`
(`offset_apex_nappe`, then `offset_apex_window`) still reads a window
whose near end IS the apex as one that has not cleared it, and refuses.

No `shell` path reaches it: an axial body takes the axial door for the
cavity and the lift. A direct per-chart offset of a cone tip, or a plain
cone solid (apex and a planar base, where the per-chart door's
re-description against the untouched plane is exact), does. Whether
the gate should take `v_near = 0` as cleared, with the apex vertex
moved to the moved apex rather than transported, is this item. It was
left out of the change that fixed the nappe decision because
`replace_face.rs` was being changed in parallel.
