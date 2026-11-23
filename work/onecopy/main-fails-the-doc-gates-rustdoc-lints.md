---
id: main-fails-the-doc-gates-rustdoc-lints
kind: issue
title: Main fails doc-gate.sh's rustdoc pass: 50 broken or ambiguous intra-doc links and redundant targets in 33 files
status: closed
opened: 2026-11-10
priority: P3
cost: C
closed: 2026-11-10
pr: 6708
---


Filed by QML T3 (PR 6701, `onecopy/refusal-standing`) on the orchestrator's ruling. 6701's review found two broken links in the moved ladder (fixed there). Running doc-gate's rustdoc pass to check that fix showed main failing the same pass in files 6701 does not touch.

## Reproduce

The workspace pass of `scripts/doc-gate.sh` (`RUSTDOC_LINTS`, its `doc_pass`), run alone:

```
RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
  cargo doc --no-deps --document-private-items --all-features --workspace --keep-going
```

At `edf1fb1413e` (main at `b2dc97d67a1` merged into 6701's branch), it fails to document geom-core, geom-brep, topo, step-import, editor-core and viewer. It reports 50 errors in 33 files, none on a line 6701 adds. The doc gate runs viewer at its default features rather than `--all-features`, so the viewer lines may not all fire there.

## The sites

Each is an unresolved link, an ambiguous one (an item that is both a function and a module: prefix `fn@` or `mod@`), or a redundant explicit target (`rustdoc::redundant_explicit_links`):

- `crates/editor-core/src/edit.rs`: :3107 `crate::take_over` is both a function and a module
- `crates/editor-core/src/eval/mod.rs`: :414 unresolved link to `measure::Observed`
- `crates/editor-core/src/names/borders.rs`: :460 unresolved link to `Obstacles::split`
- `crates/geom-brep/src/dihedral.rs`: :461 unresolved link to `WedgeWalk`; :522 unresolved link to `WedgeWalk`; :1131 unresolved link to `WedgeWalk::along_edge`
- `crates/geom-brep/src/edge_nurbs.rs`: :559 unresolved link to `CertCheck::recourse`
- `crates/geom-brep/src/intersect.rs`: :733 `crate::pair_section` is both a function and a module
- `crates/geom-brep/src/pcurve_cache.rs`: :5699 unresolved link to `periodic_envelope`; :11691 unresolved link to `incidence_over_span`
- `crates/geom-brep/src/props/quad/cells.rs`: :315 unresolved link to `topology`
- `crates/geom-brep/src/props/quad/patch.rs`: :120 redundant explicit link target
- `crates/geom-brep/src/props/quad/rational.rs`: :36 unresolved link to `grid_vec`
- `crates/geom-brep/src/props/quad/trimmed.rs`: :793 unresolved link to `knot_aligned_cuts`
- `crates/geom-brep/src/recourse.rs`: :954 unresolved link to `crate::Surface::is_placeholder_chart`
- `crates/geom-brep/src/ssi.rs`: :3201 unresolved link to `ruling_coverage`
- `crates/geom-core/src/predicate.rs`: :559 redundant explicit link target; :572 redundant explicit link target; :678 redundant explicit link target
- `crates/step-import/src/error.rs`: :398 unresolved link to `topology`
- `crates/step-import/src/knot_split.rs`: :1 unresolved link to `topology`
- `crates/step-import/src/lib.rs`: :413 unresolved link to `topology`
- `crates/topo/src/boolean/holdings.rs`: :22 `super::overlap` is both a function and a module
- `crates/topo/src/boolean/nary.rs`: :17 `super::overlap` is both a function and a module
- `crates/topo/src/boolean/ops.rs`: :5782 unresolved link to `ValidationError::VolumeUncomputable`
- `crates/topo/src/boolean/reduce.rs`: :590 `crate::finish` is both a function and a module
- `crates/topo/src/boolean/region.rs`: :268 unresolved link to `span_plane_meet`
- `crates/topo/src/boolean/sectors.rs`: :39 unresolved link to `side_code`; :1557 unresolved link to `side_code`; :1702 unresolved link to `side_code`
- `crates/topo/src/census/curved.rs`: :285 unresolved link to `TouchUnread::InBand`; :286 unresolved link to `TouchUnread::EdgeTangent`; :288 unresolved link to `TouchUnread::Corrupt`
- `crates/topo/src/link.rs`: :154 unresolved link to `Departure::Parted`
- `crates/topo/src/loop_winding.rs`: :151 unresolved link to `Conic::speed_hi`
- `crates/topo/src/offset_axial.rs`: :52 `crate::finish` is both a function and a module
- `crates/topo/src/shell.rs`: :1576 `crate::finish` is both a function and a module
- `crates/topo/src/splitting/containment.rs`: :2034 unresolved link to `RootPlace`; :2045 unresolved link to `crate::boolean::contain::contfp`
- `crates/topo/src/splitting/insert.rs`: :313 unresolved link to `ApexSectionReading`
- `crates/topo/src/splitting/neighborhood.rs`: :223 unresolved link to `crate::bound_side::read_side`
- `crates/topo/src/validate.rs`: :248 `crate::finish` is both a function and a module; :2067 unresolved link to `crate::surface_boundary::cycle_winding`
- `crates/viewer/src/app/drive.rs`: :53 unresolved link to `settle`
- `crates/viewer/src/camera.rs`: :1075 unresolved link to `geom::mid_param`
- `crates/viewer/src/drafts.rs`: :123 unresolved link to `Self::sync_trace_shown`
- `crates/viewer/src/pane/features.rs`: :95 unresolved link to `crate::display::DisplayView::body_on`
- `crates/viewer/src/pane/view.rs`: :3 unresolved link to `crate::display::toggles`
- `crates/viewer/src/session.rs`: :738 unresolved link to `DocSession::certified`
- `crates/viewer/src/test_support.rs`: :317 unresolved link to `crate::session::valued`

## Done when

The command above passes, and `scripts/doc-gate.sh` passes on main. A renamed or deleted target is re-pointed at what replaced it, or the link becomes plain code.

## Closed (2026-11-10, PR 6708)

Main at `f2d5c0639` had 51 sites in 35 files (one new since filing). All are re-pointed, the command above exits 0, and `scripts/doc-gate.sh` and its `--selftest` pass. `doc_pass_with` now passes `--keep-going`, which closes ciw's `main-red-the-doc-gates-workspace-pass-masks-35-broken-links`. For whoever owns CI cost: the workspace pass alone took 62 s wall on 4 cores with a fresh target, which is the price of a per-PR rustdoc step. Rustdoc stays nightly-only, by the latency cut's deliberate trade.
