# Reviewer brief: SECT PR 6011 (a section's components are the march's; a site's component is its foot)

You are one of two independent reviewers in a concurrent pair, under
`docs/DUAL-REVIEW-PROTOCOL.md`. Review tier: FULL, meaning claims to
falsify plus the style lane.

**Frozen head.** `b4a0d86585543c32ac547bdfccf8506e31fb9bea`, from
https://github.com/evgunter/cad/pull/6011 (branch `sect/components-by-foot`).
Review that commit only:
`git fetch origin sect/components-by-foot && git checkout --detach b4a0d86585543c32ac547bdfccf8506e31fb9bea`.

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
- the decided design: `work/sect/a-general-torus-pose-lists-no-components.md`
  as it stood on origin/main before this PR (its `## Decided` section is the
  spec; it is settled, do not re-litigate it — review the build against it);
- the C2, C5, C7 and C12.1 clauses of `crates/geom-brep/README.md`;
- `work/sect/the-section-of-a-surface-pair-is-one-object.md` (step 1a).

**Builds.**
- Point `CARGO_TARGET_DIR` outside the checkout, and set
  `CARGO_INCREMENTAL=0`.
- Run in the foreground. Anything that outlives a 600 s call runs
  `setsid`-detached with its PID recorded, and you poll it in the
  foreground. Leave no background work alive at the end.
- **Budget:** deliver within about 3 hours. Some of these marches take tens
  of minutes in debug (use `--release` for probes where you can); cut a
  long run short rather than overrun, and say which.

## Claims to falsify

A red probe or a measured value beats an argument.

1. **`Counted` / `Marched` cannot be misread.** No reader treats `Marched`
   as "no section", and an empty `Counted` is read as disjoint only where a
   closed form proved it. Check every reader the PR's hit list names, and
   search for any it missed.
2. **`winds` is sound** (`pair_section/winds.rs`): integer turns about the
   axis and the core circle, from each piece's control hull enclosed in
   intervals and padded by the tube radius; a piece whose padded box misses
   the centre turns by less than π. Falsify with an independent winding
   count (dense sampling of the true section, your own frame) on random
   transverse poses, including loops near the axis, near the core, and
   near-tangent poses; check the cut budget's refusal is typed.
3. **The Counted cross-check.** The PR deviates from the spec: it checks
   only that each winding the arm *claims* holds (claimed ⇒ marched), since
   the parallel-tori arm lists `false` without reading essentiality. Does
   any reader then use a Counted component's `false` as "null" (the
   certificate's W2, the join, island closure)? Construct a Counted pose
   whose true winding differs from its listed one and see what each reader
   does. Also: is the witness check (each listed witness on exactly one
   branch, within the band widened by the branch's tube) sound and does it
   catch a missed or doubled loop?
4. **One site reading.** A germ site's component is the carried section's
   foot (`Branch::Component(k)`). The foot must be unambiguous wherever the
   march certified (tube clearance). Look for a site the foot assigns to the
   wrong component, or one that two components both claim, at poses with 3
   or 4 loops and at loops that come close. Check the frame-era pairs still
   read `Branch` as before.
5. **The certificate** (`section_cert`): a `Marched` pose read from its
   march (W0–W4, W3's band widened by the tube radius); `Refusal::NoLane`,
   `Refusal::Unmarched`; the former wildcard arm now exhaustive. Check W0
   (empty branch list with a full receipt) cannot certify disjoint faces
   that meet, and that `classify` taking the lane changes no verdict for
   lane-free pairs.
6. **No wrong bodies.** The PR's before/after table: the tilted cut with
   the torus just clear builds every op (volumes checked against `π/8` and
   `2π²Rr²`). Check those volumes independently, and search for any other
   op on this head that now returns a body; check it against an independent
   volume. For each refusal, use the raising site and payload, not the text
   (`memories/refusal-text-is-not-cause.md`).
7. **Cost.** The PR reports two sweep rows not finishing within an hour at
   ε 1e-12 in debug, and the tilted-rod spins row taking ~1560 s at the
   compiled ε. Measure what the nightly will pay (it runs the slow set at
   every ε) and whether any row risks the nightly's time limit
   (`.github/workflows/nightly.yml`). Say whether the cost is the march's
   (SECTRUNG) or this diff's.
8. **Re-pins honest.** Each re-pinned row in the PR body, and the new
   `crates/sweep/tests/germ_torus_components.rs`.
9. **Suites.** `geom-brep`, `topo`, `sweep` (`--profile ci`) on this head and
   its main merge-base: failures and times.

## Report

Your final message, at most 150 lines:
- a verdict: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- correctness findings, each MAJOR, MINOR or NOTE, with `file:line`, its
  evidence, and whether it was shown by execution or by inspection;
- `## Style`: the questions you exercised, each finding `sure`, `likely`
  or `unsure`;
- reviewer tests worth keeping, pasted as code (commit nothing);
- any glimpse, disclosed.
