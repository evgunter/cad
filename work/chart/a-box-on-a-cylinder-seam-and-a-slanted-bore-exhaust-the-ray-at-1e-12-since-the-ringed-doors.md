---
id: a-box-on-a-cylinder-seam-and-a-slanted-bore-exhaust-the-ray-at-1e-12-since-the-ringed-doors
kind: issue
title: Since PR 5664's ruled region doors, three sweep rows refuse - a box on a cylinder's seam and a 3° slanted bore exhaust the ray at 1e-12, and the meeting-axes pose at interval escalates containment over a whole turn at every eps
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [curved-face-containment-answers-none-on-a-ringed-face, 5664]
---


Found by INTENT's nightly triage of runs 40165703340 (10-25, main
`9f5b44561d`) and 40274924857 (10-26, `38f1a24ed3`). Three `sweep` slow-set
rows went red between 10-24's run (`3dfb96b91c`) and 10-25's run. All
three are still red on main `46b97533`. The first two are red at ε 1e-12
only. The third was red at 1e-12 alone before, for its own reason, and is
now red at every ε.

## What

- `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`
  (`crates/sweep/tests/bisector_nodes.rs`, in `built`):

      3° (B, A), ∖: the node builds, got Err(Pieces(Probe { shell: ShellKey(2v5), source: Loop(RayExhausted { loop: LoopKey(16v5) }) }))

- `review_4781_r1_probes::review_4781_touch_pairs_every_op`
  (`crates/sweep/tests/review_4781_r1_probes.rs`, the per-pose
  outcome check). Every op of both orders of the `box on seam, tall box
  s=1e3` pose refuses this way:

      eps 1e-12 box on seam, tall box s=1e3 ab coarse=false past_headroom=true | Union: REFUSED PointInFaceRefused PointInFaceRefused { operand: A, face: FaceKey(3v1), refusal: RayExhausted }

  The row accepts a `past_f64_headroom` pose refusing `Escalated` or
  `CurvedPierceUnsupported`, which is the two-tolerance pair a kiss at
  that reach can honestly take. `RayExhausted` from the point-in-face
  door is neither of them. The staggered-cylinder poses on the same row
  still refuse the accepted way.

- `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`
  (`crates/sweep/tests/verbs_germarms2_interval.rs`), at default, 1e-6
  and 1e-12:

      the meeting-axes pose builds: Err(Escalated { decision: Containment { decision: None, escalation: Margin }, diag: Indeterminate { margin: MarginDiag(Enclosure { lo: -1.5707963267956369, hi: 4.712388980385431, … }), … })

  The margin enclosure is `[-π/2, 3π/2]`, a whole turn. Until 10-24 this
  row was red at 1e-12 only, with a different refusal (`EndpointEnd`,
  `work/germ/main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12.md`).
  That refusal is now hidden behind this one at every ε. The f64 twin,
  `verbs_germarms2::every_pose_of_the_family_builds_at_its_closed_form`,
  is green.

## Bisect

Each run: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all
--run-ignored all -E 'test(slanted_bores_build_every_operation_one_difference_pinched_to_itself) | test(review_4781_touch_pairs_every_op)'`.

- `fa1d94c7` (#5669, the first-parent merge before #5664): both PASS.
- `80a6f77a` (#5664, `chart/ringed-curved-doors`): both FAIL, with the
  messages above.
- `46b97533` (main, 2026-10-26): both FAIL, the same messages.

The meeting-axes row was run alone at the default ε
(`cargo nextest run -p sweep --test all --run-ignored all -E 'test(the_meeting_axes_pose_builds_at_the_certified_scalar)'`):
PASS at `fa1d94c7`, FAIL at `80a6f77a` with the message above.

## Reading

#5664 replaced the cylinder and cone point-in-face doors' chart-trim
inventory with edge readings (`topo::boolean::ruled_region`), and it
deleted `wall_outline`, `point_on_wall_in_face` and the rest, with no
fallback kept. The two ray rows put a point on a cylinder wall (a box
face on the seam, and the 3° bore's wall). At 1e-12 the new reading runs
out of rays there, where the old one answered. On the meeting-axes pose
at `Interval`, the containment decision's margin comes back as a whole
turn of angle, and no ε can decide that. It looks like an azimuth read
with no window, which is what `ruled_region` says it does ("no azimuth
window, height range or period guard is read"), reaching a margin that
expects one.

This is not the sliver ray-start defect in
`work/inside/aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots.md`.
That one is a wrong `In`. This one is a refusal, on a pose of ordinary
scale (s = 1e3 against ε = 1e-12, which is inside the f64 headroom the
test grants).

#5664 also widened the `ray_wall_margin_twins` k-probe row from 6 pairs
off to 62. SPLICE's triage PR 5754 records that on
`work/tcost/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`. It
may be the same reading seen from the k side.

## Owed

Decide which aim or leg the ruled reading exhausts on at 1e-12 for the
two ray poses, and where the meeting-axes pose's whole-turn margin comes
from. Then either give it a path that settles, or show that the
refusal is the honest answer and re-pin the rows to accept it, saying
why. All three rows are in the slow set, so #5664's gate never ran them. The
nightly reds on them each night, and so does any PR whose diff names them.
