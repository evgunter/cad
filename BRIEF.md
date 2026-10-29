# Reviewer brief: SECT PR 5974 (teapot wall 2: sphere × torus onto a torus face)

You are the single FULL reviewer (claims to falsify plus the style lane)
for this unit.

**Frozen head.** `0ebc2aaa2ecf03d8e27e99e65d5910431ff2e2bc`, from
https://github.com/evgunter/cad/pull/5974 (branch `sect/teapot-wall-2`).
Review that commit only:
`git fetch origin sect/teapot-wall-2 && git checkout --detach 0ebc2aaa2ecf03d8e27e99e65d5910431ff2e2bc`.

Read the PR body through the GitHub MCP tools via ToolSearch (there is
no gh). Post nothing on GitHub and push nothing.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md` (binds your
style half); `CLAUDE.md`; the C2, C5, C7 and C12.1 clauses of
`crates/geom-brep/README.md`; step 1b's closed row
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md` (the carried
lane, `RingClosure::Chord`, `LoopArc::Spline`, the cylinder ring-path arm
this PR extends to the torus).

**Out of scope, decided elsewhere.** `Sides::Untold` poses (a sphere cutting
both tube circles) stay refused: a design decision retires `component_of`
and `Sides` for a foot-based site reading in a later unit. Teapot wall 2's
stop at the volume door (SPIRICARM's
`a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`) is not this
PR's.

**Builds.** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`.
Foreground only; anything past 600 s runs `setsid`-detached with its PID
recorded and polled in the foreground; leave nothing running.

## Claims to falsify

A red probe or a measured value beats an argument.

1. **The torus ring-path arm is sound.** `join::choose_roles` sends a chord
   closure on a torus face to `RingFace::Carried`; `chord_join::run_loop_arcs`
   reads a certified spline edge on a torus face. The PR argues a crossing on
   the surface is a meeting in space, with no chart. Look for an island on a
   torus face whose parity this reads wrong: a ring that does not bound a disc
   of the face (wrapping either period), a spline arc near the tube's inner
   equator, a near-tangent crossing, an arc crossing the torus's seams.
   Run a seeded search against an independent membership oracle, logging the
   seed; also judge the PR's fuzzer
   (`ring_path::spline_fuzz::a_path_across_a_carried_torus_oval_reads_the_membership_parity`):
   is its oracle independent, does it cover what it claims?
2. **The matcher fix** (`join::along_facing` refuses two germs whose feet lie
   on different components). Is it right, is it the only such reader, and
   does it change any verdict on main's rows? Build a row that exercises it,
   if one can be built without the reverted `Sides::Trace`.
3. **Measurement honest.** The 24-run ball × donut table in the PR body:
   reproduce it on this head and on its main merge-base. For each refusal
   use the raising site and the payload, not the text
   (`memories/refusal-text-is-not-cause.md`). Search for an op that now
   returns a body, and check any against an independent volume.
4. **Pins honest:** `crates/sweep/tests/germ_sphere_torus_placements.rs`,
   the teapot wall 2 pin, the radial hole and lily pins.
5. **Suites.** `topo` and `sweep` (`--profile ci`) on this head and on the
   merge-base: compare failures and times.

## Report

Final message, ≤150 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); correctness findings MAJOR/MINOR/NOTE with
`file:line`, evidence, executed or by inspection; `## Style` with each
finding `sure`/`likely`/`unsure`; reviewer tests worth keeping, as code
(commit nothing).
