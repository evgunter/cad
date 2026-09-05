---
id: topo
kind: program
title: TOPO — the topology core: Euler surgery, validation and the identity channel
status: open
opened: 2026-09-04
area: kernel
prefix: topo/
tag: (TOPO orchestrator)
ab_band: 2700-2799
paths: [crates/topo/src/euler.rs, crates/topo/src/euler_ring.rs, crates/topo/src/euler_kill.rs, crates/topo/src/split.rs, crates/topo/src/attach.rs, crates/topo/src/movefac.rs, crates/topo/src/revert.rs, crates/topo/src/live.rs, crates/topo/src/merge_faces.rs, crates/topo/src/seqgen.rs, crates/topo/src/seqgen/*, crates/topo/src/validate.rs, crates/topo/src/tier3_tests.rs, crates/topo/src/review_d18.rs, crates/topo/src/review_d18_probes.rs, crates/topo/src/fixtures.rs, crates/topo/src/source_walk.rs, crates/topo/src/readback.rs, crates/topo/src/source.rs, crates/topo/src/provenance.rs]
keep_out: [the paths list is ENUMERATED rather than globbed on purpose - crates/topo/src/* would double-claim ground five programs already hold and territory is blind to a double claim (work/meta/territory-cannot-see-a-path-two-programs-both-claim), so this program names its files and nothing else, topo/src/boolean/* and splitting/* and census.rs and chord_join.rs and chart_region.rs and face_normal.rs are S-BOOL's and CURVED's (code-quality Track Q's fence), topo/src/query.rs and flush.rs are SEAT's - face-kind-read-has-two-homes reaches query.rs for ONE door and is a seam to be announced on SEAT's board before any edit lands there, topo/src/coherence.rs is S-MESH's, topo/src/shell.rs and replace_face.rs and transform.rs and offset_together.rs are SHELL's, topo/src/pcurves.rs is TRIM's, topo/src/props.rs is code-quality Track M's (S-CERT's), crates/test-utils/ is code-quality Track W's and S-TCOST's - D261 converts topo's own readers and its own census entries only and re-derives the shared UNCONVERTED_TODAY ceiling from the table at landing rather than lowering it by its own member count, the remaining ~25 crates/topo/src files (body.rs entity.rs geometry.rs instance.rs null.rs contact.rs separation.rs iso.rs chart.rs chart_iso.rs ray_parity.rs sector_face.rs sector_shape.rs offset_axial.rs and the review_m1_* readers) are UNOWNED AND NOT FINISHED in the sense the code-quality plan's geom-brep seam gives that phrase - a row landing on one draws the fence in the PR that mints it, and this program does not edit there until it has]
---

The topology core, unowned since M6 closed: the Euler operators and
their corruption tables, tier-3 validation, the `Live` unforgeability
guard, the sequence generator, and the D5/N6 identity channel
(`provenance.rs`, `source.rs`, `readback.rs`). Claims code-quality
**Track P whole** — its fourteen rows are this territory and no lane
has ever run on them. Class E and H mixed, with one D (the two-homes
face-kind read, a seam with SEAT). Charter, the fence and the unit
order: `work/topo/plan.md`; narrative in `work/topo/log.md`.
