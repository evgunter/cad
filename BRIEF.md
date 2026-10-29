# Reviewer brief: SECT PR 5994 (a torus pair's tangency and coincidence decided once)

You are the single FULL reviewer (claims to falsify plus the style lane).

**Frozen head.** `b46c4517` of https://github.com/evgunter/cad/pull/5994
(branch `sect/torus-decided-once`): `git fetch origin sect/torus-decided-once && git checkout --detach b46c4517`.
Read the PR body through the GitHub MCP tools via ToolSearch (no gh).
Post nothing on GitHub and push nothing.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md` (binds your
style half); `CLAUDE.md`; the C2, C5 and C7 clauses of
`crates/geom-brep/README.md` (C5 holds the ratified rule, PR 5975, that a
pair decides tangency and coincidence once, read by every reader); the two
rows as they stood on main:
`work/sect/torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose.md`,
`work/sect/a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane.md`.

**Builds.** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`.
Foreground only; anything past 600 s runs `setsid`-detached with its PID
recorded and polled in the foreground; leave nothing running. Budget:
deliver within about 2 hours; cut a long run short rather than overrun,
and say so.

## Claims to falsify

A red probe or a measured value beats an argument.

1. **`tori_coincide` is sound** (`pair_section/torus_coincidence.rs`,
   `TorusDisplacement`). The bound is never under the true distance between
   the two tori at any point of either (check against an exact-distance
   oracle of your own, not the PR's); `One` only where the tori are within
   ε; `Apart` only where they are definitely not one surface (is one
   datum past the band always enough? a tilt levered at the larger major
   radius?); everything else `Undecided`. Look for horn/spindle-adjacent
   tori, small r against large R, tilts near π, and order dependence.
2. **One decision, every reader.** `TorusTorusPose::decide` and
   `carrier_eq`'s undeclared torus arm answer the same decision for every
   pair; the declared reading is argued to be a different question — is
   it? Does `vtxfac::one_carrier` (which now reads the moved tilt datum)
   change any verdict on main's rows?
3. **What moved is honest.** The PR says pairs whose data are all in band
   but not decided one now refuse undecided (main built crossing parallels
   for some, e.g. ΔR = Δz = 9ε), and that nothing main decided coincident
   is lost. Reproduce both on head and merge-base; search for any op whose
   verdict changed, and for a wrong body.
4. **Row 2's argument.** The gap (is the pair tangent, where) and the glue
   door's `tangency_reading` (the held rim within ε of both surfaces at every
   sample) ask different questions, and the PR pins a pose where the gap
   reads Tangent and the touch escalates. Check the pinned pose and the
   converse claim (a touch within ε all round the rim puts both gap
   readings within ε). Is there a pose where the glue door passes and the
   gap refuses, or where they give contradictory definite answers?
5. **The k-lint gates and slow set:** `tt_coincident` and the four
   `carrier_torus_*` rows are now read on every torus pair. Run the k-lint /
   stats gates the repo has (`scripts/gates/*.sh`) and the new slow row at
   1e-6 and 1e-12.
6. **Suites.** `geom-brep`, `topo`, `sweep` (`--profile ci`) on head and
   merge-base: failures and times.

## Report

Final message, ≤150 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); correctness findings MAJOR/MINOR/NOTE with
`file:line`, evidence, executed or by inspection; `## Style` with each
finding `sure`/`likely`/`unsure`; reviewer tests worth keeping, as code
(commit nothing).
