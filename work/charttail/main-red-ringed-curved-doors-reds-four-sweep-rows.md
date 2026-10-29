---
id: main-red-ringed-curved-doors-reds-four-sweep-rows
kind: issue
title: Main red: PR 5664's ringed curved doors reds bisector_nodes and review_4781_r1 at 1e-12, widens the meeting-axes pose to every eps, and the ray-wall twins to 62 pairs
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12, main-reds-editor-core-slow-set-and-the-k-probe-sweep]
---


Found by the SPLICE nightly triage of run 40274924857 (main
`38f1a24ed3`). Each of the rows below went red, or redder, between
the 10-24 nightly (`3dfb96b91c`) and the 10-25 one (`9f5b44561d`). I
bisected each one alone over that window's 35 merges. Every one passes
at `fa1d94c7db` and fails at `80a6f77af3`, the merge of **#5664**
(`chart/ringed-curved-doors`: the cylinder and cone point-in-face
doors read the face's region from its edges):

| row (sweep) | ε | before #5664 | tonight |
|---|---|---|---|
| `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself` | 1e-12 | green | `3° (B, A), ∖: the node builds, got Err(Pieces(Probe { shell: ShellKey(2v5), source: Loop(RayExhausted { loop: LoopKey(16v5) }) }))` |
| `review_4781_r1_probes::review_4781_touch_pairs_every_op` | 1e-12 | green | `box on seam, tall box s=1e3 … Union: REFUSED PointInFaceRefused { operand: A, face: FaceKey(3v1), refusal: RayExhausted }` (and `AMinusB` the same) |
| `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar` | default, 1e-6 | green (red only at 1e-12, `work/germ/main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12.md`) | `Escalated { decision: Containment { decision: None, escalation: Margin }, … predicate: Some("bool_ruled_region_span") }` |
| `ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model` (`--features probe`, 1e-9; stops `k-lint (dev-probe)`) | 1e-9 | red, 6 pairs off | `curve_foot_branch: 62 pairs off, e.g. 9.990e2` |

The verbs_germarms2 row is bisected at the default ε. The other three are
bisected at the ε shown.

In the first two rows, a ray the new region reading exhausts
(`RayExhausted`) surfaces as a refused point-in-face. That reads as the
new doors refusing where the old ones answered, at scale `1e3` and at
`1e-12`. This is unmeasured beyond the payloads quoted here.

#5664 also deleted `crates/topo/src/boolean/wall_section_rows.rs`
without dropping its `reader_census` ledger line. The triage PR
(`splice/nightly-triage`) drops that line, so it is not owed here.

**Owed.** For each row, decide whether the new refusal is right. A
refused point-in-face on a box against a seam is suspect. Fix the
door, or re-pin with the reason. The ray-wall row also keeps its
original 6 pairs, which predate #5664 (TCOST's row).

Reproduce one row: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=<row>)'`.

**At the default ε too (QUAD triage, 2026-10-27).** On main `3a0cd80f`,
`cargo nextest run -p sweep --features probe -E 'test(ray_wall_margin_twins::)'`
fails the same way at the default ε: `curve_foot_branch: 62 pairs off,
e.g. 9.990e2`.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`.

- `bisector_nodes::slanted_bores_…`: still red at 1e-12; pinned there
  (also `work/tang/main-red-slanted-bores-b-minus-a-exhausts-its-probe-ray-at-1e-12`).
- `review_4781_r1_probes::review_4781_touch_pairs_every_op`: its 1e-12
  stop moved again, from `RayExhausted` to limb 2's `KernelLimit` on a
  cylinder, since PR 5728. Pinned there and filed as
  `work/offset/main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728`.
  The `RayExhausted` stop this row names is now behind it, unmeasured.
- `verbs_germarms2_interval::the_meeting_axes_pose_…`: pinned at every
  ε on `bool_ruled_region_span`
  (`work/charttail/main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664`).

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured. Known behind the review_4781 r1 pin: this row's `RayExhausted`.

The base six ray-wall pairs are owned by
`work/issues/ray-wall-twins-curve-foot-branch-margin-scales-with-the-model-squared.md`
since 2026-10-28 (PR 5966), which also pins the row at today's 62.
