---
id: described-net-two-state-reads-hand-a-poisoned-net-the-described-arm
kind: issue
title: Thirteen placeholder-or-described reads outside tier-3 check 1 hand a described net carrying poison to the described arm
status: closed
opened: 2026-09-05
priority: P0
cost: H
closed: 2026-10-10
---


## What

TOPO's S330 (PR 1923) gave tier-3 check 1 a third answer for a
`Surface::Nurbs` payload — placeholder, described, or described-and-
carrying-poison — because `geom`'s rule (`crates/geom/src/net.rs:128-131`)
says a net poisoned in some channel is corrupt DESCRIBED geometry that
must fail at each consumer's described arm. The dual review swept the
tree for the other consumers that ask "placeholder or described?" and
hand everything else the described arm, and found thirteen, none of
which S330 touched (other programs' files):

- `crates/topo/src/pcurves.rs:273` (TRIM)
- `crates/topo/src/props.rs:1531` (code-quality Track M / S-CERT)
- `crates/topo/src/replace_face.rs:1211`, `crates/topo/src/transform.rs:370, :495` (SHELL)
- `crates/topo/src/census.rs:1741, :2284, :3488` (CURVED; `:2272-2275`'s
  argument that no public-door body carries a placeholder is now
  narrower than it reads, since check 1 also bars poisoned nets — S350's
  live path is the in-src rows and second callers)
- `crates/mesh/src/chords.rs:508`, `crates/mesh/src/trimmed.rs:200` (S-MESH)
- `crates/step-import/src/adopt.rs:530, :710, :952` (EXCH)

The reviewer did not establish that any is WRONG — several refuse
downstream by escalation — only that the invariant now protects one
door and the sweep was never run. The tool is the door S330's fix pass
lands: `NurbsSurface::net_state() -> NetState` (`crates/geom/src/surfaces/nurbs.rs`),
one method answering the three states, so a consumer's match is
exhaustive over them instead of over a guard pair.

Two notes from the same review, same rule, for whoever takes this:
`Real::is_poison` is NaN-only at `f64` (`crates/geom/src/lib.rs:92-95`),
so a net of `+∞` control points is DESCRIBED and passes check 1 while
describing no locus — geom-core's policy, not S330's; and
`crates/topo/src/r2_probes.rs:1-2` says "committed to the reviewer's
own branch only" while sitting on main.

## Home

`work/issues/`: the thirteen sites span six programs and no one owns
the class; a taker claims it by moving this file.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/code-quality/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). A structural class over thirteen sites in six programs' files — exactly what `work/code-quality/` holds until a program claims it, and what the K–X partition refuses as a program of its own. The tool (`NurbsSurface::net_state()`) is PROPS' door; each consumer's arm is its owner's rider, and this row is the census they close against.

## Re-homed to PIPE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PIPE collects the topology pipeline's own rows: the two engines' shared
cores, the census arms, and what a refusal there is allowed to say. This
row is one of them.

Its class at the cut was **H** — thirteen sites in six programs' files;
each arm needs owner's judgement. The class is a dispatch estimate made
by reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Closed (routing pass)

Swept on `origin/main` at the cut of `pipe/described-net-routing`
(2026-10-10). This pass only routed: it filed rows and changed no
kernel code. Sites are cited by name, and line numbers are approximate.
**Pass 1** grepped `is_placeholder|net_state()|NetState::` over
`crates/*/src`, which also catches `Surface::is_placeholder_chart`. That
pattern cannot see a consumer that never asks the state question. **Pass
2** was aimed at that gap: `Surface::Nurbs(` arms (137 hits) and direct
`.control()` reads outside `geom`, triaged for raw-`f64` or per-axis
readings of a net. Where a poisoned net reaches a reading that goes
through `Decide`, an `Interval` (`from_f64(NaN)` is NaI), or a finiteness
gate, it escalates or refuses. Only an unguarded `f64` fold or per-axis
box can answer wrongly.

The thirteen, as they stand now:

