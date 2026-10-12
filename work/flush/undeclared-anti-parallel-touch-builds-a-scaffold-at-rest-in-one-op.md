---
id: undeclared-anti-parallel-touch-builds-a-scaffold-at-rest-in-one-op
kind: issue
title: An undeclared anti-parallel touch at a wedge builds a body only the final validator catches, in one op/order per pose
status: closed
opened: 2026-10-03
priority: P2
cost: M
branch: flush/anti-parallel-scaffold
pr: 4649
closed: 2026-10-11
---


## What

The PR 3962 review's `rv_coplanar_battery`
(`crates/sweep/tests/join_reflex_wedge_review_probes.rs` on branch
`join/edge-edge-reflex-wedge-review`) runs pairs of prisms over irregular
star profiles that share a vertical corner edge. One flank of `b` lies
along one of `a`'s flank lines, pointing the opposite way: an
anti-parallel touch along that wall. With the coplanar pair
**undeclared**, 24 runs build a body that only the final validator
rejects, `ResultInvalid { errors: [ScaffoldAtRest { .. }] }`. On the
base of PR 3962 the same runs refused `ClassificationInvariant` at the
edge-edge membership.

For each pose, exactly one op in one operand order does this. The other
five runs, and all six runs of the flush-declared twin, are SOUND
(tier 2, tier 3′, certificate, closed-form volume). The poses
(`θa θb φb op`) are listed below. Each one fails under all three of the
battery's maps (`id`, `rot`, `proj`), so 8 × 3 = 24 runs:

- `40 200 180 S_ba`, `40 200 20 S_ba`
- `100 200 180 S_ba`, `100 200 80 S_ba`
- `230 170 180 S_ab`, `230 170 240 S_ab`
- `300 170 180 S_ab`, `300 170 310 S_ab`

These are re-run on the PR 3962 fix-pass head, merged with main after
JOIN-3. To reproduce, check out the review branch and run
`cargo test --release -p sweep --test all rv_coplanar_battery --
--ignored --nocapture`, then grep `ResultInvalid`.

The result is loud, not wrong: no body ships. What is odd is the
asymmetry, one subtraction in one operand order. Nobody has
root-caused it yet.

## Measured

Re-measured with the plain booleans (`topo::union` / `intersect` /
`subtract`, no declaration) through the battery's port,
`crates/sweep/tests/a_flank_touch_at_a_shared_corner.rs`
(`flank_touch_battery`, ignored, with the review's poses, maps and
closed form).

- **Current main** (`19b704d92`): all 24 runs of the 8 poses are
  SOUND. The whole battery bar `θa = 180` gives 2 304 runs: 2 124
  SOUND, 180 `EMPTY ok`, nothing else. That covers the parallel and
  anti-parallel flank poses, every op in both orders, under `id`, `rot`
  and `proj`.
- **`θa = 180`** is not measurable: the fixture `a` is no finished body.
  Its corner edge is scaffolding between two walls on one plane, and
  `finished` refuses it as `ScaffoldAtRest`. The review battery fed
  ungated bodies, so those 96 poses were never sound evidence. They are
  dropped from the port.
- **PR 3962's merge** (`d0d3490bc`): the 24 runs reproduce, each
  `ResultInvalid { errors: [ScaffoldAtRest { .. }] }`.
- **Bisected** on main's first-parent line between `d0d3490bc` and
  current main: the first fixed commit is `80fc1b19a`, the merge of
  PR 4496 (INTENT stage 4 E, "booleans glue on Zero and record;
  provenance retires"). Its parent `5c9315aed` still fails all 24.

## Root cause

**The asymmetry is geometric.** At each of the 8 poses, exactly one
op leaves the corner a single half-turn: one flank from each operand,
collinear and on one side. In `40 200 180`, `b` spans 180°→20° and `a`
spans 0°→40°, so `b ∖ a` is 180°→360°. Its two walls are `b`'s 180°
flank and `a`'s 0° flank reversed. They lie on one plane with one
outward normal, and the corner edge between them is the only edge
there. No other op at that pose leaves that shape.

**What was missing is the face merge.** Before PR 4496,
`merge_coplanar_faces` read neighbours through `planes_declared_equal`
(`crates/topo/src/merge_faces.rs`). It merged two faces on distinct
surface keys only when the pair was declared (or same-source). An
undeclared pair from the two operands was "not a candidate", so the
corner edge survived between two coplanar faces and only the
finished-body gate saw it.

PR 4496 replaced that with `faces_continue`. It decides an undeclared
pair by `oriented_plane_eq` with `PlaneIdentity::NONE`, through
`undeclared_pair_verdict`, and glues `SameOriented`. That is D10's
"glues what its verdicts decide Zero". So the glue fires here now, with
no declaration.

**The class.** 18 pose-ops in the battery leave the corner a single
half-turn: the 8 subtractions, `a ∪ b` at 4 poses and `a ∩ b` at 6.
At `5c9315aed` the union and intersect ones refused
`UndeclaredCoincidence` in both orders, before any merge, so the
review only saw the subtractions. All 18 are SOUND on main in every
order and map.

## Fix

No kernel change: PR 4496 fixed it. This row adds the regression gate,
`a_flank_touch_glues_the_half_turn_it_leaves`: the 18 class poses ×
6 runs × 3 maps, each SOUND or `EMPTY ok`. It is red at `5c9315aed`
(the first pose, `ScaffoldAtRest`). It takes 10.7 s in debug, so it is
in the `ci` profile's slow set.

## Closed

PR 4496 fixed it. INTENT stage 4 E made `merge_coplanar_faces` glue an
undeclared coplanar pair on a Zero verdict (`faces_continue`) instead of
only a declared one. The 8 poses are SOUND on main, and `5c9315aed`, just
before PR 4496, is the last commit that fails. PR 4649 pins the whole
class, the 18 pose-ops (the 8 subtractions plus 4 union and 6 intersect
poses that refused `UndeclaredCoincidence` before PR 4496), with
`a_flank_touch_glues_the_half_turn_it_leaves`, which is red at
`5c9315aed`.
