---
id: offset-door-declared-transport-has-no-built-witness
kind: issue
title: topo: replace_face_offset's transport of a DECLARED description on a translated plane face has no witness the sweep builds
status: open
opened: 2026-10-06
priority: P2
cost: E
---


Found by `band/lamina-annulus-is-one-face` (BAND,
`lamina-plane-annulus-keeps-its-slit`).

`topo::replace_face_offset` carries a declared `MappedCurve` on an edge
of a face it TRANSLATES (a plane offset): the description arm keeps
the declaration with its placement moved by `d·n` rather than demoting
it to `Derived` (`crates/topo/src/replace_face.rs`). Its only witness
was `verbs_offd::the_moved_caps_own_seam_keeps_its_declaring_pushforward`,
which offset a full-revolve tube's top cap and read the cap's radial
seam — a declared chart image of the profile segment.

A full revolve now builds every plane wall as one face with no meridian
(`crates/sweep/src/revolve/full.rs`, `build_lamina`'s phase 4), so no
sweep output carries a declared edge on a plane face: the tube, a
partial revolve, an extruded block and the vessel were measured, and
the vessel's one declared edge was its cylinder's angle-π meridian. The
row was deleted with nothing to stand on, and the arm is now
unwitnessed. The neighbouring arm — an UNTOUCHED face's declared edge
re-anchored — has lost its witness too: a full revolve's curved wall is
now one face whose one meridian is the chart's derived seam
(`a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote`),
and the extruded two-arc drum, whose struts are declared, refuses the
cap offset on its declared cap arcs (`CarrierLaneUnsupported`), so
`verbs_offd::the_untouched_walls_declared_meridian_is_re_anchored` was
retired with its subject. So was the axial door's re-authoring of a
declared meridian ARC (`topo::offset_axial`'s `reauthor`), whose witness
`torax_axial::torax_re_authored_arcs_put_both_corners_on_the_moved_circle`
read the torus fixtures' angle-π meridians.

The rows: for each arm, build a face carrying a declared edge through
a door a user has (or plant a TRUE declaration through `set_edge_curve`,
which `CertCheck::MappedSource` now meters), offset it, and pin that the
declaration survives with its placement moved by exactly `d` along the
normal.

- 2026-11-02 — moved from SHELF to SHELL by `git mv` when SHELF merged into SHELL, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and SHELF had fallen far under it).
- 2026-11-23 — the untouched-face arm's witness retired with the full revolve's angle-π meridian (LATHE, `a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote`); the untouched-face arm and the axial door's arc re-authoring are now unwitnessed.
