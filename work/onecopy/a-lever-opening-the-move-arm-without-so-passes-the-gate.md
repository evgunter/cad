---
id: a-lever-opening-the-move-arm-without-so-passes-the-gate
kind: issue
title: Four production levers open "move the geometry" with a continuation other than "so", and the gate passes a fifth
status: open
opened: 2026-11-10
priority: P3
cost: C
---


Left by PR 6753 (`work/onecopy/move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites.md`), at the orchestrator's direction.

`scripts/gates/recourse-ending-by-hand.sh` refuses `"move the geometry so` opening a production literal, since `geom_core::move_lever!("so …")` is its home. A lever that opens the arm with any other continuation passes: `move_lever!`'s compile-time check takes only `so `. Four production levers do that today:

| site | lever opens |
|---|---|
| `crates/geom-brep/src/certify.rs:1607` | "move the geometry to a scale where the spline face's control points …" |
| `crates/geom-brep/src/ssi.rs:2632` | "move the geometry, or the searched region, until each traced branch's tube …" |
| `crates/geom-brep/src/ssi/exhaust.rs:470` | "move the geometry nearer the origin, within the model's size range, …" |
| `crates/profile/src/fillet_recourse.rs:193` | "move the geometry nearer the origin, keep the radius no larger than …" |

A fifth would pass as silently.

**What is owed.**
1. Make the gate fire on any production literal opening `"move the geometry` at a word boundary, not only before `so`.
2. Hold the four as a counted, recorded exception list in the gate, each with its reason, in the shape `scripts/gates/viewer-module-kinds.sh` keeps its hand-kept lists. The list's count is checked against the hits, so a fifth goes red and a site that goes away takes its line.
3. Decide whether the two "nearer the origin" levers (`fillet_recourse.rs:193`, `ssi/exhaust.rs:470`) want one home beside `geom_core::RANGE_RECOURSE` ("scale the geometry into the session's range"). Both name the model's range, not a coincidence's move, so they may be the range arm's and not the move arm's.
