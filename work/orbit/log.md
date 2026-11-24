# ORBIT — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. Rows dispatchable: 8, for 25 points.
— (REACH orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)

- 2026-10-10 — CONTACTHOLD closed `census-backstop-clears-a-curved-pair-only-along-world-axes` as a duplicate of `census-backstop-separates-curved-pairs-only-along-world-axes`, which it claimed from RESTREAD and measured. (CONTACTHOLD orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) re-words one doc in `topo/src/boolean/ops.rs` (the lane's stop is its floor or its work wall). No code change. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `boolean/ops.rs`'s volume backstop only (see CLEAVE's note). (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-25 — **Re-cut (refold approved in chat):** moved `the-lilys-stem-and-arch-walls-have-no-certificate-arm` to SECT, `delete-the-boolean-operand-edge-gate` to SPLICE by `git mv`, ids and bodies unchanged, each with a dated Moved note. SPLICE (`work/splice/`) is the new track for a body built by loft or sweep entering every boolean; SECT now carries the torus pairs it is built for.

## 2026-10-26 — a row moved to WIRE (WIRE orchestrator)

`a-sphere-recut-grafts-its-rotated-shell-under-keys-no-record-carries` moved to `work/wire/` by `git mv`, with its id unchanged, at P1. PR 5795's review found it misnames a published name (`emit_ball_cut_by_brick`'s pole). The fix reads naming through PR 5795's `Recharts` map, which is WIRE's lane.

Signed (WIRE orchestrator).
- 2026-10-27 — **Seam (ATREST):** `atrest/a-crossing-touch-is-interference` touches this program's ground. A touch between two solids that the census's touch analysis decides is a crossing is now `ValidationError::TouchInterference { a, b }` (the touch's two `Cell`s), which `census_interference_solids` reads; `Undecided::MixedTouch` ("one passes into the other where they touch") retires. A pair that a standing finding already decides interferes (a pierce, a same-side crossing, two coincident edges' wedges overlapping) no longer also refuses `CensusUndecidable` beside it. New exhaustive-match arm: `boolean/ops.rs`, `checks/at_rest.rs` `finding_solids`, pncad-py `touch_interference`. Restated rows: `contact5_gate_and_beam` (the sunk beam and ridges), `contact7_touch_sweeps::refused`, `coincident_line_wedges` (the pinwheel), `a_vertex_read_by_two_sector_passes` (its allowlist). (ATREST crossing-touch lane)
- 2026-10-26 — Seam note from WIRE (branch `wire/re-chart-rows-name-input-cells`, row `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`): `boolean/ops.rs` `apply_recuts` now returns each re-charted shell's cells in the re-cut operand (`Recharts`), and `respell_through_cuts` (which now takes the rows, not the body) reads each such cell as the least face of the input shell it was turned over. A `Meet` or `OnCarrier` row keeps its relation on that face; any other relation naming a re-charted cell refuses `FallbackExtentUnsupported`, except a face of a one-face shell. The debug check `rows_name_input_cells` from `cleave/rows-name-input-cells-guard` lands with it at `boolean_op_with`'s exit. No expected value in `topo` or `sweep` moved; rows on the four re-chart tests now name ball faces where they named turned edges. (WIRE implementer, wire-rechart)
- 2026-11-02 — **Seam (BOXES):** PR 6336 retires the split gate's own torus arm (`gate_face_reach`/`torus_window_reach`, K name `split_gate_torus_ring`); dated notes on the rows here that cite them. (BOXES orchestrator)
- 2026-11-21 — **Seam note from TESSLIE:** on main at 1597f5deb, `sweep offer_rows::every_sphere_offer_passes_just_below_it` fails deterministically at offer_rows.rs:333 ("false offers"). Two cases fail. `nested_off_axis_in_the_zero_band_union` / `_subtract` quote None and offer Some(4.999e-11) at margin 5e-10. `ball_beside_a_slab_in_band_union` quotes Some(-5.4999992e-9) and offers Some(5.499e-10) at margin -5.5e-9. It was measured by a TESSLIE lane, it is not TESSLIE ground, and this program's `closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut` lists the test among rows that move. Reported, not fixed. (TESSLIE orchestrator)
- 2026-11-23 — **From HELDRECORDS: `closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut` is re-banded P0.** The tour's die, with two pips dragged together (a valid part), now refuses `UnsupportedTurnedCell { Edge }` at its union; before PR 7516 fixed the n-ary door's lineage it panicked. That refusal's way through is this row. The evidence is on the row. (HELDRECORDS orchestrator)

## 2026-11-23 — a sitting opens (ORBIT orchestrator)

Track taken (`status: active`). Fable is unavailable for this sitting
(Ev, in chat: "fable usage is out, so run everything on opus, using two
opus reviewers/designers as indicated"). So every lane is Opus 5.5; a
dual review is two Opus reviewers on one frozen head, and a design fork
is weighed by two Opus designers. Under `docs/MODEL-COMPARE-PROTOCOL.md`
rule 12 no draws are taken and no model-compare row is recorded for a
unit dispatched in the outage. The design-fork row is recorded with both
designers' models named (the Opus/Fable arm is not run).

Load on taking it: 34/30. The unbanded
`a-section-certificates-witness-point-is-picked-off-the-axes` is banded
P2, cost E: unmeasured whether any pose reaches W3 with a circle partly
inside a face, and the fix (the sphere's own `u_ref`) is named in the
row. That leaves 31/30; not split, since this sitting works the whole
P0/P1 slate.

Dispatch, all at once (lanes that build run in their own cloud sessions;
this container has 4 vCPUs):

- `closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`
  (P0, H): implementer. Tier **dual**: it retires a mechanism (the
  re-chart) under 21 moved rows, and how every sphere escape is cut is
  hard to change later. The lane also checks the two P2 re-chart-key rows
  (`a-re-cut-booleans-verdicts-…`, `a-step-given-pairs-…`), which the
  retirement may moot.
- `a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key`
  (P1, M): implementer, in parallel with the P0 (coupled ground:
  `apply_cut_ins`; each lane is told of the other). Tier **single, full**:
  a carry that lands a record on the wrong piece is a correctness claim.
- `separation-grants-disjointness-only-along-world-axes` (P1, M):
  implementer. Tier **single, full**: a disjointness certificate.
- `boolean-gate-refuses-a-pair-the-census-reads-apart` (P1, M):
  implementer, measure first. Tier **single, full**: the reach it shares
  is read mid-operation. Coupled with the separation lane through
  `boolean::separating`; each is told of the other.
- `a-torus-near-a-tilted-cut-stops-at-the-extent-scan` (P1, H):
  implementer, measure and scope first. What is left is the plane ×
  torus germ's general rung (`RoutesToGeneralRung`), next to GERM's
  `co-surface-direction-table-and-spiric-plane-door` (PR 7431). If the
  rung needs a representation decision the lane stops and reports it.
  Tier decided once scoped.
- The two torus-touch rows (`design: true`) are one fork: two Opus
  designers, concurrent, one statement of the problem.
- 2026-11-23 — **Torus-touch fork weighed.** Two Opus 5.5 designers, concurrent, one problem statement. First reports split on saddles and the gap margin; after one round both recommend reading the touch by its connected near band, saddles included, with the sweep premise S′ stated and pinned. The two rows close into one unit, `a-torus-touch-is-read-by-its-near-band`; `[ev]` PR 7587 asks Ev to approve it or to choose the extremes-only claim. The build is dispatched now (`orbit/torus-touch-build`), with the saddle admission kept as one separable commit, since everything else is common to both answers. Tier **dual**: it widens what a soundness certificate certifies. Off-question findings: a decided on-face touch told with R-tan's "not certified" sentence (noted on SECT's `r-tan-carries-the-arms-proof-gaps-as-tangent-carriers`); torus × plane classified twice (folded into the unit's build). (ORBIT orchestrator)
- 2026-11-23 — **Friction:** a cloud review session created with a short commit SHA as its checkout revision dies at clone (`ref_not_found`) and sits PENDING with no other sign. Three reviews (7611 ×2, 7588) lost 45 min that way before an hourly check-in read their events. Launch a review on the PR's BRANCH and have the brief check out the frozen SHA. (ORBIT orchestrator)
- 2026-11-23 — Reviews in flight: PR 7593 (cut-in carry), reviewed once: APPROVE-WITH-FIXES; the fix pass takes the edge-lineage sibling (cut-in arc splits never reach the re-entered op's edge pieces, so a vertex-on-edge record names the wrong arc) in the same PR, and files `records::landings`' silent drop as a row. PR 7611 (torus touch at extremes): dual, two reviewers on `e5cbc7cd4`. PR 7588 (pair doors read apart): single full, on `404224d83`. (ORBIT orchestrator)
- 2026-11-23 — **PR 7611 dual review: both NOT-MERGEABLE-AS-IS, bilateral MAJOR, demonstrated by each.** A parallel touch read off the exact pose builds across a real crossing inside both faces (a sleeve sector with its axis 0.5ε off; a D-pin in the hole), where main refuses R-tan. **Class, not instance:** a certificate reading taken at the EXACT pose (`at` at `u_ref` on a coaxial reading that dropped an in-band offset or tilt) and then trusted across every pose the band admits. The near band stays connected; the side argument for S′ needs `at` at the actual pose's deepest point. Fix pass: parallels leave the unit (parked with the saddles on GERM's line × torus door, `torus-parallel-touches-wait-on-the-line-torus-door`); every point arm reads `at` from the actual pose (the parallel-wall point touch drops an in-band tilt too, possibly on main); rows that make `at`'s placement matter. Ev told on PR 7587; the designers' "any point of a parallel gives the same verdict" was false off the exact pose, which neither designer nor I caught before the build. (ORBIT orchestrator)
- 2026-11-23 — **P0 escape, PR 7602: the re-chart is retired; the die does not build yet.** Every sphere escape is now cut along a meridian. The die's eight edits pass the turned seam and refuse at the union on a surviving chart cut: C12(5)'s `PeriodClosure` keeps `merge_faces` from gluing it. Claimed HONE's `a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick`, re-banded P0 with `design: true`; two Opus designers dispatched. PR 7602's tier is **dual** (two reviewers on `05f03d174`). The near-pole circles now reach TESS's known mesher limit: evidence is on `work/tesslane/a-tilted-circle-near-a-sphere-pole-…` (P3), whose band TESS weighs. (ORBIT orchestrator)
- 2026-11-23 — **The surviving chart cut: designers agree.** Two Opus 5.5 designers, first reports concordant: a cut the kernel makes for its own reasons is transient. Its door hands the record into the re-entered op, and one step before the coplanar merge deletes each surviving piece by record (kef / kev / kemr), with no carrier verdict. That is what the ratified seam clause ("the kernel adds none") already requires, so the build runs now: `orbit/transient-chart-cut`, stacked on 7602, tier **dual**. `[ev]` PR 7639 asks only whether `[edges]` should say so, and for C12(5)'s restatement (its stated rationale is stale). Rows the build lane files: no maximal-faces check at rest (`design: true`); the interior-loop class on the crossings path (`design: true`). (ORBIT orchestrator)
- 2026-11-24 — **PR 7593 (cut-in carry) merges.** Tier: single full review (APPROVE-WITH-FIXES: the edge-lineage sibling was fixed here, and the silent drop in `records::landings` was filed to HELDRECORDS), then a verify (APPROVE), then a last pass (one home for the piece order, a row for the found-row key split) on my read. CI's reds are main's filed rows: the prism corner at 1e-12, `square_search`, and the python suite's family-union red (GREEN, P1). No model-compare row: Fable outage, rule 12. (ORBIT orchestrator)
