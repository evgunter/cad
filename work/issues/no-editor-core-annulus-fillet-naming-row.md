---
id: no-editor-core-annulus-fillet-naming-row
kind: issue
title: No editor-core row drives an annulus fillet's naming records through emit_fillet
status: open
opened: 2026-08-30
github: 1294
refs: [935, 1268]
---

## From GitHub issue 1294

opened 2026-08-30, 0 comments.

**Raised by the BLEND-2 (#935) review (R2 MINOR-2), filed from the fix pass.**

## The gap

`editor-core`'s `names::emit_fillet` consumes every rim-phase channel of `FilletNaming` — `bands`, `rim_trims`, `rim_feet`, `meridian_splits`, `meridian_remnants`, `slits` — but no editor-core test evaluates a document whose `Node::Fillet` carves an ANNULUS rim, so the whole rim-phase emission path (BandFace/BandTrim/BandFoot/BandCross/BandCut/BandSlit role segments) is exercised only by the box/die's open-chain + ladder channels. In particular the #935 shared-wall path — where a later band's re-split of a recorded meridian remnant RETIRES the earlier row and re-covers both pieces — is held to the records-are-a-partition contract in `sweep` (`blend_tworims::a_shared_wall_carve_records_every_birth_and_every_death_once`, plus `blend2_r2_probes`' cap/cycle rows), and the emitter's `put` + `check_total` would refuse a violation, but nothing actually drives those records through `name_fillet` end to end.

## What would close this

One corpus-style document: a revolve (the `test_support::sphere_zone` profile is the natural body) with a fillet node naming two rims sharing a wall, evaluated green, with the emitted name table asserted total over the result — the annulus twin of `m5_pr12_fillet_node.rs`. Judged not cheap inside the BLEND-2 fix pass (a new revolve corpus document plus name-table assertions), so filed rather than rushed; the citation lives at the partition helper in `crates/sweep/tests/blend_tworims.rs`.

Filed by the BLEND-2 lane during the #1268 fix pass.

## Home

`work/issues/` — the row it owes is BLEND-2 residue over `editor-core`'s `names::emit_fillet` and a `sweep` annulus carve, both S-BLEND-era ground, and S-BLEND is closed.
