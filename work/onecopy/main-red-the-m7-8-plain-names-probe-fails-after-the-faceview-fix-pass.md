---
id: main-red-the-m7-8-plain-names-probe-fails-after-the-faceview-fix-pass
kind: issue
title: main red: cert_m3r1_probes::m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else fails on main 7d3f0b73
status: closed
opened: 2026-11-04
priority: P1
cost: E
closed: 2026-11-05
pr: 6535
---


Filed by the SPLICE orchestrator, from the `splice/spiric-on-cap-plane` lane's topo run (PR 6523's fix pass, 2026-11-04).

## What

`crates/topo/src/cert_m3r1_probes.rs`:
`m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else`
fails in `cargo nextest run -p topo --lib` at default ε. The lane reproduced it on a clean worktree of `origin/main` at `7d3f0b73`, so it is main's red, not the lane's.

The file was last changed on main by `e383ccfbcc` ("topo: FaceView fix pass — Body::face_surface and find_half_edge's walk retire onto the home, …", 2026-11-04). The FaceView handle is ONECOPY's (`a-face-is-read-through-a-resolved-handle`). Whether that commit or another one moved the probe is unmeasured. The probe pins what `export/no-public-door-reaches-a-body-carrying-the-m7-8-class` describes: on the corrupted m7_8 wall, the plain `validate_pseudomanifold` / `contact_marks` names report `EdgeCertification` edge by edge, and nothing else.

## What closing it takes

1. Reproduce the failure on main and read the panic: which edge or payload differs.
2. Bisect to the commit that moved it.
3. Either restore the verdict, or re-pin the probe with the reason the verdict legitimately moved, and update the export row's text to match.

## Resolution

The cause was PR 6498 (`pcert/spline-chart-incidence`), not the
FaceView fix pass. A bisect over main's first-parent merges found
#6490 (FaceView, carrying `e383ccfbcc`) and #6500 green, and #6498 red.
#6498 runs the spline chart's incidence read ahead of the general lane
in `topo::pcurves`' NURBS derivation. The corrupt wall's refusal
therefore ends as that read (`CarrierOffChart`, decided past the band)
where it used to be the general image's `ResidualExceeded { MapResidual }`.
The move is legitimate. PR 6535 re-pins the probe through
`geom_brep::IncidenceRead::of` and bounds the off margin by the bow's
height over the wall's edges.

