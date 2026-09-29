---
id: axis-source-lowered-bytes-carry-no-minter-namespace
kind: issue
title: AxisSource's opaque lowered base carries no minter tag, so a recipe encoding and an import encoding can collide
status: open
opened: 2026-09-29
priority: P2
cost: M
design: true
refs: [axis-per-component-source-beside-geom-source]
---


Found by the review of PR 3419 (NOTE 4); filed at adjudication.

`AxisSource`'s base is opaque bytes the recipe layer lowers
(`AxisSource::from_lowered`, `crates/topo/src/source.rs`). The module
doc motivates keeping axis rows beside the origin record because step
2's import adoption can give an axis an identity no recipe gave it (a
shared STEP axis-placement entity). But nothing tags who minted the
bytes, so a recipe's lowered axis and an importer's lowered entity id
can compare equal by accident, and token equality would then read two
unrelated axes as one. `docs/AXIS-DECLARATION-DESIGN.md` Q4 calls
namespace disjointness load-bearing. Decide the tag (an enum over
minters, or a minter prefix the lowering owns) before step 2 stamps
anything; until then only the recipe layer mints, so the collision is
latent.
