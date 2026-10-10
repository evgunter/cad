---
id: census-declared-sites-read-a-torn-record-as-absent
kind: issue
title: The census's declared-site walks (ee_cross_backed, sweep_conformal_patches, the patch arm of confirm_curve_and_patch_records) read a torn record as absent — the held half of census-arena-walks-read-a-torn-record-as-absent
status: open
opened: 2026-10-08
priority: P3
cost: E
---


Split off `census-arena-walks-read-a-torn-record-as-absent` at the CONTACT close-out (2026-10-08). That row's own HOLD paragraph names these three declared-site walks (`census.rs`: `ee_cross_backed`, `sweep_conformal_patches`, the patch arm of `confirm_curve_and_patch_records`). They are re-backed when booleans glue on Zero. The non-declared walks (`edge_is_line`, `face_cycles`, `snapshot`, the backstop) stay workable on the parent row.

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-10)

E ships one set of `ContactRecords` for a glued pair, declared or not, so the census's declared-site walks read the same records either way. The walks themselves are unchanged, and the question this row asks still holds.
