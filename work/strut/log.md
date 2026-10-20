# STRUT — the log

## 2026-09-20 — opened

Cut out of CARVE, which was carrying 73 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CARVE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

11 rows arrived by `git mv` with their ids, bodies and history
unchanged. CARVE keeps its band 5600-5699; band 7900-7999 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/src/extrude.rs`. `extrude.rs`'s walls state `wall_sense` in their spec (the post-mint `set_face_sense` is gone). (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; `sweep::extrude` reads `.diag` and behaves as before, a zero arm's payload now quoting its margin. (TOPO implementer)

- 2026-10-01 — Seam note from AUTHOR: Ev ruled on #3551 that an extrude's distance is a positive depth with a structural `side`, and that a negative depth refuses with a recourse naming `side`. The work is filed on EDIT as `extrude-distance-is-a-depth-and-a-side`. Its kernel half is `sweep::Extrusion`/`ExtrudeError` (CARVE/STRUT) and its eval wiring `wire_extrude` (WIRE). The rule's follow-ons are `carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` and `edit/pattern-spacing-is-a-signed-size-beside-a-direction`. (AUTHOR orchestrator)

## 2026-10-02 — note from SHOW

`seed-finder-home-reads-only-the-y-station` cites `no-public-rim-arc-selector`,
which has no file anywhere in `work/`. This was found by SHOW's review of PR
3787 and predates it. It is a dangling reference for STRUT to resolve, either
by re-pointing it or by filing the row it names.

## 2026-10-06 — two rows re-homed here from CARVE's second cut

CARVE measured 62.5 budget points and was cut on its priority seam
again (`work/carve/log.md`, same date). Two of its rows fit STRUT's
charter (one rule with several homes) and moved here by `git mv`, with
their ids kept and priced at the cut:
`extrude-arc-walls-are-ruled-in-n-not-w` (P2, M; extrude rules a line
leg's wall in `w` and an arc leg's in the sketch normal) and
`sweep-body-makes-every-caller-derive-its-start-frame` (P3, M; five
callers derive one start frame by hand). STRUT now carries 23.5 points.

Signed: (CARVE orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4450, merged at `59cdb05871`). `topo::DihedralReading::of_lever` / `of_must_carry` are the one map from a dihedral escalation to a reading; use them, not `(Lever(e.rung()), e.diag())` by hand. `sweep::blend` has two new closed decisions, `ContactArm` and `ContactWedge`, and `ContactSecondOrder` now offers the tolerance (`AnySign`). `ExtrudeError`/`RevolveError` `SliverJoin`/`SliverRim` now carry `reading: DihedralReading`, and their second-order text reads through `Indeterminate::undecided`. The editor-core fault line for `Escalated(contact)` is exactly at the 75-word budget. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
