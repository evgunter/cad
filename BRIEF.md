# Reviewer brief: SECT PR 6047 (a cylinder face re-homes its rings by the path)

You are the single FULL reviewer (claims to falsify plus the style lane).

**Frozen head.** `3872c6c8fd` of https://github.com/evgunter/cad/pull/6047
(branch `sect/rehome-by-path`): `git fetch origin sect/rehome-by-path && git checkout --detach 3872c6c8fd`.
Read the PR body through the GitHub MCP tools via ToolSearch (no gh). Do not
read its comments or reviews. Post nothing, push nothing.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`; `CLAUDE.md`;
the C2, C5 and C7 clauses of `crates/geom-brep/README.md`; the row
`work/sect/a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord.md`
as it stood on main; step 1b's row
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`.

**Builds.** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`;
foreground; anything past 600 s `setsid`-detached and polled; leave nothing
running. Use `--release` or `--profile ci` for sweep rows. **Budget: about 2
hours**; cut a long run short rather than overrun, and say which.

## Claims to falsify

1. **Re-homing by the path is sound on a cylinder face.** The PR deletes the
   chart walk (`chart_ring_side`) and argues that on a cylinder face (an
   annulus) every path gives the same parity. That holds for a ring bounding a
   disc of the face; what about a ring that winds the cylinder (an essential
   loop, a full-period ring), or a face that is a full annulus with two such
   rings? What does `path_ring_side` answer there, and does the code refuse
   rather than read a parity that depends on the path? Construct such faces
   (a full-turn ring on a wall, a chord across a full-period wall) and run a
   seeded search against an independent membership oracle.
2. **The `ring_path::path_parity` change** (a segment with one end on an arc's
   plane and the other off it meets the plane once, at that end, read as a
   crossing) changes the reading on every curved face kind (sphere, cone,
   torus, cylinder). Is it sound — can it flip a parity it should not, at an
   end exactly on an arc, near-tangent, or where the end lies on the arc's
   plane but off the arc? Run the existing `ring_path` fuzzers and the
   sphere/cone/torus re-homing rows on head and merge-base and compare.
3. **What was deleted** (`chart_ring_side`, `run_window_over`, the
   `split_ring_chart_*` predicates, their agreement row): nothing else read
   them, and no verdict anywhere moved except those the PR discloses
   (`pocket_ring_steep_ellipse` restored, drum × donut placement 5 re-pinned).
   Search for an op that now returns a body; check it against an independent
   volume. For each refusal use the raising site and payload, not the text.
4. **Rows honest:** `wall_ring_rows` (renamed from `chart_ring_rows`, same
   verdicts), the restated full-period row, the new seeded search, the
   two-ball sweep row, the `ring_path` segment row (red without the fix?).
5. **Cost:** about 2 s per ring in debug on a spline run edge (bisection walk
   per path). Measure on the slow rows at opt-level 2 at the default ε and
   1e-12; the nightly's 180-minute full-suite job is tight.
6. **Suites:** `topo`, `sweep` (`--profile ci`) on head and merge-base.

## Report

Final message, ≤150 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); findings MAJOR/MINOR/NOTE with `file:line`,
evidence, executed or by inspection; `## Style` with each finding
`sure`/`likely`/`unsure`; reviewer tests worth keeping, as code (commit
nothing).
