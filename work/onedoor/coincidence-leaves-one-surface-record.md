---
id: coincidence-leaves-one-surface-record
kind: unit
title: Every act that decides two surface records are one carrier leaves the faces on one record ([coincidence.door], PR 6427): the boolean's glue, the split's ON face, the chord join's tangent plane, a sweep's coplanar caps
status: open
opened: 2026-11-03
priority: P1
cost: H
---


Ratified on PR 6427 (2026-11-03). `docs/DESIGN.md` `[coincidence.door]`: "A verdict that two surface records are one carrier ends in one surface record: the act that decided it leaves every face on the other record re-pointed onto the kept one, and the row records why. Two distinct records at rest are two surfaces." The matching `[one-way.places]` row: "Whether two surface records are one surface | the act that decided their coincidence".

A read-only check of the tree on 2026-11-03 found the producers that leave two equal records at rest today:
- the boolean's glue of a curved group whose merge is skipped (`SkippedMerge`), which keeps `GluedSurfaces`/`GluedSurfaceEq`;
- the split's flush or coplanar section: `Rechart::new` mints a fresh plane while the operand face keeps its key;
- the chord join's tangent section, which mints its own `plane_key`;
- a half revolve's coplanar caps (`sweep/tests/revolve_partial.rs`);
- possibly import adoption.

Readers that re-read values to call such records one: `rest_locus` (via `carrier_eq`), `faces_continue`'s rungs, `carrier_eq_reading` in `recl.rs`/`vtxfac.rs`, and `vtxfac.rs`'s `one_carrier`. Pinned tests: `a_ruling_lying_on_a_wall.rs` `across_a_wall` and `revolve_partial.rs`.

The designers' shape:
- **One primitive**, `Body::unify_surfaces(kept, other, charts_agree)` in `attach.rs`. It re-charts every face on `other` onto `kept` through `Rechart::shared`, with the sense flipped where the charts oppose. It restates the stranded chart images in `kept`'s chart and drops `other`.
- **Each deciding act calls it**, keeping its `Coincidence` row:
  - the boolean before the merge;
  - the split, which states its ON section face as `FaceSurface::Shared` on the operand's key;
  - the chord join;
  - a sweep with coplanar caps (recording its row if the verdict is from values).
- **What retires with it:** `GluedSurfaces`/`GluedSurfaceEq` and the merge's glued rung. `rest_locus`'s value read goes once SELECT's form reading lands; the merge's value rungs are their own follow-up.
- **To measure first:** the per-face cost of re-charting a curved skipped group, and whether any serialised body (`step-export`, `persist`) relies on two keys.

Review tier: dual. It changes what a coincidence leaves in the arena, for every op that decides one.
