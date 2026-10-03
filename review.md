# Review of PR #3984, frozen head 8abb6e7931

Lane reach-dual3984-r2. Wall clock 21:17 – 22:27 UTC, 2026-10-03. Glimpses: none. I read only the PR body (`get`), and no comments, reviews or `analysis/reach-dual/*` branches.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 4. No wrong body, and no refusal that is anything but typed, through any public door I drove.

## Premise (correction to the brief, and to the PR body)
At 8abb6e7931 **the gate is kept** (`reduce.rs:449,473`). The variant is `CrossingCarrierUnsupported` (`mod.rs:1618`). Only the sweep's **planar arm** (`reduce.rs:1215`) and **curved arm** (`reduce.rs:2181`) are typed. The join's germ frame and ring run, which claim 1 lists, are untouched at this head, and so are the sector walk and the face extent. The PR's live body and title describe a later head (5501a3f03: the gate deleted, `EdgeCarrierUnsupported`, six sites), not this one. I reviewed the frozen head, so claims 1 and 2 are judged on "the PR keeps the gate".

## Correctness findings
1. **MINOR — the gate's stated reason is stale, and no measurement backs it at this head** (claim 2). By inspection. `reduce.rs:416-419` still justifies the gate as "rung-3 edges being what the zip MINTS rather than what it consumes", a premise the unit's own parent issue disputes. The real reason is the inline comment at `reduce.rs:479` ("no join, section or pierce arm reads a spiric"), and the unit item says only "The gate stays; it is no longer what keeps the arm sound" (`work/reach/planar-crossing-lane-reads-a-curved-carrier-as-a-line.md`). Nothing at this head records **which** sites the gate still protects. The PR body's later measurement names four: `JoinDesync` at the germ frame, `SectionInvariant` at the ring run, the NURBS chord at `build_sectors`, and `ClassificationInvariant` at the continuation scan. The pinned Python comment `test_north_star.py:3504` repeats the stale reason.
2. **MINOR — the PR body and title do not describe the reviewed head.** By inspection plus `git merge-base --is-ancestor`. A merge of this head on the strength of that body would claim a gate deletion it does not contain. The CI that verifies this head is run 37141807328 (green, on 8abb6e793); the run the body cites, 37151358002, is on 5501a3f03.
3. **MINOR — a public variant no public door can raise.** DEMONSTRATED. `CrossingCarrierUnsupported` adds a py tag (`tags.rs:1571`), an offer row, a `refusal_concision` entry and a recourse message. My probe A drove 120 public outcomes, and none reached it: the gate refuses first. It is the same fact as `CurvedEdgeUnsupported`, stated twice (`mod.rs:1601-1604` says so).
4. **NOTE — claim 1, the gate bypassed, is UNEXERCISED by me.** The permission system denied my gate-bypass probe, so per the brief I did not pursue it by another route. What stands is the PR's own in-crate rows, which call `sweep_direction` directly: all three are green at ε 1e-9, 1e-6 and 1e-12, and the mutants below kill them.
5. **NOTE — claim 3, no caller can read the result as a line.** By inspection. Every caller of `plane_crossing_lane` and `crossing_lane` matches exhaustively: `reduce.rs:1203,2928`, `classify.rs:719`, `offer_rows.rs:1028`, and none of them has a `_` arm. On the line lane, `touch_at_end`'s new `meet.is_none() ||` (`reduce.rs:1224`) behaves exactly as before: the line lane below makes the same `bool_vertex_face_side` decisions and raises the same coincidence error. Mutant M6 is equivalent.
6. **NOTE — claim 4, mutants** (`probes/reach-dual3984-r2/mutants.py`, DEMONSTRATED).
   - M1 (planar arm: Unlaned read as a line): killed, 2 rows red.
   - M2 (curved arm: back to `frontier()`): killed, the cylinder row red.
   - M3 (lane: a spline answered as `Line`): killed, 3 rows red.
   - M4 (`boundary_meets_circle_only_at`: Unlaned read as clear): killed by `lying_on_rows::a_nurbs_boundary_edge_does_not_certify`, in a full topo run.
   - M5 (the split's `insert_crossings` Unlaned arm skips `edge_clears`): **survives** all of topo and sweep's split, loft and spiric rows. The split's own `gate_operand` (`classify.rs:73-76`) runs the identical check first, so the arm at `classify.rs:734-739` is a second, unpinned copy (pre-existing).
7. **NOTE — off target (identical on main).** For rod∖brick (`d = 0.0137`), a further ∖ slab across `z ∈ [3.5, 4]` refuses "the solids do not cross", although the two solids overlap. `point_in_solid` on that body answers `WallOutlineUnsupported`. I have not measured the cause.

## End-to-end exercise (public doors; `probes/reach-dual3984-r2/probe_3984_r2.rs`)
- **A**: lofted boxes (NURBS walls) at ×1e-3, ×1 and ×1e3; the loft prism, plain and rigidly re-posed; the swept elbow; the vessel cavity (spiric). Each went against bricks placed far, across, inside and around, and through every op in both orders: 120 outcomes.
  - 96 are `CurvedEdgeUnsupported`, naming the right operand and a NURBS or spiric edge of it, with the new "spiric or spline" text.
  - 24 are main's typed `CurvedPairUnsupported{RevertRoster}`.
  - Nothing builds. The same at ε 1e-9, 1e-6 and 1e-12.
- **B**: the conic arm the refactor re-plumbed. Circle-edged rods ∪, ∩ and ∖ a brick, in both orders, at three scales and three cut depths, plain and posed, with the results reused as operands: 108 outcomes.
  - Volumes against the closed form (circular-segment area × overlap): worst relative error 1.3e-12.
  - 400 Halton points per body through `point_in_solid` against my own analytic membership: **zero** wrong answers. Every miss is a typed escalation or `WallOutlineUnsupported`.
  - Output is **byte-identical to the merge base 3cb7bc807** at all three ε, each tree built in its own target dir.
- Suites, all of topo, editor-core and pncad-py plus the touched sweep rows: 4948/4948 and sweep 57/57 at ε 1e-9 and 1e-6; at 1e-12 4947/4948, the one red being `rigid_map_near_eps_plane_nurbs`, which the brief records as main's.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q5 partly; Q8 on classify.rs's non-test half only)
- `classify.rs:466-485`, **sure**: `plane_crossing_lane` destructures Circle and Ellipse and builds a `geom_brep::Conic` by hand, which is what `Conic::of` (`implicit.rs:662`) already does. `curved_face_arm` calls `Conic::of` on the same carriers. That makes two spellings of "a conic's frame".
- `reduce.rs:2188`, **sure**: the new `(Circle|Ellipse, None) => ClassificationInvariant` arm cannot be reached (`Conic::of` is `Some` for both kinds). It exists only because the match keys on a pair that the type could have made one.
- `classify.rs:291-299`, **likely**: `crossing_lane` is a one-line wrapper over `plane_crossing_lane`, which gives three names for one door after the rename.
- Q4, **likely**: the renamed `conic_plane_crossing_roots` is still cited by open items:
  - `work/pipe/topo-shared-cores-hosted-in-one-half.md:4,29-31` (P1; its subject is this very function, hosted in the wrong half, with an open K-name question);
  - `work/pipe/S5.md:54`;
  - `work/hone/line-roots-carry-no-root-slack-meter.md:47`;
  - `work/restfront/an-ellipse-stored-minor-over-major-passes-tier-3.md:79`;
  - `docs/GERM-VERBS-CONE-SPEC.md:119`.

  The PR widens that wrongly homed core (it now answers for lines too) without moving it, and does not touch the P1 item.
- `mod.rs:2845`, **unsure**: "a spiric or spline curve **near** a face". A spline edge's box is the whole space, so the face named is the first one in arena order, not a near one.
- Q3, **likely**: the three planar-lane rows exercise code that no production path reaches at this head (the gate is first). They pin a future state rather than a current one, and nothing says when they become live.
- Q6, **likely**: the kept gate is a disclosed deviation from the brief's "type every site". The unit item schedules no follow-up for the four sites the gate still guards at this head.
