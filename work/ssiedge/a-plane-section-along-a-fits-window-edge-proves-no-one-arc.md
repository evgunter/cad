---
id: a-plane-section-along-a-fits-window-edge-proves-no-one-arc
kind: issue
title: A plane x fit section lying on the fit's window edge refuses its tube's one-arc proof undecided, so a moved fitted cap whose face is its whole fit cannot re-certify its edges
status: open
opened: 2026-10-09
priority: P2
refs: [a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section]
---

Found by SHELL's fitted-corners unit. `crates/sweep/tests/r1_lane0_e2e.rs:41`'s
`the_f64_seam_answers_every_public_door` moves the unit box's cap, swapped
for a bilinear NURBS patch over exactly the face (`[0, 2]²` at `z = 1`),
by 0.05 through `topo::replace_faces_offset` and through
`topo::replace_face_offset`. The cap's offset fit covers the same window,
so each side plane's section with it is the fit's own boundary row
(`crates/topo/src/offset_derive.rs:692`, `level_row`). The edges derive and the corners solve; the
surface swap then refuses re-certifying the first one:

`Op { edge: None, error: RechartFalsifies { door: SetFaceSurfacesDescribing,
edge, error: PlaneNurbs(TubeNotOneArc { rungs: 20, cause: Undecided(
Indeterminate { margin: MarginDiag(Invalid), predicate: Some("ssi_tube_one_arc"), .. }) }) } }`

at ε = 1e-9, for d = +0.05 and −0.05. The `Undecided` arm is
`limb_three`'s (`crates/geom-brep/src/ssi/certify.rs:1130`, the
`Shortfall::Undecided` arm at :1175): every rung was a
graph whose one-arc walk read no margin it could classify. The same pair
one window wider (the patch over `[-1, 3]²`, so each section crosses the
fit's interior) certifies every edge and builds
(`crates/sweep/tests/encl_curved_loft_shell.rs:719`,
`a_moved_fitted_cap_stands_its_corners_on_the_held_sides`), so the
refusal reads as the tube's walk leaving the fit's chart along a
section that lies on its window edge. Unmeasured beyond that: which rung
and which box the walk stopped in.

Owed: a one-arc proof for a section on the fit's window edge (the
row is exact structure, `interior_iso_u`'s boundary case), or a refusal
that names the window edge rather than an undecided margin. Either makes
the box with a fitted cap that is its whole fit the first fitted face to
move and re-certify, and the row above re-baselines.
