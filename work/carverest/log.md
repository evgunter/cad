# CARVEREST — the log

## 2026-10-10 — opened

CARVE's orchestrator cut this program out of CARVE at the close of
CARVE's P0 sitting (`work/README.md`, Track size: a track that grows
past its ceiling splits along its priority seam). The sitting closed
six of its seven P0 rows, and its reviews filed rows faster than the
units closed them. Other programs filed two more during the sitting.
CARVE measured 37 points against 30.

CARVE keeps P0–P2 (20 points). The P3 and P4 rows came here by
`git mv`, keeping their ids. CARVETAIL, CARVE's earlier cut, could not
take them: the D10 hold released two of its parked rows and SHELL filed
one, which put it at 19 points on its own.

Two of the rows were filed on PR 4189's branch:
`material-jet-readings-take-plus-minus-order` and
`certify-residual-predicates-still-name-a-slot`. They arrive with that
PR's state sync.

## 2026-10-10 — PR 4189's two rows arrived

`material-jet-readings-take-plus-minus-order` and
`certify-residual-predicates-still-name-a-slot` moved here with PR
4189's state sync. The slate is 17 points.
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
- 2026-10-26 — (tracker sweep) Closed `sweep-places-vanishing-tangent-is-a-bare-f64-compare` as fixed: `47d913107f` replaced `unit_tangent`'s bare `n > 0.0` with `UnitVec3::new(.., SWEEP_PATH_TANGENT, band)` (`crates/sweep/src/skin.rs:1730`). The plan's slate now lists `cap-plane-refusals-end-as-typed-kernel-defects`, filed 10-20, and stands at 15.5 points. Re-read notes went on `corner-valence-reads-a-self-closed-edge-once` (all five readers stand, plus a sixth candidate in `reach.rs`) and `material-jet-readings-take-plus-minus-order` (it now reads through `second_order_walk`). Every other row was re-verified against the code and stands.
- 2026-10-26 — (split) Three rows arrived from BAND's priority-seam split by `git mv`, ids kept: `corner-config-indeterminate-has-no-real-body-witness` (P3 E) and `open-rim-end-corner-of-a-partial-revolve-does-not-classify` (P3 M), both `battery::corner_at`'s fold of a neighbour's refusal into `Indeterminate`, beside `corner-valence-reads-a-self-closed-edge-once`; and `blend-an-edge-in-another-shell-moves-predicate-4-at-a-cavity-corner` (P3 M), `walk_chains`' pairing at a cavity corner, beside `a-walked-chain-closes-at-a-corner-without-a-junction`. The slate is 21.5 points (`work/band/log.md` has the whole split).

## 2026-10-31 — a row in from CARVETAIL

`skinned-wall-weights-drift-an-ulp-along-the-stacking` (P3, E) moved
here from CARVETAIL, which took CARVE's skin-arc row in the same cut
(`work/carve/log.md`). It is skin-rounding residue, beside
`skin-union-mints-a-hairline-span-from-knots-an-ulp-apart`.

Signed: (CARVE orchestrator)
