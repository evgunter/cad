---
id: census-declared-sites-read-a-torn-record-as-absent
kind: issue
title: The census's declared-site walks (ee_cross_backed, sweep_conformal_patches, the patch arm of confirm_curve_and_patch_records) read a torn record as absent — the held half of census-arena-walks-read-a-torn-record-as-absent
status: closed
opened: 2026-10-08
priority: P3
cost: E
pr: 4559
closed: 2026-10-10
---


Split off `census-arena-walks-read-a-torn-record-as-absent` at the CONTACT close-out (2026-10-08). That row's own HOLD paragraph names these three declared-site walks (`census.rs`: `ee_cross_backed`, `sweep_conformal_patches`, the patch arm of `confirm_curve_and_patch_records`). They are re-backed when booleans glue on Zero. The non-declared walks (`edge_is_line`, `face_cycles`, `snapshot`, the backstop) stay workable on the parent row.

## Re-parked on B2 (2026-10-10)

Its code survives E and F: these walks read the op's `ContactRecords`,
not user declarations, and E changes census.rs only in tests. But B2
(`contact-records-cite-their-decision`, `intent/s4-b2-records-cite`)
rewrites `census::confirm_curve_and_patch_records`, one of the three
sites, so the fix waits for B2's merge rather than colliding with it.
(CONTACTHOLD orchestrator)

## Done (2026-10-10, CONTACTHOLD)

- `ee_cross_backed`: the pair's faces are read with `proven` (each is in
  the snapshot, so it was read off the live arena) and their surfaces
  with `face_surface_linked`. Witness:
  `census::tests::the_crossing_rung_panics_on_a_torn_declared_surface`
  (the cap's surface torn after the snapshot; the old `continue` read
  the pair as absent and answered `Unanswered`).
- `sweep_conformal_patches`: both faces read with `proven`. No witness
  of its own can reach it: `ChartGroups::within` resolves the same keys
  first and already panics on a torn face.
- The patch arm of `confirm_curve_and_patch_records`: nothing to
  convert. Its only reads are the record's own face keys, which are the
  op's keys, and a miss there is the typed `StaleContactDeclaration`,
  which stays. The curve arm's `body.curves.get(e.curve)` hop is the
  parent row's (`census-arena-walks-read-a-torn-record-as-absent`).

## Closed (2026-10-10, PR 4559)

Merged after a full single review and a verified fix pass.
