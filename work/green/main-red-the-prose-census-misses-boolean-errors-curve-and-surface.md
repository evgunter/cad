---
id: main-red-the-prose-census-misses-boolean-errors-curve-and-surface
kind: issue
title: Main red: pncad-py's prose census meets BooleanError's curve and surface fields (topo/src/boolean/mod.rs) and neither decides nor names them
status: closed
opened: 2026-11-23
priority: P0
cost: E
closed: 2026-11-23
pr: 7670
---


**What fails.** `pncad-py prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason` (a slow-set test) panics at `crates/pncad-py/src/prose_census.rs:2422`, the "the blind spot is a CENSUS, not a silence" assertion. The census meets two sites it can neither decide nor find named in its undecided list: `BooleanError`'s `curve` and `surface` fields in `crates/topo/src/boolean/mod.rs`. Seen on PR 7599's run 45287945444, `test (app)`'s slow-set step, at test merge `ccc754708d`. It ran there only because PR 7599 edited `crates/pncad-py/Cargo.toml`, which seeds pncad-py. That diff touched no Rust, so the red is the tree's and not the PR's. The nightly runs the slow set, so it should red there too. It has not been reproduced on main alone.

**Suspects, unconfirmed.** The heldrecords/elbow commits to `crates/topo/src/boolean/mod.rs` on 2026-11-22 and 23: `787b232c20`, `c853a38051`, `835cb357c8`, `ddceb48397`. No bisect was run.

**The same shape before.** BIND's `work/bind/main-red-prose-census-misses-two-boolean-error-fields.md` (closed 2026-10-27) was this census missing two other `BooleanError` fields, `edge` and `face`. Its fix shows the two ways out: decide the site, or give it a line, with its reason, in the census's undecided list.

**Owed.** Whoever lands it: bisect to the commit that added the fields (or the arm that reads them), then decide the sites or name them with a reason. The disposition is BIND's ground (the census), and the cause is in topo, whichever program owns the commit.

## 2026-11-23 — named, with their reason (PR 7670)

The two sites are one arm of `BooleanError`'s `Display`: `JunctionEndUnread { gap: JunctionEndGap::NoLane { curve, surface }, .. }`, whose sentence renders `{curve:?}` and `{surface:?}`. They came in with HELDRECORDS' `835cb357c8` (`heldrecords/elbow-ends`, PR 7463), found by `git log -S` on the arm rather than by a bisect. The census cannot type either binding: each is introduced by a pattern nested inside the field pattern it reads, the largest reason its `UNDECIDED` list already has (`CertifyError`'s `check` is the same shape). Each now has its own line in `UNDECIDED` in `crates/pncad-py/src/prose_census.rs`, with that reason. Both fields are `geom::CurveKind` and `geom::SurfaceKind`, unit enums, so their `Debug` is prose and the binding cannot panic on them. Reducing the list stays with `work/bind/prose-census-undecided-residue.md`.
