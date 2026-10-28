# Reviewer brief: SECT PR 5894 (step 1b, the cylinder × sphere chord and its island closure)

You are one of two independent reviewers, a concurrent pair under
`docs/DUAL-REVIEW-PROTOCOL.md`. Review tier: FULL, meaning claims to
falsify plus the style lane.

**Frozen head:** `6e71fe1134db5ac6e00a0d65abc3d497f999cef0`
(https://github.com/evgunter/cad/pull/5894, branch
`sect/cylinder-sphere-chord`). Review that commit only:
`git fetch origin sect/cylinder-sphere-chord && git checkout --detach 6e71fe1134`.

**Lane isolation.** Until your report is delivered:
- read the PR body only, through the GitHub MCP tools via ToolSearch
  (there is no gh);
- do not read the PR's comments or reviews, any other session's
  output, or any `review/*` branch;
- post nothing on GitHub and push nothing;
- disclose any accidental glimpse in your report.

**Read first, in full:**
- `docs/prompts/reviewer-style-lane.md`, which binds your style half;
- `CLAUDE.md`;
- the row `work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`
  as it stood before this PR: `git show origin/main:work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`.
  Read its "The shape to give" and "Split at design" sections;
- the step-1a row `work/sect/the-section-of-a-surface-pair-is-one-object.md`,
  which holds the design this builds on.

**A decision already made, not to re-litigate:** a closed fitted loop is
seated once at its first site, by knot insertion and rotation, then
re-certified. Periodic NURBS were not added.

**Builds:**
- Point `CARGO_TARGET_DIR` outside the checkout and set
  `CARGO_INCREMENTAL=0`.
- Run in the foreground. Anything that outlives a 600 s call runs
  `setsid`-detached with its PID recorded, and you poll it in the
  foreground. Leave no background work alive at the end.
- The per-pose fixture tests take 3–120 s each in debug.

## Context

On main, all 54 runs of `crates/sweep/tests/cylinder_sphere_frame.rs`
refused at `bool_connect`. On this head every run passes the join, then
stops typed at a door downstream:
- props has no `Projected` boundary arm on a cylinder or sphere chart,
  and its ring gate refuses;
- `edge_join` refuses `JoinCarrierUnsupported { Nurbs, open }`.

No run builds a body, so the fixture's volume oracle checks nothing yet.
**Your job is to check the chords and closures directly.**

## Claims to falsify

A red probe or a measured value beats an argument.

1. **Each chord is the right sub-arc.** Extract the chord edges that 1b
   mints, before the downstream door refuses, for the poses in the table
   in the PR body. Check each of the following:
   - each chord lies on both surfaces within band;
   - it runs between its two partner sites, in the direction the germ
     leaves, never the complementary arc;
   - each section loop is covered by the op's chords exactly as the op
     needs, with no gap or overlap;
   - both solids' copies cut the same carrier bits.

   Use an independent oracle: the analytic quartic, or a dense sampling
   of the true section.
2. **Seating is sound.**
   - The seated carrier is the same point set as the marched one.
   - The re-certification is real.
   - No chord straddles the seat.
   - A chord that would cross it refuses typed (`chord_carried_forward`).

   Construct a pose where the first site in match order makes the seat
   fall inside a later chord, if you can.
3. **Island closure is right.** Run a counterexample search, logging its
   seed, of the following against an oracle parity:
   - `RingClosure::Chord`;
   - the new `ring_path::Quadric::Cylinder` (`Piece::Ruling`);
   - `LoopArc::Spline` (crossing parity through `curve_pair::meet`).

   Include islands whose boundary runs along a ruling, round a
   parallel, and through near-tangent crossings.
4. **`AxialCarrier::Hull` is sound.** A spline edge's axial span is read
   from its control points. Check that it is always an outer bound on
   the curve's true axial span, never inner, including for rational
   carriers and under the merge's use of it.
5. **Coaxial pairs still refuse at the join**, as the PR says. Confirm
   that no coaxial pose now builds anything.
6. **No wrong bodies and honest refusals.**
   - Each downstream refusal (P, S, R, J) is raised where the PR says,
     for the reason it says. Use the raising site and the payload, not
     the text (`memories/refusal-text-is-not-cause.md`).
   - Search for any op on this head, in the fixture or beyond it, that
     now returns a body. Check that body against an independent volume.
7. **Nothing that built before breaks or slows badly.**
   - Run the boolean suites (`topo`, `sweep`) on this head and on main.
   - Compare their failures, the ops that build, and the time of
     anything that built on main.
   - Measure the limb-2 cost per chord the PR reports, and say whether
     cutting the arc rather than keeping the whole loop is sound.
8. **The re-pins are honest:**
   - `verbs_cylsph_opening::a_transversal_pose_reaches_the_join_lane_in_both_poses`;
   - `conic_edge_curved_face::a_rim_crossing_reaches_the_join`;
   - `join_carrier_site_rows::a_ring_run_along_a_spline_edge_refuses_at_the_ring_run`;
   - the per-pose fixture split, `pose_rows!`.

## Report

Your final message, at most 150 lines:
- **Verdict:** APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT.
- **Correctness findings:** each MAJOR, MINOR or NOTE, with `file:line`
  and its evidence. Say whether each was shown by execution or by
  inspection.
- **`## Style`:** the questions you exercised, with each finding marked
  `sure`, `likely` or `unsure`.
- **Reviewer tests worth keeping,** as code. Paste them in the report;
  commit nothing.
- **Any glimpse,** disclosed.
