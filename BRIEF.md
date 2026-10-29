# Reviewer brief: SECT PR 5972 (the cylinder × torus and torus × torus doors)

You are one of two independent reviewers in a concurrent pair, under
`docs/DUAL-REVIEW-PROTOCOL.md`. Review tier: FULL, meaning claims to
falsify plus the style lane.

**Frozen head.** `f2bd69c80a24b7069753831fa90828b22d41f9a6`, from
https://github.com/evgunter/cad/pull/5972 (branch `sect/torus-doors`).
Review that commit only:
`git fetch origin sect/torus-doors && git checkout --detach f2bd69c80a24b7069753831fa90828b22d41f9a6`.

**Lane isolation.** Until your report is delivered:
- read the PR body only, through the GitHub MCP tools via ToolSearch
  (there is no gh);
- do not read its comments or reviews, any `review/*` branch, or any
  other session's output;
- post nothing on GitHub and push nothing;
- disclose any accidental glimpse in your report.

**Read first, in full:**
- `docs/prompts/reviewer-style-lane.md`, which binds your style half;
- `CLAUDE.md`;
- the unit's two rows as they stood on origin/main before this PR (the PR
  closes them): `work/sect/cylinder-torus-rung-three-arm-is-unretired.md`
  and `work/sect/torus-torus-rung-three-arm-is-unretired.md`, their
  "Designed" and "Opened" sections;
- the C2, C5, C7 and C12.1 clauses of `crates/geom-brep/README.md`;
- the closed row `work/sect/each-torus-pair-decides-tangency-on-one-pose.md`
  (the critical gap these doors read; settled, not to re-litigate).

**Out of scope, decided elsewhere.** A general cylinder × torus or
torus × torus pose still answers `RoutesToGeneralRung` in `pair_section`:
the lane stopped on a design fork (how a general pose lists its
components) and filed it as `work/sect/a-general-torus-pose-lists-no-components.md`.
Do not review that choice. Do check that nothing on this head reads the
missing list as "no section" (`section_cert::of_pair_section`, the germ
mint `join::section_branch`).

**Builds.**
- Point `CARGO_TARGET_DIR` outside the checkout, and set
  `CARGO_INCREMENTAL=0`.
- Run in the foreground. Anything that outlives a 600 s call runs
  `setsid`-detached with its PID recorded, and you poll it in the
  foreground. Leave no background work alive at the end.
- Each door pose costs about 3–10 s in debug.

## Claims to falsify

A red probe or a measured value beats an argument.

1. **The march's domain holds the whole section** (`PairPose::ball()`,
   `march_over_the_ball`). Every section point lies inside the ball; the
   cube built 1% outside it closes every branch inside; for two tori the
   smaller ball is taken, and a cylinder's ball is never used. Look for a
   pose whose section leaves the domain or whose branch the exhaustiveness
   accounting closes wrongly (a branch cut at the cube, a loop missed).
2. **Each door decides the pose once.** No second tangency decision; a
   tangent gap refuses the pair's own `PairTangent` before any march;
   coincident tori refuse; an escalated gap never marches. Search for a
   tangent or in-band pose that marches, and for a transverse one refused.
3. **The door's branches are right and complete.** Against an
   independent oracle (exact distance to each surface, and an independent
   loop count, e.g. a contour of the implicit on the torus's chart): every
   certified branch lies on both surfaces within its certified bound, and
   no loop is missed or doubled. Run a seeded counterexample search over
   random transverse poses, logging its seed.
4. **The symmetric poses' carriers.** The square-wall and parallel-axis
   components now carry the general rung's carrier, and `Unbuilt`
   (`ClassCarrier`, `CarryError`, topo's `SectionUnbuilt`) is deleted.
   Check that each carried component is the component it claims to be,
   and that `Site(Untold)` refusals are raised where two components
   cannot be told apart, never folded into a wrong pick.
5. **`march_past_the_gap` is a pure refactor** for the sphere doors:
   cylinder × sphere and sphere × torus answer what they did on the
   merge-base, by verdict and by carrier.
6. **`route` / `route_pose` flipped.** No caller now reaches a door with
   an operand the door cannot honour (an unbounded cylinder; a horn or
   spindle torus; a coincident pair).
7. **The off-ε rows are honest.** Rows that accept typed refusals only
   off the compiled ε (`TubeStraddles` at 1e-6, `StepBudget` at 1e-12)
   must not accept a wrong answer. Run the new rows at 1e-6, 1e-9 and
   1e-12, and say what each accepts.
8. **Re-pins:** `intersect_table::route_inventory`,
   `m5_pr7_ssi::the_c5_table_retires_the_arm_whose_proof_is_complete`,
   the lily pin in `crates/sweep/tests/mate7a_r1_probes.rs`, and the
   radial hole pin `crates/sweep/tests/germ_radial_hole.rs`. For each
   refusal, use the raising site and the payload, not the text
   (`memories/refusal-text-is-not-cause.md`). Search for an op that now
   returns a wrong body.
9. **Suites.** Run `topo`, `sweep` and `geom-brep` (`--profile ci`) on
   this head and on its main merge-base, and compare failures and times.

## Report

Your final message, at most 150 lines:
- a verdict: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- correctness findings, each marked MAJOR, MINOR or NOTE, with `file:line`
  and its evidence, and whether it was shown by execution or by inspection;
- `## Style`: the questions you exercised, each finding marked `sure`,
  `likely` or `unsure`;
- reviewer tests worth keeping, pasted as code (commit nothing);
- any glimpse, disclosed.
