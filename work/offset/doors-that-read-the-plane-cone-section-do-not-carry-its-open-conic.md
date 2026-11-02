---
id: doors-that-read-the-plane-cone-section-do-not-carry-its-open-conic
kind: issue
title: the Boolean germ, the split chord and the per-chart and general offset doors refuse the plane×cone parabola or hyperbola the axial door now rests
status: open
opened: 2026-10-28
priority: P2
cost: H
---


Found by `a-classified-pairs-section-states-the-bounds-that-retire-the-tube`.
The axial door now mints a moved cone/cap hyperbola between its corners and
the arc rests, its tube retired by the plane×cone section's stated bounds
(`geom_brep::intersect::tube_bounds`, C2). Every other door that reads
`geom_brep::plane_cone_section` still refuses the open conic, typed:

- the Boolean's germ frame, `crates/topo/src/boolean/join.rs`
  (`pk_germ_frame`, `FrameError::OpenConic` →
  `BooleanError::GermSectionOpenConic`): it has no crossing layer for a
  NURBS carrier, which is still the reason;
- the split's chord, `crates/topo/src/chord_join.rs`
  (`SplitJoinError::OpenConicChord`);
- the general offset door, `crates/topo/src/offset_derive.rs`
  (`SectionVerdict::OpenConic`, at its plane×cone arm);
- the per-chart door, `crates/topo/src/replace_face.rs` (its plane×cone
  edge check, `SectionVerdict::OpenConic`).

The split's and the offset doors' reasons said the arc "cannot yet be
certified on a cone"; that is no longer the reason, and the texts now say
the door does not carry it. Each door minting the arc (the window from its
own corners, `ConicBranch::arc`) would let it rest wherever the section
states the bounds, as the axial door does. Ev's reservation on the
exemption ("i'm still not totally sure about having special cases for the
parabola and hyperbola", PR 5652) bears on how far to spread it.