- `topo/src/pcurves.rs` `DescribedChart::of` (was `:273`; no program claims the file). **Fine.** A poisoned chart reaches the mint and the validator. There `chart_stretch_sup`'s NaN arm escalates in `spline_gap_closes` (`PinMiss::Escalated`), and the certification lanes refuse (`ChartSpeed(NotFinite)`, foot point inconclusive). No wrong answer.
- `topo/src/props/quad_lane.rs` `nurbs_face` (was `props.rs:1531`; no program claims the file). **Fine, by reading.** The net enters as `Interval::from_certified` (NaI), and `geom_brep::props::quad`'s `nurbs_patch_face_rounds` refuses an uncertified flux or area (`QuadratureUnsupported`, "… a non-finite net"). Not probed. That needs a loft wall with its rows kept.
- `topo/src/replace_face.rs` `mint_offset` (was `:1211`; SHELL/SHELF). **Fine.** The offset fit door refuses: whole-patch `Meter`, or `OffsetFitError::NonFiniteSample`, whose doc names exactly this case.
- `topo/src/transform.rs` `map_surface` / `map_curve` NURBS arms (were `:370`, `:495`; SHELL/SHELF/OFFSET). **Wrong (latent).** The arms map a poisoned net instead of refusing it, and a probe shows a 45° rotation turns an all-`x`-poisoned net into the placeholder. Filed `transform-rigid-maps-a-poisoned-net-into-the-placeholder` (SHELF, P3/E).
- `topo/src/census.rs` reach-box `ControlNet` arm (was `:1741`). **Excluded: S350's lane.** For that lane, a probe of `face_reach` on a poisoned wall answered `Some((NaN, 0, -1), (NaN, 0, 2))`, which is the partial box S350 describes.
- `topo/src/census.rs` arm-1 placeholder skip (~5465; was `:2284`; RESTREAD). **Fine once S350 lands.** The poisoned net is correctly not skipped and goes on to `reach_box`, the described arm, whose fault is S350's. The comment's argument that "no public-door body carries one" holds for poisoned nets too, since check 1 bars them.
- `topo/src/census.rs` (was `:3488`). **No live site on main.** The only other two-state read left in the file is the in-src test helper `swap_placeholders`.
- `mesh/src/chords.rs` `nurbs_tighten` (was `:508`; CHORD/TESS). **Fine.** `face_bound` → `nurbs_cell_grid`: `patch_bound::comp_nets` reads the net as `Interval::point` (NaI), and the per-cell finite check refuses `UnsupportedNurbsFace` ("second-derivative hull is unbounded/refused"). This is in the chord pass, before any lane runs.
- `mesh/src/trimmed.rs` placeholder arm (was `:200`; TESS). **Fine.** A poisoned face has already refused the whole tessellation in the chord pass, and `face_cells` answers `MissingEntity` for an unfilled face.
- `step-import/src/adopt.rs` arc-rim wall pick, iso-candidate wall loop, `nurbs_plane_pair` (were `:530`, `:710`, `:952`; EXCH), plus `entities.rs`'s recognition gate. **Fine.** No NaN can enter an imported net: the lexer's number token is digits, sign, `.` and `E`, `as_real` parses that, and `length_scale` is an exact nonzero power of ten. So `Poisoned` is unreachable at import. `±∞` is reachable (see the first note below), and the import's closing `validate_geometric` refuses it.

The other pass-1 hits, outside the thirteen:

- `editor-core/src/mate/reach.rs` `face_reach` (MSOLVE). **Fine.** The NaN bound propagates through `Real::max`, and `eval`'s `reach_over_cache` refuses `ReachRefusal::NoFiniteBound`.
- `geom-brep/src/certify.rs` `resolve_iso`, `plane_nurbs_pair` (no program claims the file), and `edge_nurbs.rs` `plane_nurbs_limbs` / `chart_image` / `chart_foot` (ISO). **Fine.** `SsiOperand::nurbs` refuses `SsiError::ChartSpeed(NotFinite)`, which the probe saw, and projection onto the net does not converge (`FootPointInconclusive`).
- `geom-brep/src/pcurve_cache.rs` `chart_stretch_sup` / `chart_stretch_inf` / `run_{fitted,iso_arc,iso}_checks` (PCERT/PCTAIL) and `pcurve_cache/projected.rs`. **Fine.** The sup arm is NaN and escalates in `decide`, `net_inf` answers zero on poison (the refusing answer), and the projected lane refuses a non-analytic chart.
- `step-export/src/writer.rs` `surface_kind`, `printable_carrier`, the `B_SPLINE_SURFACE` arm (EXCH/EXPORT). **Fine.** `fmt_real` refuses `NonFiniteReal` at the first poisoned control point. The kind is named "nurbs", not "poisoned", which is wording and not a wrong answer.
- `topo/src/attach.rs` `slot_chartless`, `landing_of_spec`, `wears_no_chart` (TOPO). **Fine.** A poisoned chart is not chartless, so it is held to the vouching rule. That is stricter than the placeholder gets.
- `topo/src/validate.rs` check 1 and `topo/src/merge_faces.rs` `MergeKind::of` already match three states.

From pass 2: the kind dispatches (`chart.rs`, `chart_region.rs`, `face_normal.rs`, `readback.rs`, `param_source.rs`, `boolean/*`, `editor-core/src/clearance.rs`, `sweep/src/blend/reach.rs`) give the placeholder no benign arm, so a poisoned net fares exactly as a placeholder does: refused, or escalated through `chart_stretch_inf`'s zero inf. The direct net readers checked were `boolean/boxes.rs` `ControlNet` (`nurbs_surface_aabb`, poison box since CERT-N2), `ssi/enclose.rs` `CellNet` (NaI channels, gated behind `SsiOperand::nurbs`), `offset_fit.rs` `recentre_origin` (NaN-dropping `f64::min`, but it only places a cost centre and NaI channels refuse), `offset_derive.rs` `translates_along` (an undecided margin does not hold), and `mesh` `patch_bound` (NaI). **Blind spot:** the 137 `Surface::Nurbs(` arms were triaged, not each traced to its refusal. Curve-side nets (`NurbsCurve3::is_placeholder`, which has no `net_state` twin) were out of scope except where `transform.rs` shares the fix.

The two notes:

- `±∞` nets read as described. **A real but narrow gap.** At rest it is closed by `validate.rs` check 1 (`net_is_finite`). The three-state door still answers `Described`, and `merge_faces` already takes it as `Curved`. Filed `net-state-reads-an-infinite-net-as-described` (FLUX, P3/M, design).
- `topo/src/r2_probes.rs`'s header said "committed to the reviewer's own branch only". **Fixed as a drive-by** in this branch.
