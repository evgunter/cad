---
id: main-red-the-m7-8-plain-names-probe-fails-after-the-faceview-fix-pass
kind: issue
title: main red: cert_m3r1_probes::m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else fails on main 7d3f0b73
status: dispatched
opened: 2026-11-04
priority: P1
cost: E
branch: onecopy/m7-8-plain-names
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
