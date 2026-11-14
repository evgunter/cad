---
id: a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt
kind: issue
title: a ball turned a quarter turn refuses the curved lanes CDT insert at its pole - the chart frame an ulp off its axis (first met as a PlacedUnion master against exact copy frames)
status: dispatched
opened: 2026-10-20
priority: P0
cost: M
branch: tesslane/turned-ball
---

Found by INTENT stage 3 D's exact-turn lane (`work/intent/transform-retires-into-a-placement.md`), which measured it and then moved its own corpus off it; nothing in the suite reaches it now.

**What was measured.** At `ccf433d0e4` the corpus document `die_tool` (`crates/editor-core/tests/corpus/die_tool.rs`) placed its master ball with `place_against`, whose spins come from `atan2` and so leave the master's pole an ulp off the cube's `z` (a cosine of `6.1e-17` where `0` is meant). Its six `PlacedUnion` frames had just become exact (a quarter turn now has cosine `0` exactly), so the master and its copies disagree by that ulp. The viewer's display budget then refused it at a display tolerance of `1e-4`: `tessellate: the CDT rejected a point insertion on a face — a non-finite or out-of-range chart coordinate, i.e. corrupt geometry`, raised as `TessellateError::Triangulation { face: FaceKey(14v3) }`. The face is a sphere face of the subtracted cavity. The rows that showed it were `viewer::all display_budget::*` ("die_tool fits at 0.0001") and `index_memo::every_parametric_corpus_document_indexes_the_same_through_the_seam_across_edits`.

The same document with both the master and the frames inexact, or both exact (`5cf76d715e`, master on `corner_frame` plus an axis mate), tessellates. So the mesher refuses a sphere face whose chart is tilted by an ulp against the plane cutting it, though the geometry is valid.

**Where.** `crates/mesh/src/curved.rs:295-297` (the `insert` closure) and `:329-331` (`can_add_constraint` refused); the message cannot tell these apart, so the first step is to find which one fires. To reproduce, check out `die_tool.rs` at `ccf433d0e4` on top of the current tree.

**Done when** a sphere face whose pole is tilted by an ulp off its cutting plane's normal tessellates, or refuses with a typed reason that names that cause and not "corrupt geometry"; add that row.

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## A plain ball turned a quarter turn refuses (2026-11-14, TESSLANE) — re-banded P0

Met by the reach measurement at `d6e1eb1f`; pinned as `reach_probes::a_z_poled_ball_of_radius_root_two` on `tesslane/measure-reach`.

`sweep::test_support::ball_poled_z(√2, 0)` is a revolved ball turned π/2 about `x` through `topo::transform_rigid`, the doors a user has. It refuses `Triangulation { face }` at δ 2e-2 and 5e-3. The same ball at radius 1 and at 0.3 meshes.

**The site is settled**: the `insert` closure in `curved::tessellate_curved` (spade's `insert` errors), not `can_add_constraint`, on a pole entry with `u = −4.838e−49`, `v = ±π/2`. This is the question this row left open, answered for this witness.

The quarter turn leaves the chart frame an ulp off the axis it means (a cosine of `6.1e-17`), which is this row's cause; the witness shows it needs no `PlacedUnion` and no cutting plane, only a turned ball. It is also why tube ∪ ball in `valid-ringed-torus-faces-build-but-cannot-be-meshed` reports `Triangulation` rather than its ring.

**P0**: a whole ball that a user has rotated does not draw at an ordinary δ. Not yet measured: why radius √2 and not 1, and what `−4.838e−49` is a product of.

## Unit (2026-11-14, TESSLANE)

**Letter M. Tier: single review, FULL.** The change is at a chart pole in the curved lane's CDT, where watertightness and the pole fan are argued rather than visible: believing a fix takes more than reading it, so the review carries claims to falsify.

Branch `tesslane/turned-ball`, cut from `tesslane/measure-reach` so the probe `reach_probes::a_z_poled_ball_of_radius_root_two` is in hand.

- **Measure before building.** The site is known (spade's `insert` errors in `curved::tessellate_curved`, on a pole entry with `u = −4.838e−49`, `v = ±π/2`). Not known: what that `u` is computed from, why radius √2 refuses and 1 and 0.3 mesh, and whether the `PlacedUnion` witness this row was filed on (a master ball an ulp off its cutting plane, `die_tool.rs` at `ccf433d0e4`) is the same defect or a second one at `can_add_constraint`. The unit answers those first and says so in the PR body.
- **Fix it where it starts.** A chart coordinate at a pole is not a number the chart defines; whatever reads one as if it were is the cause. A threshold typed in to absorb the dust is not a fix: a comparison that decides anything goes through the kernel's own band, and a new constant owes its derivation.
- **Done when** the probe is re-pinned to `Ok` with `check_mesh` at both δ and at radius √2, 1 and 0.3; the tube ∪ ball op of `reach_probes::a_ball_face_holding_a_tilted_ring_refuses_at_the_curved_lanes_ring_guard` is re-pinned to what it then answers; and the ulp-tilted witness either meshes or the PR says, measured, what it still needs.
- A mesh byte that moves (D9 digests, the budget baseline, tour pins) is re-baselined with its cause named.
