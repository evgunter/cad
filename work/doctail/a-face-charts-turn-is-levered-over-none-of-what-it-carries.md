---
id: a-face-charts-turn-is-levered-over-none-of-what-it-carries
kind: issue
title: A face's chart levers its least turn over its slides and the face's distance alone, so a wide profile on it builds where a placement of that size refuses backwards
status: open
opened: 2026-11-18
priority: P1
cost: M
---

Found by the FULL review of `a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face` (PR 7111, MINOR 2), and left open by its fix pass because the fix has more than one home.

## What

`mate::value::face_chart` (`crates/editor-core/src/mate/value.rs`) levers the chart's least turn over `solve::arm_of(0.0, ..)`: no copy reach, so only its two slides and the face's plane's distance from the origin. A placement levers the same turn over `bundle_arm(copy_reach, ..)`, the copy's reach included (`eval::wire::fold_with`, `copy_reach`). But what a frame carries is not nothing: the profile drawn on it, and every sweep of it, stand in the frame (`ProfileValue::framed` and `lifted`, `crates/editor-core/src/eval/anchor.rs`), so the turn carries them out to their own reach.

Repro, pinned as `face_chart_and_signed_depth::near_the_pole_the_frame_charts_where_a_wide_placement_refuses`: a top cap tilted `1e-6` about `x`, the orientation `x: +x, z: −z` (backwards), a disc of radius 100 drawn on the frame. The chart is Ok and its `+0.5` boss builds; a cube of side 100 placed on the same face under the same orientation refuses `MateFault::Backwards`. DESIGN.md `[placement]` (Ev's ruling on `[ev]` #7073) says the frame "refuses, naming the orientation to write, as a placement does", so the ruling's text is not yet true.

## Owed

Whatever the frame carries is levered as a placement levers its copy: it refuses typed in the band when its reach over the frame's turn exceeds what the conditioning allows, with the same recourse (write the orientation), by the placement's own rule (`arm_of` over the carried reach, then `Chart::read`), not a second band computation. The turn's value does not depend on the arm (`RigidMotion::least_turn` reads it for the pole verdict and the conditioning alone), so this is a verdict at each carrier, never a different frame. Homes, to be chosen:

1. **The reader re-reads its frame at its own reach.** `face_chart` takes a reach; the profile node reads its frame (`eval::wire::trace::drawn`) at its loops' reach, and each sweep (`wire_swept` for extrude and revolve, `wire_loft`'s first section) re-reads the profile's frame at its body's reach. The pose read grows a reach argument, and the sweeps, which today hold only the profile's value, need the document and the frame's variable.
2. **The profile's value carries the chart's reads beside its frame**, and the one door where a swept body takes the frame (`ProfileValue::framed` / `lifted`, which every sweep goes through) re-decides `Chart::read` at the body's reach (`part_reach_from`); the profile node decides at its own reach. One door for every sweep, but it carries a representation only a check reads (DESIGN.md `[simple.checks]`), and `framed` becomes fallible.
3. **Lever the frame over the face's body's reach.** Local and cheap, but the repro's disc is fifty times the block it is drawn on, so the ruling's text stays untrue.
4. **Restate the ruling**: the frame's band is levered over the frame's own arm, and a wide profile on a frame near the pole is the author's to see. That changes ratified text, so it is a question for Ev.

Each re-decision at a carrier records its own `mate_chart_pole` funnel sample. When this lands, the pinned row above turns over (the frame, or its boss, refuses backwards) and is re-pinned with the outcome.
