---
id: section-cert-classification-takes-no-body
kind: issue
title: "[topo.homes] arm 1: section_cert's classification takes no Body but reads a topo root reading; its certify reads arena keys"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep` (2026-11-03), which checked `boolean/section_cert.rs` item by item and moved none of it.

- **Takes an arena key, stays:** `section_cert::certify`, whose `place` closure answers `[Option<FaceContainment>; 2]`; `FaceContainment` (`boolean/contain.rs`) carries `EdgeKey` and `VertexKey`.
- **Takes no Body, cannot move yet:** the classification (`classify`, `classify_known`, `of_pair_section`, `of_carried`, `torus_plane_loops`, the closed-form pair arms `torus_pair` … `sphere_sphere`, and `Section`, `Component`, `Touch`, `Cleared`, `Refusal`, `Sheet`, `Side`) reads `crate::pcurves::chart_side_bounds` through `bounding`. `chart_side_bounds` (`crates/topo/src/pcurves.rs`) takes a surface, a chart side and a band, no `Body`, so arm 1 sends it to `geom-brep` first; then the classification and its rows (`section_cert/{ruling, cone_search, cone_pair_search, square_search, search_kit}.rs`, `section_cert_*_rows.rs`) can follow; its K names (`section_*`, `pair_section*`) carry no lane prefix, so the move renames none.
- `minted-reference-from-inputs` names `boolean/section_cert.rs` as a home; the line moves with the file.
