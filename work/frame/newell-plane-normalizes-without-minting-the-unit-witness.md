---
id: newell-plane-normalizes-without-minting-the-unit-witness
kind: issue
title: newell_plane normalizes its cross-sum through the bare Vec3::normalize instead of minting a UnitVec3 under the band it holds
status: open
opened: 2026-11-03
priority: P4
cost: E
---


## What

`crates/geom-brep/src/newell.rs` `newell_plane` builds its normal as `normal_sum.normalize()`, the bare door, and decides no length first. The band is in hand at the site. `geom_core::UnitVec3::new(sum, "newell_plane_normal", band)` is the witnessed door: it would refuse TYPED on a sum whose length is not decided positive, instead of handing an `Invalid` normal downstream.

On main `0f01217666` this is no longer reached as a refusal. FRAME's re-measure (`a-widened-derived-placement-normalises-a-straddling-newell-sum`, closed) found `|N|²` decided positive at every one of about 460 calls. So this is cleanup, not a live wrong answer.

That row's 2026-09-15 section names this site and `step-import`'s `recognize` (exch's `recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`) as the last two callers of the bare door. When both mint the witness, the bare `Vec3::normalize` can retire, or be narrowed to scratch.
