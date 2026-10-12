---
id: a-fitted-face-trimmed-by-a-section-has-no-volume-rule
kind: issue
title: A fitted face trimmed by a plane x fit section is past the trimmed exact lane's Newton-Cotes window, so tier 3 refuses its volume (VolumeUncomputable)
status: open
opened: 2026-10-09
priority: P3
refs: [a-fitted-wall-has-no-section-with-a-moved-cap, trimmed-quadrature-composite-rounds]
---

Found by SHELL's plane × `Approx` unit. `crates/sweep/tests/common/approx.rs`'s
`box_with_approx_cap` with one side wall moved by `topo::replace_face_offset`
(`crates/sweep/tests/encl_curved_loft_shell.rs`,
`a_moved_plane_meets_a_fitted_cap_along_their_certified_section`): the
cap's edge with the moved side is now a plane × fit section with a
`General` image on the fit's chart, and tier 3's check 7 refuses the
cap: `VolumeUncomputable { source: Face { source: QuadratureUnsupported {
TRIM_NC_WINDOW } } }` (`crates/geom-brep/src/props/quad.rs:4246`, the
exact lane certifies `p_u + p_v ≤ 4`; the fit is past that). The
constant's own text says the composite trapezoid fallback "is not built
(no fixture reaches it)"; this fixture reaches it, at ε = 1e-9 and 1e-6.
The structural phase (every edge certificate) passes on the same body.

The rule this face needs is the composite fallback
`work/iso/trimmed-quadrature-composite-rounds.md` (P1) scopes; that item
fenced it on "no fixture reaches it", which this fixture falsifies, and a
dated note there says so. Building that unit closes this item.
