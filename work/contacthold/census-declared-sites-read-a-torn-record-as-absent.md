---
id: census-declared-sites-read-a-torn-record-as-absent
kind: issue
title: The census's declared-site walks (ee_cross_backed, sweep_conformal_patches, the patch arm of confirm_curve_and_patch_records) read a torn record as absent — the held half of census-arena-walks-read-a-torn-record-as-absent
status: parked
opened: 2026-10-08
priority: P3
cost: E
blocked_on: [booleans-glue-on-zero]
---


Split off `census-arena-walks-read-a-torn-record-as-absent` at the CONTACT close-out (2026-10-08). That row's own HOLD paragraph names these three declared-site walks (`census.rs`: `ee_cross_backed`, `sweep_conformal_patches`, the patch arm of `confirm_curve_and_patch_records`). They are re-backed when booleans glue on Zero. The non-declared walks (`edge_is_line`, `face_cycles`, `snapshot`, the backstop) stay workable on the parent row.
