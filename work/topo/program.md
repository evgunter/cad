---
id: topo
kind: program
title: TOPO — the Euler operators: the surgery doors that mutate a body, and how they leave it half-described
status: active
opened: 2026-09-04
area: kernel
prefix: topo/
tag: (TOPO orchestrator)
ab_band: 2700-2799
paths: [crates/topo/src/euler.rs, crates/topo/src/euler_ring.rs, crates/topo/src/euler_kill.rs, crates/topo/src/attach.rs, crates/topo/src/movefac.rs, crates/topo/src/revert.rs, crates/topo/src/merge_faces.rs, crates/topo/src/fixtures.rs]
keep_out: [the 2026-09-20 priority-seam cut moved validate.rs and tier3_tests.rs to ATREST, split.rs and query.rs to TQUERY, provenance.rs source.rs source_walk.rs readback.rs live.rs to ORIGIN, seqgen.rs seqgen/* review_d18.rs review_d18_probes.rs to PROBE, and WALKS claims no paths at all - a TOPO unit reaching any of them announces the seam in its PR, the paths list is ENUMERATED rather than globbed on purpose - crates/topo/src/* would double-claim ground five programs already hold and territory is blind to a double claim (work/meta/territory-cannot-see-a-path-two-programs-both-claim), so this program names its files and nothing else, topo/src/boolean/* and splitting/* and census.rs and chord_join.rs and chart_region.rs and face_normal.rs are S-BOOL's and CURVED's (code-quality Track Q's fence), topo/src/query.rs and flush.rs were SEAT's until SEAT left the tracker on 2026-09-06 (docs/DOC-LEDGER.md sweep 8) - they are unowned now and this program edits them as its own ground where a readback or query door is the unit (edge-carrier-kind-has-no-readback-door), announcing nothing to a closed board, topo/src/coherence.rs is S-MESH's, topo/src/shell.rs and replace_face.rs and transform.rs and offset_together.rs are SHELL's, topo/src/pcurves.rs is TRIM's, topo/src/props.rs is code-quality Track M's (S-CERT's), crates/topo/tests/* is S-TCOST's glob (crates/*/tests/*) — this program's units add rows there as ordinary tests and say so in the PR rather than drawing a second fence, crates/test-utils/ is code-quality Track W's and S-TCOST's - D261 converts topo's own readers and its own census entries only and re-derives the shared UNCONVERTED_TODAY ceiling from the table at landing rather than lowering it by its own member count, the remaining ~25 crates/topo/src files (body.rs entity.rs geometry.rs instance.rs null.rs contact.rs separation.rs iso.rs chart.rs chart_iso.rs ray_parity.rs sector_face.rs sector_shape.rs offset_axial.rs and the review_m1_* readers) are UNOWNED AND NOT FINISHED in the sense the code-quality plan's geom-brep seam gives that phrase - a row landing on one draws the fence in the PR that mints it, and this program does not edit there until it has]
priority: P0
---

The Euler operators proper, after the 2026-09-20 cut: the surgery doors
that MUTATE a body, and the ways they leave it half-described. `mev`,
`mef` and `mekr` mint half-edges into a cached curved face with no
pcurve row beside them, and `kev`'s fan merge moves one end of a null
edge onto a distinct point unchecked. Each is a live corruption of the
arena, which is why the track stayed P0 when its other halves left.
(The run doors' silent chart change closed with PR 2603; the fan
`mev`'s null-edge hole closed with PR 3148.)

TOPO was 132 budget points in one directory on 2026-09-20 — about four
and a half sittings. The cut (Ev, in chat, the same day) divided it on
its PRIORITY seam per `work/README.md` "Track size", into five
successors plus this remainder: ORIGIN (the identity channel), ATREST
(the at-rest validator), TQUERY (the split and query doors), WALKS
(one relation spelled n times) and PROBE (the guards that cannot go
red). Each row MOVED by `git mv` with its id, body and history
unchanged. TOPO keeps its band 2700-2799.

Charter and the unit order: `work/topo/plan.md`; narrative in
`work/topo/log.md`.
