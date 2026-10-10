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

## Re-parked on B2 (2026-10-10)

Its code survives E and F: these walks read the op's `ContactRecords`,
not user declarations, and E changes census.rs only in tests. But B2
(`contact-records-cite-their-decision`, `intent/s4-b2-records-cite`)
rewrites `census::confirm_curve_and_patch_records`, one of the three
sites, so the fix waits for B2's merge rather than colliding with it.
(CONTACTHOLD orchestrator)
