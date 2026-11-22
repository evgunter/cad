# Delta review — PR 7219, `oneround/lever-sites-brep`, frozen at `c65b9292`

Delta read: `37da28e8..c65b9292` (the fix pass and the merge of `main`,
`a80623a5`). Merge base with `main`: `3900eb46`. Hosted CI: run
45061123383. First review: `review-report.md` beside this file.

## Verdict: APPROVE

The MAJOR is closed: the gate reads 47 against the base's 59 and goes red
on the calls it was green on. The three surviving mutants and the
Face-arm ones are red. No reproducible decision moves, base against head,
at either ε. Nothing new rises above MINOR. Two MINORs are worth a row
each: the "from above" half of step-import's new both-bounds read is
pinned by nothing (D1), and its "below" is hulled with a reading from
above for a cone pair (D2).

## What I executed, and what I only read

EXECUTED (probes under `probes/delta/` on this branch):

- `lever-doors.sh` on the head with `GATE_BASE=3900eb46` (47 pins, judged
  against the base's 59; green), its selftest, its census, and the first
  review's M1 plant re-spelled with the doors' current names.
- 34 mutants re-planted **on the frozen head** from one build
  (`delta/mutants.py`), each over the whole fast set of its crate, lib
  and integration: geom-brep 2 482 tests, step-import 424. The fast set
  only: step-import's slow set at opt-level 0 runs past ten minutes a
  mutant here and I stopped it.
- The new step-import rows and a fold sweep through `choose`
  (`review_delta_fold_sweep`, in `delta/head-with-mutants-and-probes.diff`).
- The differential: section "Differential" says exactly what ran.

ONLY READ: the PR's own diff against `3900eb46` (all 23 files); the
delta's combined merge diff for `cylinders.rs`, `props/curved.rs` and
`reach.rs`, and `build.rs` as it stands; the three rows; `topo`'s
`section_near` and `section_closed`. Not executed: a whole import of
anything but the PR's own row; the FreeCAD STEP job; clippy, rustdoc and
the other gates (CI ran them).

### The CI run, at step level

Run 45061123383, head `c65b9292`, 14 jobs. Ran green: `change filter`;
`lint` (tracker lint, self-tests, ruff, actionlint, **gates**, rustfmt,
clippy `--all-features`, rustdoc); `test (kernel)` and `test (app)`
(build; default ε; ε 1e-6 and 1e-12 for the touched ε-sensitive crates;
"the slow-set tests this diff adds or edits"; doc-tests; slow-set name
check); `tools`; `mesh budget`; `corrupt input`; `gate ok`. Skipped, as
on the first head: `step import (freecad)`, `interval`, demos, viewer
app, python suite, cache prime, and the slow set as a whole.

## The adjudicated findings

**F1 (my M1). Closed.** DEMONSTRATED, `sure`. `generator_lever` and
`of_crossing_cylinders` carry the base's names, are on `DOORS`, one pin
each. The gate reads 47 against the base's 59, with seven pin lines
removed carrying twelve references (3 + 1 + 1 + 3 + 1 + 2 + 1). The two
plants that were green on the first head are red on this one
(`ROSE generator_lever … 0 -> 1`, `ROSE of_crossing_cylinders … 0 -> 1`).
The header and the row say what each takes.

**F2. The fix does what it claims at the three consumers it names; the
"from above" half is pinned by nothing.** See D1 and D2 below.

Each consumer, on each sign:

| Consumer | Bound | Acts on | Right? |
|---|---|---|---|
| `read_section` (analytic and spline pairs) → `topo::section_near` | below | builds on a definite crossing | yes, and it is the callee's stated contract (`section_near`: "its extent from below"). Inside, `section_closed` takes its own bound from above for the rows that act on a Zero (`edge_extent_sup` of the seed), so the import's `above` never reaches them (D3) |
| the `step_planes_cross` gap in the `Touching` refusal | below | a refusal's number | yes: the same reading the verdict was decided at |
| `coaxial_tangent_circle` | above | builds a circle on its axis rows' Zero | yes |
| `tangent_locus` (the other tangent arm) | neither: the edge's region ball | builds on a Zero at the run's band | unchanged by this PR |

`edges_extent_inf` is a bound from below of the faces' run: each width
is the difference of two sampled boundary points' coordinates along a
unit direction, so no more than their distance, and the samples are
points of the faces' own boundaries. It takes carriers and spans, the
hand-built-geometry class the gate disowns, and no number; it is not on
the gate's list and need not be. Measured on the folded discs: below
3.99990 m, above 4.47205 m, true 3.99990 m. The crease now refuses as one
plane up to a fold of 2.4e-7, is in the band to 2.4e-6, and builds from
2.6e-6; nothing builds where the faces stand under the file's 1e-6 apart.

The disclosed window (`work/importgate/an-imported-crease-…`): the row is
accurate about what happens and says the cost is the refusal's wording.
**It does not say whether any import that built before this unit now
refuses, which was the question.** By inspection the answer is no for the
window itself: certification is untouched by this PR, so a crease that
certification refuses on the head was refused there on the base too
wherever the base's `choose` built it, and refused earlier, as parallel
planes, wherever it did not. But see D4: the lever from below can be
shorter than the base's, which is the other way an import that built
could come to refuse. `likely`.

**F3. Closed.** DEMONSTRATED, `sure`. The three survivors (`r11` chart
read at the world origin, `r12` reach doubled, `r13` refusal read as 1e9)
are each red on the head, on the new rows.

**F4. Closed, with one stale phrase.** The row's body and dated section
say 47; the header's lists name the new doors. The dated section still
writes `classify_seam`'s lane as `Option<NurbsLane<T>>`, which the merge
made `Option<impl NurbsLaneOf<T>>`. NOTE, `sure`.

**F5. Closed.** DEMONSTRATED, `sure`. The dead term and its comment are
gone. The no-region refusal names the entity at fault (mutant `s7`, the
asker named: red). The Face-arm row holds the turn, the shift's sign and
the hinge (`r4`, `r5`, `r6`: red).

**The n7 row** (`the-per-datum-carrier-ladder-claims-distinct-…`) says
what the code does: the ladder levers at `at_consumed_extent`'s arm, a
ball's reach, and acts on every sign. Its fix paragraph is a proposal,
fairly marked as one. `sure`.

**F6, S4 argued down.** The argument holds for the value: poison is what
an unread arm should be. It does not answer the spelling, and "until
then" is a deferral with no row: nothing under `work/` schedules a
rim-arms struct with no level arm for a cone (the pinned-sites row
records the correction, and its pin is gone). Per the style lane's Q6, a
disclosed deferral owes a schedule. Style, `likely`.

## New findings in the delta

### MINOR

**D1. The "from above" half of `faces_run` is pinned by nothing.**
DEMONSTRATED over step-import's fast set (424 tests), `sure`; not run
against its slow set.

Four mutants survive: `coaxial_tangent_circle` handed `extent.below()`
(`s4`); `faces_run` returning its below for both (`s9`); the above read
without the cone apexes' reach (`s6`); and the spline pair's section
handed `extent.above()` (`s2`). The other six of that family are red
(the analytic section or the planes' gap handed above; below made the
box diagonal; below over one face; below without the apexes; the asker
named). So F2's claim "`coaxial_tangent_circle` reads above" is true of
the code and held by no row, and neither is the spline pair's "below".
This is the guard the dispatch asked about (a consumer handed the bound
its verdict does not act on): for two of the four consumers the suite
would not notice.

**D2. `below` is hulled with a reading from above for a cone pair.**
BY-INSPECTION, `likely`.

`build.rs:575-577`: `below: from_apexes(run.below())`, where
`from_apexes` takes the larger of the lever and
`region.lever_from(apex)`, the far distance of the edge's *cover* from
the apex. That is exact on a line or a circle arc and a bound from above
on an ellipse, a spiric or a spline (`extent.rs`'s `cover_is_exact`,
whose `edge_reach_bounds` exists to give this read from both sides). So
on a cone pair whose edge is not a line or a circle, the bound named
`below`, handed to the readings that build on a definite sign, can pass
the edge's true reach from the apex. The fold predates this PR (the base
has it, and mutant `s5` shows seven rows need it); what is new is that it
now sits under the name `below`.

