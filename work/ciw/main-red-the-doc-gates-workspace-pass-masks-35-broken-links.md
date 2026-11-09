---
id: main-red-the-doc-gates-workspace-pass-masks-35-broken-links
kind: issue
title: main red, masked: the doc gate's workspace pass stops at its first failing crate, so 35 more rustdoc errors in geom-brep, topo and viewer never print
status: open
opened: 2026-11-09
priority: P0
---


`scripts/doc-gate.sh`'s `doc_pass_with` runs `cargo doc --no-deps --document-private-items --workspace --all-features` without `--keep-going`. Once one crate fails, cargo stops scheduling new rustdoc jobs, so the nightly (run 42567046151, job `rustdoc (gate, every root)`) printed errors only for the crates that happened to be running. The same pass with `--keep-going`, at `origin/main` `a2a5b8e384` (after BOXES' `boxes/nightly-reds` fixes), fails `geom-core`, `geom-brep`, `topo`, `step-import`, `viewer` and `editor-core`. It reports 35 errors that the nightly log never shows:

- `crates/geom-brep/src/dihedral.rs:1130`: unresolved link to `WedgeWalk::along_edge`
- `crates/geom-brep/src/dihedral.rs:460`: unresolved link to `WedgeWalk`
- `crates/geom-brep/src/dihedral.rs:521`: unresolved link to `WedgeWalk`
- `crates/geom-brep/src/edge_nurbs.rs:559`: unresolved link to `CertCheck::recourse`
- `crates/geom-brep/src/intersect.rs:733`: `crate::pair_section` is both a function and a module
- `crates/geom-brep/src/pcurve_cache.rs:11678`: unresolved link to `incidence_over_span`
- `crates/geom-brep/src/pcurve_cache.rs:5699`: unresolved link to `periodic_envelope`
- `crates/geom-brep/src/props/quad/cells.rs:315`: unresolved link to `topology`
- `crates/geom-brep/src/props/quad/rational.rs:36`: unresolved link to `grid_vec`
- `crates/geom-brep/src/props/quad/trimmed.rs:793`: unresolved link to `knot_aligned_cuts`
- `crates/geom-brep/src/recourse.rs:954`: unresolved link to `crate::Surface::is_placeholder_chart`
- `crates/geom-brep/src/ssi.rs:3199`: unresolved link to `ruling_coverage`
- `crates/topo/src/boolean/holdings.rs:22`: `super::overlap` is both a function and a module
- `crates/topo/src/boolean/nary.rs:17`: `super::overlap` is both a function and a module
- `crates/topo/src/boolean/reduce.rs:590`: `crate::finish` is both a function and a module
- `crates/topo/src/boolean/region.rs:242`: unresolved link to `span_plane_meet`
- `crates/topo/src/boolean/sectors.rs:1547`: unresolved link to `side_code`
- `crates/topo/src/boolean/sectors.rs:1692`: unresolved link to `side_code`
- `crates/topo/src/boolean/sectors.rs:39`: unresolved link to `side_code`
- `crates/topo/src/census/curved.rs:280`: unresolved link to `TouchUnread::InBand`
- `crates/topo/src/census/curved.rs:281`: unresolved link to `TouchUnread::EdgeTangent`
- `crates/topo/src/census/curved.rs:284`: unresolved link to `TouchUnread::Corrupt`
- `crates/topo/src/link.rs:151`: unresolved link to `Departure::Parted`
- `crates/topo/src/loop_winding.rs:151`: unresolved link to `Conic::speed_hi`
- `crates/topo/src/offset_axial.rs:52`: `crate::finish` is both a function and a module
- `crates/topo/src/shell.rs:1552`: `crate::finish` is both a function and a module
- `crates/topo/src/splitting/insert.rs:313`: unresolved link to `ApexSectionReading`
- `crates/topo/src/splitting/neighborhood.rs:217`: unresolved link to `crate::bound_side::read_side`
- `crates/topo/src/validate.rs:248`: `crate::finish` is both a function and a module
- `crates/viewer/src/app/drive.rs:53`: unresolved link to `settle`
- `crates/viewer/src/drafts.rs:123`: unresolved link to `Self::sync_trace_shown`
- `crates/viewer/src/pane/features.rs:95`: unresolved link to `crate::display::DisplayView::body_on`
- `crates/viewer/src/pane/view.rs:3`: unresolved link to `crate::display::toggles`
- `crates/viewer/src/session.rs:738`: unresolved link to `DocSession::certified`
- `crates/viewer/src/test_support.rs:317`: unresolved link to `crate::session::valued`

Two pieces of work:
1. The gate: pass `--keep-going` (cargo 1.74+) in `doc_pass_with`, so that one red crate does not hide the rest.
2. The sites: each is its own owner's. `python3 scripts/work.py territory --files -` names them; route each one or fix it.

None is BOXES': BOXES' cover-build sites in `topo` (`boolean/boxes.rs`, `cover.rs`, `shell.rs`) and `sweep` are fixed on `boxes/nightly-reds`.

(BOXES nightly-reds triage, 2026-11-09)
