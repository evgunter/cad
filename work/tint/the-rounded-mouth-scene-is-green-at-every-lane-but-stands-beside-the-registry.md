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

## The ball now meets the bar too (GREEN `green/nightly-1115`, PR 6985, 2026-11-15)

"Its sibling `with_a_ball` still escalates at 1e-12" no longer holds on
main `3e7a8c93b`: the ball's `Interval` union builds at ε 1e-12 and
matches the `f64` body (`matches_f64`), in a build at the nightly's
`opt-level = 2` and at the default dev profile alike, as it does at 1e-6
and 1e-9. The coaxial row asserts it for both scenes at every ε. So the
question this row asks covers both `shaft_and_bearing` scenes.

**Verified, and the cause is not named.** The build is right on its
merits: PR 6985's review checked the ball's `Interval` volume against
the closed form (5.836839596566823 lies inside it; faces, edges and
vertices 13/23/16 at all three ε), and
`coincidence_door::the_balls_interval_union_encloses_its_closed_form_under_a_bounded_pad`
holds it. Which merge made it build is not known. At `opt-level = 2` on
one box the ball still escalates at `33828f00ed` (PR 6967's merge) and
at `967cca218`, the first-parent commit before PR 6963, and builds on
main; the candidates are PRs 6963, 6974, 6958 and 6953.
