---
id: the-y-poled-ball-is-still-spelled-per-suite
kind: issue
title: The y-poled ball is still spelled per suite at about fifty sites beside sweep::test_support::ball_poled_y
status: open
opened: 2026-09-26
priority: P4
cost: D
---


## Finding

- **Where**: the half-disc lamina `(0, -r) bulge 1, (0, r) bulge 0`
  revolved about the sketch y-axis, written out per suite — the hit
  list below, five crates' suites, `mesh/src`, a `sweep` example and a
  `tools/` root.
- **Confidence**: sure about the hits; each one's disposition is the
  taker's (see below).
- **Raised by**: the `dup/sweep-topo-drain` lane, 2026-09-26, closing
  `the-interval-ball-fixture-is-homed-in-a-row-that-never-varies-it`.

That PR gave the ball one door, `sweep::test_support::ball_poled_y(r,
c, tol)` — generic in the scalar, the shape of `ball_poled_z_at`'s —
and folded the members that were the door's body
exactly: `m5_s12_curved_ops_interval`'s `ball(r)` and its recut
cutter, `review_arceval_r1_probes`' E1 ball, the eight private
`ball_at(r, c)` copies (`m5_pr12_battery`, `m5_pr12_die`,
`m5_s12_curved_ops`, `m5_s13_pips`, `m5_s13_pips_interval`,
`m5_s13_review_probes`, `m6_rider`, `m6_surgery`) and
`review_ring_clearance_r1_probes`' local `ball_poled_y(r, c)`. Every
fold there was measured bit-identical (`Debug` of the body) before it
landed, including the zero translation the origin callers now pass: at
r ∈ {1, 0.6, 0.16} and both scalars, `transform_rigid` by a zero
translation leaves the ball's `Debug` unchanged.

## The residue, at `4e671da75`

Instrument: `git grep` over every tracked file, no path argument, for
the lamina's first vertex `(0.0, -<r>), 1.0)` (also as `iv(1.0)`).
**Blind spots**: a lamina written from the north pole or offset along
the axis matches only when its first coordinate pair is `(0.0, <expr>)`
with a bulge of exactly `1.0`; a ball built by `profile::circle` or by
a sphere-kind door is a different construction and is not a member.

`crates/sweep/tests` (the S-TCOST/S-TINT ground this program announces
by seam): `blend6_verb_vocab` (~:274), `curved_mergedoor` (`ball(r)`,
~:538), `m5_pr9_boss_union` (~:347, ~:450, inline), `m5_pr9c_sphere_doors`
(`half_disc`, ~:48), `m5_s10_face_sense` (`ball_at(cy)`, ~:65 — a
different pose: the lamina slides along the axis), `m6_5_fillet_naming`
(~:34), `m6_chart_mints` (~:73), `m6_surgery_interval` (~:54),
`m9_d1_r1_probes` (~:26, ~:114, ~:141), `m9_d1_r2_probes` (~:49, an
offset lamina; ~:131; ~:173), `mass_props` (~:106), `mass_props_interval`
(~:151), `pcurve_p1b_r2_probes` (~:513), `r1_probes_issue1362_donut`
(~:297), `review_chamfer_r1_probes` (~:306), `review_d2_adv_probes`
(~:92), `review_m2_pr5` (~:466, ~:669), `review_m6_surgery_probes`
(~:38), `review_must_carry_rule_r1_probes` (~:229, north-pole first),
`review_pr12_probes` (~:33), `review_ring_clearance_r2_probes` (~:76),
`review_verbs_rim_lever_probes` (~:383, ~:584), `revolve_ball` (~:29),
`revolve_determinism` (~:21), `sf2b_r1_probes` (~:186, offset),
`tcost_k3_certificate` (`ball()`, ~:188), `trim_3_chart_bound_bodies`
(~:806, a raw vertex list), `verbs_cylsph_opening` (~:60),
`verbs_rim_closed_lever` (~:133), `verbs_rim_r1_probes` (~:321),
`verbs_sphsph_chart` (~:53), `verbs_sphsph_opening` (~:37).

Outside `sweep`: `crates/mesh/tests/common/mod.rs` (~:129, ~:177) and
`crates/stl/tests/common/mod.rs` (~:228) and
`crates/step-export/tests/common/mod.rs` (~:94, ~:540) are
`work/tint/tests-common-body-fixtures-triplicated.md`'s and stay with
it; `crates/mesh/tests` (`issue1362_band_placement` ~:39,
`r1_probe_hash` ~:67, `r1_probes_issue1362` ~:317 and ~:396,
`review_m2_pr6_walk_shapes` ~:36); `crates/mesh/src/curved.rs` (~:1261,
~:1276, ~:1410 — in-src test fixtures, TESS's ground);
`crates/sweep/examples/p1b_r2_m2.rs` (~:41);
`tools/tess-meter/tests/mesh5_probe.rs` (~:188).

## What a taker owes

Most sweep members are the door with `c` at the origin, and fold the
way this PR's did: measure bit-identity at the radius each one uses,
then call the door. Three kinds need a decision first rather than a
fold: a lamina slid along the axis (`m5_s10_face_sense::ball_at(cy)`,
`m9_d1_r2_probes` ~:49, `sf2b_r1_probes`) is a different pose from a
translated ball, and whether it becomes one is the verdict question
the conic-corpus cylinder's row settled by giving the door both knobs;
a reviewer probe's copy may be deliberately independent of the
library (the argument
`work/fixture/the-two-vertex-bulge-one-circle-fixture-has-eight-copies.md`
makes), and says so at the copy if it stays; and of the members
outside `sweep`, the `mesh`, `stl` and `step-export` suites and
`tools/tess-meter` already reach `sweep::test_support` through a
dev-dependency, while `mesh/src`'s in-src fixtures cannot (`mesh` sits
below `sweep`), so those stay or move with a manifest decision.

**Fix pass (2026-09-26, PR #3284).** Not every member contains the lamina: a copy that
builds on a ball door and then rotates it (the `ball_poled(r, c,
pole)` pair in `m5_pr12_die` and `m6_surgery`) is invisible to this
row's grep. That pair is folded (`sweep::test_support::ball_poled`);
a taker re-censuses with a second instrument aimed at
`rotation_about_axis` applied to a ball before trusting the list above.
