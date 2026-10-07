---
id: mesh-slit-annulus-rows-no-longer-build-a-slit
kind: issue
title: mesh rows that witness a slit plane face build it with a full revolve, which no longer mints one
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [4136]
---


Since PR 4136 a full revolve mints each plane wall as one face carrying its bore as a ring, with no meridian slit. Several mesh rows build their "slit annulus" through a full revolve, so they now pass on an unslit annulus and no longer witness what their names and docs say: the projection keying that keeps a slit's two traversals one chart point, and the odd-multiplicity fill toggle (`crates/mesh/src/planar.rs` module doc, "Slit note").

- `crates/mesh/tests/issue555_subfloor_cap.rs` — `the_slit_annulus_cap_survives_the_written_zero` (and the module doc, "a plain slit annulus").
- `crates/mesh/tests/r2_mesh2_probes.rs` — `r2_slit_annulus_at_several_proportions` ("PROBE E — a slit annulus (full revolve …)").
- `crates/mesh/tests/review_m2_pr6_checkmesh_audit.rs` — `survives_concentric_slit_annuli` (and the module doc).
- `crates/mesh/tests/newell_probes.rs` — `probe_slit_washer_rebuild` ("the full-2pi revolve annulus wall is a slit").
- `crates/mesh/tests/genus.rs` — module doc ("slit annulus planes").

The `planar.rs` unit rows still pin the bit-keying on a constructed frame, so the mechanism is not unpinned; the end-to-end witnesses are. Recourse: re-slit the revolved annulus with one public `Body::mekr_chord` (`MekrSite::Cycles { target: <outer anchor>, ring: <bore anchor> }`), as `crates/sweep/tests/verbs_shell.rs`'s check-9 row does, or rename the rows to the unslit annulus they now mesh and keep the slit witness elsewhere.
