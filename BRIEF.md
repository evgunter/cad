# Reviewer brief: SECT PR 5924 (each torus pair decides tangency on one pose, the critical gap)

You are one of two independent reviewers in a concurrent pair, under
`docs/DUAL-REVIEW-PROTOCOL.md`. Review tier: FULL, meaning claims to
falsify plus the style lane.

**Frozen head.** `3bc5a10f115c8c47b3966b98878d9edaf08edf26`, from https://github.com/evgunter/cad/pull/5924
(branch `sect/torus-pose`). Review that commit only:
`git fetch origin sect/torus-pose && git checkout --detach 3bc5a10f115c8c47b3966b98878d9edaf08edf26`.

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
- the unit's spec, `work/sect/each-torus-pair-decides-tangency-on-one-pose.md`,
  as it stood on origin/main before this PR (the PR closes it). Read its
  "The design" and "Scope" sections;
- the C2, C5, C7 and C12.1 clauses of `crates/geom-brep/README.md`;
- the step-1a pattern, `work/sect/the-section-of-a-surface-pair-is-one-object.md`.

**Builds.**
- Point `CARGO_TARGET_DIR` outside the checkout, and set
  `CARGO_INCREMENTAL=0`.
- Run in the foreground. Anything that outlives a 600 s call runs
  `setsid`-detached with its PID recorded, and you poll it in the
  foreground. Leave no background work alive at the end.

**Known and out of scope.** `germ_torus_doors::subtract_and_intersect_refuse_where_union_does`
is red on main from PR 5869, an exact tube-ownership tie outside the
arm's triangle. A separate fix is in flight, so do not count that row
against this PR. Do check whether this PR's torus × torus gap reads that
pair right.

## Claims to falsify

A red probe or a measured value beats an argument.

1. **The critical gap is sound** (`pair_section/gap.rs`). It never reads
   a tangent pose Positive, and never reads a definitely transverse pose
   Zero.
   - Attack the following cases:
     - kinks of the torus × torus D, where the first core crosses the
       second torus's axis;
     - near-coaxial and near-parallel tilts;
     - the tube-circle tangencies (equal minor radii with cores touching,
       and `r_c = r` with the axis touching the core);
     - near-node poses;
     - spindle and fat tori near the ring convention;
     - non-unit f64 axes.
   - Check the clearing tests:
     - the R-Lipschitz value test;
     - the Taylor range of G = D²;
     - the `|G′|` lower bound from an exact `G‴` bound.
   - Use your own brute-force oracle, not the PR's.
2. **The envelope argument is complete.** Every critical value of the
   spine-to-spine distance is seen by the 1-D sweep over the core angle,
   on the axis as well as off it, for cylinder × torus and for
   torus × torus.
3. **The lever** is the torus's own ball `|x−c| ≤ R+r` (the smaller ball,
   for two tori), not a face reach. Check that this is sound, and say what
   it over-reads. Does any pose move verdict compared with `section_cert`'s
   old per-face reach?
4. **The moved closed forms are moved, not copied:** coaxial circles, and
   the parallel and square classifications. Check that no second spelling
   remains in `section_cert` or elsewhere, and rerun the hit list in a
   shape the PR could not have matched.
5. **Verdicts that moved.** Check each one the PR lists:
   - the donut × donut #4 change;
   - the tilted-torus R-reach that became R-tan, which the PR says is a
     genuine common-normal tangency, so verify it;
   - the axis-pose and square-lever re-derivations;
   - the boolean's new `GermSectionUnbuilt` refusals (`RoutesToGeneralRung`
     and `Untold`);
   - teapot wall 2 now reaching `CurvedBooleanUnsupported`.

   For each refusal, use the raising site and the payload, not the text.
   Search for an op that now returns a wrong body.
6. **`glue::section_door`'s per-decision `DecisionSite`s.** No
   coincidence record is lost or mislabelled, compared with main.
7. **Suites.** Run `topo`, `sweep` and `geom-brep` (`--profile ci`) on
   this head and on its main merge-base, and compare.

## Report

Your final message, at most 150 lines:
- a verdict: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- correctness findings, each marked MAJOR, MINOR or NOTE, with `file:line`
  and its evidence, and whether it was shown by execution or by inspection;
- `## Style`: the questions you exercised, each finding marked `sure`,
  `likely` or `unsure`;
- reviewer tests worth keeping, pasted as code (commit nothing);
- any glimpse, disclosed.
