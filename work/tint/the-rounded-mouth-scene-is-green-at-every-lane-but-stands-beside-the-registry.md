---
id: the-rounded-mouth-scene-is-green-at-every-lane-but-stands-beside-the-registry
kind: issue
title: shaft_and_bearing::rounded_mouth is green at every lane and every ε, yet stands beside the corpus registry, whose bar it now meets
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/eps-1e-12`, 2026-11-14).

`crates/editor-core/tests/corpus/mod.rs` `stage6_beside_the_registry`
holds the INTENT stage-6 scenes that are short of registry membership,
"which requires every node green at every lane". On main,
`shaft_and_bearing::rounded_mouth` is green at `f64`, and its union
certifies at `Interval` and matches the `f64` body at every ε (default,
1e-6 and 1e-12). The ε 1e-12 case was mis-stated as an escalation from
PR 6795, where it was written, and is re-blessed in GREEN's PR 6967.
`coincidence_door::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`
asserts it per scene. So by the registry's own bar this scene could join
[`documents`].

Decide whether it moves into the registry, which runs it through the
registry battery, or stays beside it for a reason the module doc then
states. Its sibling `with_a_ball` still escalates at 1e-12 and stays
beside.
