---
id: a-certified-bound-refusal-reads-as-a-stored-contradiction
kind: issue
title: geom-brep: a refusal on a certified upper bound (hull limbs, TangentHull) ends as a kernel or file defect at rest and as the miss at the import door
status: dispatched
branch: encl/bound-kind
opened: 2026-10-10
priority: P3
cost: M
---



(Filed by the ENCL orchestrator: the converged core of design-fork row 104, from `hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door`. Both designers agree, and no ratified text changes.)

## What

`CertCheck::TangentHull`, `PlaneNurbsHull` (`ssi_hull_sup`, `ssi_hull_sup_chart`) and the AnalyticRung3 hull limb (`net_offset_sup`) refuse on a certified **upper bound** on the miss. Two places read that bound as a measured miss:
- **At rest:** `Unsized::LastResort` (`crates/geom-brep/src/recourse.rs`) ends a sign-certain arm in `KERNEL_OR_FILE_DEFECT_ENDING`. That is justified for a fitted carrier's definite miss, but false for a bound: a loose hull contradicts nothing.
- **At the import door:** `MissReading::Definite` reads the bound as the miss.

The checks-window reason ("its stored description does not match its geometry") and the `Limb` / rung-3 payload texts ("measured …", "leaves that surface between the samples") overclaim the same way.

## Repair shape (converged)

- **Split the variant.** `Unsized { Defect, Fit, Bound }`, where `Fit` is today's `LastResort`, renamed for the fact it encodes. `Bound` ends in `KERNEL_LIMIT_RECOURSE` on every arm at every reading; an unreadable margin keeps the defect or note rule (PR 4475).
- **Door.** `MissReading::Bound(margin)`. A bound whose far end is ≤ ε_in gets a sentence naming the bound: "the certificate's bound on this miss lies within the file's declared coincidence distance ε_in … set the tolerance to ε_in …". This applies on both the in-band and the definite route. A bound past ε_in gets the last resort, never "may lie within" and never the defect ending.
- **Rows.**
  - `TangentHull` and `PlaneNurbsHull` take `Residual(Unsized::Bound)`.
  - The AnalyticRung3 hull limb gets a check of its own, e.g. "the carrier's certified distance bound from the {surface}". It currently borrows "the plane × NURBS sup-norm bound".
  - `Surface1/2Residual`, `PlaneNurbsOnLocus` and topo's `PLANAR_BOUNDARY` stay `Fit`.
- **Readers.** Reword the checks-window reason and the payload texts for the bound kind. Per fork-log row 9, the at-rest text stays with its reader, matched exhaustively on `SsiLimb`.
- **Gate.** No new `MarginDiag` mint, door call or sentence site. Gate counts unchanged.
- **Not taken:** a valued last resort ("loosen to {bound} m or more"). Both designers were only weakly for it, and one would keep it out of the import door.

The foot-orthogonality limb is the `[ev]` PR 4498 question. Leave it as it is in this unit.

## Re-baselines expected

- `m7_8_plane_nurbs_edge::the_door_refuses_a_displaced_carrier_with_the_measured_bound`;
- `analytic_rung3_certificate` (`in_file_words`);
- the `Miss(Unsized::LastResort)` rows in certify.rs's ending-table test.

`review_probes_m7_3::probe_refit_seam_refuses_typed` is a limb-1 measured miss and must not move.