### NOTE

**D3. The import's `above` reaches one consumer; the Zero-acting pose
rows inside the section read another bound.** BY-INSPECTION, `likely`;
inherited. `topo::section_closed` takes `extent` as "the old edge's
extent from below" and builds its own `LeverBounds::Between { below:
extent, above: edge_extent_sup(seed) }` for the cone rows, and reads
`edge_extent_sup(seed)` for `pt_axis_in_plane`. The import hands it the
*faces'* run from below, so that pair can be inverted (below 4 m, above
the seed edge's 2 cm, or 0 for two planes' canonical seed). Reading both
and acting where they agree stays conservative whichever is larger, and
the import handed a faces' reach here before this PR too. But it is two
both-bounds reads of two different regions stacked, and the one the fix
built stops at the door.

**D4. The lever from below moved more readings than the first head
did, and the question about imports that built before is not answered
in the row.** DEMONSTRATED that they moved, `likely` on the rest.
`offset_section_planes_parallel`'s margin differs from the base's in 158
tests at ε = 1e-9 (61 on the first head). No verdict moves in the suite.
`edges_extent_inf` is at most the farthest pair of sampled boundary
points, and the base's vertices' box diagonal can pass that by up to √3,
so outside the suite a crease that built on the base with its reading
just past the band can now read in it and refuse. That is the direction
the README asks for; neither the PR's claim nor the importgate row says
it.

**D5. Inherited reds, and what their rows name.** DEMONSTRATED.
`sweep::all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`
is red at 1e-12 on the base and the head with the same message; **its
row (`work/nearpoint/a-cylinder-circle-…`) does not name it** (no match
for the test's name in the file). `geom-brep …::a_carrier_on_its_chart_pinned_where_the_frame_cost_refuses`
passes on both sides under my seed; under mine the sibling
`every_arm_and_term_is_load_bearing_in_a_pinned_sweep` is red on both.
Its row names the test, as a mutant-coverage matter and not as red under
a foreign seed. Also red on both sides here at 1e-9, same messages: the
ten slow-set rows of my first report's n8 less the two editor-core ones
(not in this scope), plus `sweep::all bisector_nodes::the_near_node_poses_at_the_witness_band`
and `section_end_poses::a_section_end_at_the_band_edge_…` (both red on
the base binary with no seed and no ε set), and three source-scan rows
I did not separate from my logger patch.

**D6. The dated row section's `Option<NurbsLane<T>>`** is
`Option<impl NurbsLaneOf<T>>` since the merge (F4). `sure`.

### Style

- **`edges_extent_inf` is a second spelling of a point set's diameter**
  (Q1). `extent.rs:579-644` reads thirteen widths where `farthest_pair`,
  forty lines up, is documented as "the one spelling of a point set's
  diameter" and is what `extent_inf`'s own sampled pairs use. I assume
  the widths are there for cost (pairs are quadratic in the samples, and
  the symbolic tier pays per margin); the doc does not say. `likely`.
- **Its doc understates its own bound** (Q2). "Some one of them stands
  within 36° of any direction, so … at least 0.8": over 400 000 random
  directions the worst is 27.6°, cosine 0.886. Safe, and not what the
  code does. `sure` on the number.
- **S4's deferral has no row** (Q6, above). `likely`.
- **`Occupant`** is a second ball-shaped pair of (centre, radius) beside
  `ExtentBall` in one module (Q1). It is the right trade here: it cannot
  be levered. `unsure` it needs anything.
- **The crease row's new third assertion** (the 2.4e-7 fold decided at
  the run from below) is the row that turns `s1` and `s8` red: a good
  pin, and the reason D1 stands out, since nothing like it exists for the
  other half.

## The merge of main

Read: the four hand-resolved files. `cylinders.rs` and `reach.rs` are
mechanical (the placement takes an `Occupant`; `solid.surface_of(face)`
for the face record). `props/curved.rs` re-threads `surface` into
`cone_boundary` under main's `Frame`. `build.rs` I read as it stands, not
hunk by hunk. The seam's lane as `Option<impl NurbsLaneOf<T>>` with
`chart_arm` on the sealed trait and `match self {}` at `NoLane` is the
shape main's sealing asks for; `vtxfac.rs` keeps the `NoLane` refusal
(`nurbs_lane().ok_or(NoLane)`).

Post-merge gate fixes (`c65b9292`): **no allowlist entry was added.**
`git diff 3900eb46 c65b9292 -- scripts tools .github` touches the three
`lever-doors` files only. The width directions are built in `T`, and
`impl From<ExtentBall> for Occupant` became `Occupant::ball`. `sure`.

## Differential

Re-taken with my logger (`klog.patch.py`), which tallies each decision by
(scalar, predicate, outcome) per test **and sums a hash of its margin**,
so a moved value with an unmoved verdict is visible, which the
implementer's logger states it is not. Base `3900eb46` against head
`c65b9292`, `ci.yml`'s build profile, `CAD_FUZZ_SEED=0x5eed0000c0ffee01`.

Scope, narrower than my first review's: the kernel packages geom-brep,
step-import, topo, sweep, mesh, verbs, step-export and stl, **the slow
set included**; not editor-core, pncad, the viewer or the tools.

ε = 1e-9, 12 190 tests on the base and 12 205 on the head (the 15 new
rows):

| scalar | decisions, base | decisions, head |
|---|---|---|
| `f64` | 3 510 607 247 | 3 510 606 751 |
| `Interval` | 31 090 985 | 31 090 985 |
| `Sym<f64>` / `Sym<Interval>` | 135 350 / 614 611 | the same |
| `Dual<f64>` | 232 641 | 232 641 |

- 10 342 tests log a decision on both sides; every count agrees in
  10 337. The other five are topo's unseeded generators
  (`seqgen::random_op_sequences`, four of `validate::tests`), each of
  which I re-ran twice on the head binary: all five differ between the
  two runs. They carry the whole `f64` difference.
- 186 tests have equal counts and a moved margin:
  `offset_section_planes_parallel` (158), `carrier_cyl_reach` (16), the
  cone-coaxial and tangent readings (`coc_*` 11, `pt_axis_in_plane` 11,
  `kos_*` 6), `carrier_cone_*` (3), a few `ssi_*`. No verdict among them
  moves.
- Outcomes: the same 13 reds on both sides with the same messages (D5),
  and every new row passes on the head.

ε = 1e-12, a **targeted** row, not the suite: the 186 tests above, all of
geom-brep and step-import, and the inherited red: 3 408 tests on the
base, 3 423 on the head. `f64` 493 917 199 decisions and `Interval`
2 027 024 on **both** sides; of 2 602 tests that log on both, **every
count agrees**; 187 have a moved margin; the same six reds on both.

So the implementer's conclusion holds on my instrument, with one
addition it could not see (D4). My count of noisy tests differs from
theirs (5 and 0 against 11 and 9) because my scope leaves out the
`profile` crate's unseeded tests and their thread-timed ones did not
differ here. Limits: the 1e-12 row cannot see a test that reaches a
moved lever at 1e-12 and moved no margin at 1e-9, outside those two
crates; the log is keyed by binary stem and test name (two source-scan
keys collide); it sees only decisions through `k_stats::classify`.

Mutants, re-planted on the frozen head (`probes/delta/mutants.log`): 34,
of which 30 red and 4 surviving (D1). Red: all 17 of the first review's
PR list that still apply, the three former survivors at
`chart_curvature_arm`, the three Face-arm ones, three on
`edges_extent_inf` (the edges' own extents alone; one width for the
widest; the width scaled past the farthest pair), and six of ten on
step-import's bounds.

## Is anything in the delta a fresh instance of what it fixes?

- **A pin taken off by a move out of the gate's sight: no.** The two
  renamed doors are back under their names and pinned, and the gate is
  red on a made-up call to either. `Occupant::point` stands where
  `ExtentBall::point` stood three times, and it is not the same move: an
  `Occupant` is private to the pair-section module and has no lever, so
  nothing placed through it can become one. `edges_extent_inf` is a new
  public door that takes carriers and spans, no number.
- **A consumer handed one bound whose verdict acts on the other sign:
  none found** at the three the fix names. Two things near it: D2 (the
  bound named `below` carries a reading from above on a cone pair) and D1
  (for two consumers no row would notice the wrong one).
- **A second spelling of a both-bounds read: yes, mildly.** D3: the
  import's `LeverBounds` and `section_closed`'s own, over different
  regions, stacked. And one more second spelling, of a point set's
  diameter (style).

## Ratings (the delta)

- **Idiom and structure: 4/5.** Doors hand back what they read, one
  placement path, one lane method; `below` on a cone pair is not quite
  what its name says.
- **Test quality: 4/5.** Every mutant from the first round that survived
  is red, and the crease row now pins the deciding bound; the other half
  of the same read has no row.
- **Doc and comment honesty: 4/5.** The row files what the fix could not
  close and measures it; it stops short of the one question asked of it,
  and one inherited red's row does not name its test.
