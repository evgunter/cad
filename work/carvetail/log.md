# CARVETAIL — the log

## 2026-10-06 — opened

Cut out of CARVE, which measured 62.5 budget points against 30, by
CARVE's orchestrator at the start of its sitting (`work/README.md`,
Track size: a track that grows past its ceiling splits along its
priority seam). CARVE keeps its seven P0 rows, which are exactly 30
points. The rest came here, except two rows that fit STRUT's charter
(one rule with several homes) and went there.

Three of the rows that came here are parked on the D10 hold (Ev's
`[ev]` PR #3990; `work/intent/d10-one-way-to-say-intent-is-unbuilt.md`).
Each row's body says why. The six unpriced rows were priced at the cut.
`revolve-angle-is-a-signed-size-beside-a-directed-axis` was parked too
and then re-opened in the same commit: Ev's ruling on PR 3941 already
answered its question (an angle about a directed axis stays signed),
and what is left is refusal text, which the hold does not cover.
- 2026-10-09 — Seam note from ENCL (PR 4450, merged at `59cdb05871`). `topo::DihedralReading::of_lever` / `of_must_carry` are the one map from a dihedral escalation to a reading; use them, not `(Lever(e.rung()), e.diag())` by hand. `sweep::blend` has two new closed decisions, `ContactArm` and `ContactWedge`, and `ContactSecondOrder` now offers the tolerance (`AnySign`). `ExtrudeError`/`RevolveError` `SliverJoin`/`SliverRim` now carry `reading: DihedralReading`, and their second-order text reads through `Indeterminate::undecided`. The editor-core fault line for `Escalated(contact)` is exactly at the 75-word budget. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)

## 2026-10-31 — a row in from CARVE, a row out to CARVEREST

CARVE's fourth priority-seam cut (`work/carve/log.md`) moved
`the-skin-cuts-an-arc-wall-at-each-sub-arc-joint` (P1, H) here: it is
the loft's arc columns, this program's charter, and it touches the
same `skin::segment_curve` as `loft-arc-columns-end-on-the-carrier-not-the-vertex`.
TESS's `tess/degree1-loft-slabs` branch reworks the loft's walls per
slab on the same assembly; read it before speccing. To stay inside 30,
`skinned-wall-weights-drift-an-ulp-along-the-stacking` (P3, E) moved to
CARVEREST, beside its other skin-rounding residue.

Signed: (CARVE orchestrator)

## 2026-10-31 — the half-turn P0 moved to CARVE

`a-half-turn-spine-sweeps-only-off-its-exact-tangents` moved to CARVE
to ride with `sweep-frame-is-a-minimal-rotation-from-the-start-tangent`.
Its designer pair found the half-turn is the start-tangent law's one
singular point on a planar path, so the frame unit (`carve/sweep-frame`)
closes both.

Signed: (CARVE orchestrator)
