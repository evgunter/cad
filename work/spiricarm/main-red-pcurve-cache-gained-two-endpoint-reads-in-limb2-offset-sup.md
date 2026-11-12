---
id: main-red-pcurve-cache-gained-two-endpoint-reads-in-limb2-offset-sup
kind: issue
title: main red: pcurve_cache.rs gained two endpoint reads in limb2_offset_sup (PR 6521), and the certified-endpoint census still counts 2
status: open
opened: 2026-11-10
priority: P1
cost: E
---

**What fails.** `geom-core` `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` (`crates/geom-core/tests/certified_endpoint_census.rs:968`) is red on main `343823b7`, and is still red at `328f3671`. The only row that moved is `pcurve_cache.rs`: the census reads `("crates/geom-brep/src/pcurve_cache.rs", 4, 2)` and the roster at `certified_endpoint_census.rs:401` pins `(…, 2, 2, "")`.

**Cause.** PR 6521 (`spiricarm/spindle-partway`, commit `3de36b3e`, "apex grading reads its ends in limb2_offset_sup, through mid_param and powi") added `let mid = |x: T| geom_core::mid_param(x.lo(), x.hi());` at `crates/geom-brep/src/pcurve_cache.rs:3702`, inside `limb2_offset_sup` (`:3665`). That fn does not ask `is_certified()`, so both reads land in the remainder, and the roster row gained no note.

**Fix.** Judge the two reads at the site. A midpoint used as a frame or grading choice, with no enclosure resting on it, is the `patch_bound.rs` `centre` precedent. Either argue that at the site and give the roster row `(…, 4, 2, "<note>")`, or ask `is_certified()` before reading. Do not just raise the count: the census's message says this is not a number to silence.

Found by the ONEHOME stage-2 lane's full-suite run. It fails the same way on main without stage 2 (stage 2 adds `PcurveCache::carried`, which reads no endpoint).
