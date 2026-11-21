# Review — PR 7219, `oneround/lever-sites-brep`, frozen at `37da28e8`

Unit row: `work/oneround/the-lever-sites-that-hold-their-geometry-read-it-outside-topo.md`.
Merge base with `main`: `b5c3fb4b`. Hosted CI: run 44848429486.

## Verdict: APPROVE-WITH-FIXES

The arithmetic is right and nothing in the suite moves (C3, measured base
against head). What has to be fixed before the pin count is believed:

- **M1**: two of the fourteen pins fell because a door was renamed out of
  the gate's list, not because the site reads geometry. The gate is green
  on production calls that hand both doors made-up numbers.

Everything else is MINOR or NOTE.

## What I executed, and what I only read

EXECUTED (this container, 4 cores; every probe is under
`analysis/oneround/lever-sites-brep/probes/` on this branch):

- `lever-doors.sh` on the head with `GATE_BASE=b5c3fb4b` (45 pins, green),
  on the merge base (59), its `--selftest` (green), and its census
  (`LINES_FOR=ALL`) on both.
- A plant of four production functions that hand the new doors made-up
  numbers and points (`gate-plant.rs.txt`), type-checked with
  `cargo check -p geom-brep --tests`, with the gate run over it, and a
  control plant of an old door name.
- The PR's ten new rows on the pristine head (10 pass).
- 35 mutants switched at run time from one build (`mutants.py`): the PR's
  17 and 18 of mine, each over the crate's whole lib suite (geom-brep
  1 459 tests, step-import 140); the nine that survived geom-brep's lib
  suite again over geom-brep's and topo's whole fast sets (6 444 tests);
  one more over step-import's whole fast set (396).
- Three step-import probes through the private `choose` (the crease
  against its closed form, a sweep of the fold through the file's band,
  the no-region arm), and one door-level probe of `edge_extent_inf`
  against nine samples on a circle arc past a half turn.
- The differential (C3): section C3 says exactly what ran.
- `python3 scripts/work.py lint` on the head (ok).

ONLY READ: the whole diff; `crates/geom-brep/src/extent.rs` end to end
(the non-test part; its test modules by outline and the new one in full);
the README's "Float rounding"; both rows; the gate, its helper and counts;
`topo::wedge_reach`'s module docs; the PR body. Not executed: the sweep
row `open_arcs_approaching_a_full_turn_do_not_collapse` under the
`edge_extent_inf` conversion (I checked the door's numbers below that row
instead), the FreeCAD STEP job, a whole import of the new step-import
fixture, clippy and rustdoc (CI ran them).

### The CI run, at step level

Run 44848429486, head `37da28e8`, 14 jobs.

- **Ran, green**: `change filter`; `lint (fmt, clippy, gates)` (work
  tracker lint, three self-tests, ruff, actionlint, **gates
  (scripts/gates)**, rustfmt, clippy `--all-features`, rustdoc; its
  `change filter self-test` step was skipped); `test (kernel)` and
  `test (app)` (build; default ε; **ε 1e-6 and 1e-12 for the ε-sensitive
  crates and files the diff touches**; "the slow-set tests this diff adds
  or edits"; doc-tests; slow-set name check); `tools`; `mesh budget meter
  + certificate falsifier`; `corrupt input (release profile)`; `gate ok`.
- **Skipped**: `prime the build cache`; **`step import (freecad)`**;
  **`interval`**; `demos (tour + wild)`; `viewer app feature`; `python
  suite`; and in both test jobs the step "the slow set did not run (an
  all-tier run that seeds no crate)" is the skipped *notice*, with the
  slow set itself run only for tests the diff adds or edits.

So hosted CI did not run the slow set, the `interval` job or the FreeCAD
STEP check on this head. The diff changes step-import's reading at 22% of
its suite reads, and the STEP job was not selected.

## Findings

### MAJOR

**M1. Two pins fell by a rename (C1).** DEMONSTRATED, `sure`.

`Reach::cone_generator_lever(axis: Line<T>, half_angle: T)` replaces
`generator_lever(apex, a, cos_a)`, and
`ExtentBall::of_cylinders_crossing([l1, l2], [r1, r2], &reach)` replaces
`of_crossing_cylinders((foot, normal), sin, gap, [r1, r2])`
(`crates/geom-brep/src/extent.rs:1065`, `:376`). The old names were on
`lever_doors.py`'s `DOORS` under the gate's own heading, "an extent
widened, scaled or measured by the caller's number". The new ones are not
on it. What the caller hands is still a point, a direction and a number
(the cone), or two lines and two numbers (the cylinders): the lever is
`window / cos(half_angle)`, the ball's radius is built from
`(r1 + r2)/sin` and `min r`. Nothing ties those numbers to a carrier.

Planted in production code (`ssi.rs`) and type-checked:

```rust
reach.cone_generator_lever(Line { origin: (1e3, 0, 0), dir: z }, 1.5707)
ExtentBall::of_cylinders_crossing(lines, [1.0e6, 2.0e6], reach)
```

`GATE_BASE=b5c3fb4b bash scripts/gates/lever-doors.sh` stays green at 45
pins. The same plant spelled with an old door (`Lever::curvature_radius`)
goes red (`ROSE curvature_radius crates/geom-brep/src/ssi.rs: 0 -> 1`).

What did improve at these two sites is real and worth keeping: the
derived quantities (the cosine; the perpendicular's foot, direction, sine
and length) are now computed once, inside the door, at the reach's own
pivots. But that is single-sourcing arithmetic. The author's question was
whether these two sites should stay pinned. My reading: yes. A door that
takes a carrier's axis and sizes as loose arguments is the pinned shape
under a new name. The honest count is 59 → 47 with
`cone_generator_lever` and `of_cylinders_crossing` on `DOORS` at one pin
each, until a door exists that takes the pose (as `ruled_pivot_lever`
takes a `PlaneCylinder`) or the cover. The PR body and the dated row
section disclose the loose arguments in prose; the gate and its header do
not.

The other new doors, door by door:

| Door | Takes a number or point the caller could make up? | On the list? | Gate red on a made-up call? | Reading |
|---|---|---|---|---|
| `ExtentBall::lever_from_either(p, q)` | two points | no | no (planted, green) | a conversion: the caller's *number* is gone. It is the class the header already disowns ("a caller's point: `lever_from(p)`"), but neither the header nor the row's "A caller's point" names it (m4) |
| `Reach::ruled_pivot_lever(&PlaneCylinder)` | a pose with crate-public fields (the new row builds one by literal) | no | no (planted, green) | a conversion; the header's "geometry built by hand" class, which does not name `PlaneCylinder` |
| `Reach::cone_generator_lever(axis, half_angle)` | **yes: a point, a direction, a number** | **no** | **no** | **not a conversion (M1)** |
| `ExtentBall::of_cylinders_crossing(lines, radii, &reach)` | **yes: two lines, two numbers** | **no** | **no** | **not a conversion (M1)** |
| `Reach::common_perpendicular(l1, l2)` | two lines; returns no lever and no ball | n/a | n/a | not a lever door |
| `geom_brep::cover_diameter(&cover)` | a cover (public constructors: the disowned class) | no | n/a | a conversion |
| `chart_curvature_arm(lane, chart, p, reach)` | a point, and a `Lever` (already sourced) | home | n/a | a conversion; `pub`, so any crate can read any chart at any point |
| `point_placement(p, region, band)` | a point; builds no ball and no lever | n/a | n/a | a conversion: the three `ExtentBall::point` sites never levered anything |

### MINOR

**m1. `step_import::build::choose` now reads the bound the README assigns
to the other kind of verdict (C4).** BY-INSPECTION, with a probe that
found no wrong build; `likely`.

The lever is `Reach::of(solid, [plus, minus]).lever()`, the cover's box
diagonal, "from above" in the code's own comment (`build.rs:505`). The
plane×plane verdict it feeds **builds on its definite sign** (the crease)
and **refuses on its Zero** (`Touching`). The README: "a verdict that
claims a definite sign reads the extent from below". `extent.rs`'s module
docs say the same about this exact shape ("On a two-sided row whose
definite side is a SERVED class … it decides a reading that is in the band
at the consumed extent"). And the reading this site was written to match,
`topo::wedge_reach` (the deleted comment named it), says of itself: "The
measure never over-states the faces' boundaries, the unsafe direction for
a lever". The tangent and coaxial arms fed by the same `extent` act on
their Zero, where from above is right; one lever cannot serve both, which
is what `LeverBounds` is for.

What I measured on the PR's fixture (`review_c4_fold_sweep`): the faces'
true greatest separation is `2·fold`; the lever is 4.47. No fold builds
where the faces stand under the file's 1e-6 apart: folds from 2.3e-7 to
2e-6 refuse as in the band, and 1e-5 builds. The band's width (the build
needs a reading somewhere past 9e-6) covers the 2.24× over-statement here.
The suite's own reads go up to 83× (PR body), which it does not cover. I
could not build a fixture that shows a wrong crease, so this stays
BY-INSPECTION.

"Toward the truth" is true of the one fixture and is not evidence for
"from above": the faces' true run there is 4 m, and any reading from below
that sees past the two vertices builds the same crease.

Also unverified by anyone: the new row calls `choose` alone. The built
body's own wedge reading at rest (`topo::wedge_reach`, the vertices from
the edge's middle: 0.01 m on this fixture) levers the same two faces 450
times shorter than the import now does. Nobody has shown the import of
such a file completes. `unsure`.

**m2. `chart_curvature_arm`'s point, reach and refusal arm are pinned by
nothing (C9).** DEMONSTRATED, `sure`.

Three mutants survive geom-brep's and topo's whole fast sets (6 444
tests): the chart read at the world origin in place of `p`; the reach
doubled; a lane's refusal read as an arm of 1e9 in place of poison
(`r11`, `r12`, `r13` in `survivors.geom-brep+topo.log`). The new row
reads at `ORIGIN` with a reach of 1, so it cannot tell. The PR's own
mutant (a lane-less arm read as the reach) is red. The claim that the
same-number conversions are "pinned against the wrong pivot and the wrong
bound" does not hold for this door's pivot.

**m3. The dated section of the pinned-sites row points at the wrong two
sites (C11).** DEMONSTRATED (read), `sure`.

`work/oneround/lever-doors-pinned-sites-await-a-geometry-read.md`: the
paragraph after the converted list opens "The last two take the carriers'
axes and sizes as the lane's pose holds them". The last two bullets of
that list are `step_import::reach::Reach::lever` and
`step_import::build::faces_reach`. The sentence is about
`intersect::pair_bound` and `cylinders::decide_stance`, the fourth and
fifth. As written the row says step-import takes a pose.

Two smaller ones in the same section. It and the PR body's table put
`of_cylinders_crossing` at `cylinders::decide_stance`; on the head the
call is in the pose's constructor (`cylinders.rs:281`), and
`decide_stance` calls `common_perpendicular`. And the row's body above
the dated section still says 59 pins and still lists every converted site
and the `classify_seam` wrapper as standing.

**m4. The gate's header and the row's "Doors the pins do not see" were not
brought up to the doors this PR adds (C10, C11).** DEMONSTRATED (read),
`sure`.

`lever-doors.sh`'s "WHAT IT DOES NOT HOLD" lists the caller's-point doors
by name (`lever_from(p)`, `curvature_lever_arm(cone, p)`,
`parallel_curvature_arm(s, p)`) and the hand-buildable types by name
(`Surface`, `Frame`, `Curve3`, a conic, `Cover`). This PR adds
`lever_from_either(p, q)` and `chart_curvature_arm(.., p, ..)` to the
first class and `PlaneCylinder` to the second, and (M1) two doors that
take loose numbers. None is in the header or in the row's list. The edit
the PR does make to the header (dropping four names) is correct as far as
it goes, and `hinge_lever` and `turn_lever`, now private, are rightly
still listed.

### NOTE

**n1. A premise of the brief is wrong: this branch was not created from
the frozen head.** DEMONSTRATED, `sure`. `oneround/lever-sites-brep-review`
stood at `c97711be`, a later `main`, of which `37da28e8` is not an
ancestor. I reviewed in detached worktrees at `37da28e8` and `b5c3fb4b`;
this branch carries the report and probes over that later `main`, and the
probes apply to `37da28e8`.

**n2. The PR body's "10 pin lines dropped" is 9** (`git diff --stat`:
`lever-doors.counts | 9 -`; the nine lines carry the 14 references).
DEMONSTRATED, `sure`. The count 59 → 45 is right, nothing was added to a
pin, and no pin rose against the base.

**n3. `edge_extent_inf(..).max(faces.lever())` has a dead term, and the
comment beside it describes a case that cannot occur** (`build.rs:505`:
"and the edge's own extent where that is longer"). DEMONSTRATED (mutant
`i3`: the edge term dropped, step-import's 396 fast tests all pass) and by
argument: the edge bounds both faces, so its cover is inside the region's
and no two of its points stand farther apart than the region's box
diagonal. `sure`. The term was live before (a closed edge with one
vertex).

**n4. The no-region refusal is typed, and names the wrong entity.**
DEMONSTRATED (`review_c4_a_pair_with_no_region`), `sure` on the fact,
`unsure` on whether it matters. With a sibling edge's interval made
non-finite, edge 1 (its own carrier and interval sound) answers
`StepImportError::Topology { id: 1, what: "an edge whose faces have no
region to read their reach from" }`. It is the same type its neighbour
refusal uses, so it is typed consistently. But the defect is edge 2's,
which would refuse on its own turn; which entity the file's reader is
told about now depends on the order edges are read. On the base, edge 1
built. No row reaches this arm; the probe is there to keep.

**n5. `ruled_pivot_lever`'s `Face` arm is not reached by the new row.**
DEMONSTRATED, `sure`. Three mutants (the turn dropped, the shift's sign,
the hinge left at the foot) survive geom-brep's lib suite: the new row
reads a `Region`, whose turn is zero and whose shift enters as `|shift|`.
All three go red in `geom-brep::all section_reads_at_the_reach` and two
topo rows, which predate this PR. Pinned, by older rows.

**n6. On the conversions' own arithmetic, the new rows are the only
pins in geom-brep's lib suite.** Every PR mutant of
`cone_generator_lever`, `of_cylinders_crossing`, `common_perpendicular`,
`cover_diameter` and `stands` is red on exactly one test, the new row.
That is a point for the PR.

**n7. The per-datum cone ladder claims `Distinct` on an arm read from
above.** BY-INSPECTION, `likely`; pre-existing, and this PR makes it
less so (see C2).

## The claims

**C1.** Falsified in part: M1 (two doors), with the table above. The
other pins fell for the reason claimed.

**C2. `lever_from_either` is sound; a shorter arm is sound at each verdict
it feeds.** Holds. BY-INSPECTION, `sure` for the cones, `likely` for the
cylinders (my derivation, which agrees with the doc's).

| Verdict | How it is acted on | Bound the README assigns | Shorter arm |
|---|---|---|---|
| `coincident_over_extent_as` on `reading`'s sum (`carrier_cone_reach`, `carrier_cyl_reach`) | on its **Zero** only (the coincidence stands); every other outcome is the straddle refusal | from above | sound. A point of cone *i* in the ball stands `s ≤ reach(pᵢ)` from its own apex, and its partner on the other cone, at the same `s` from the other apex, is within `\|p₁ − p₂\| + s·(tilt + Δα)`. For cylinders the offset is read at `pivot` on the second axis; a point `X` of the second in the ball moves by at most `apart + tilt·\|X − pivot\| + Δr`, a point of the first by the same with `q = first.foot(pivot)`. So the larger of the two reaches bounds both. The old arm, `reach(p) + \|p − q\|`, is never shorter than `reach(q)`, so the new one is never longer, and is still a bound from above because the ball's reach is. |
| `ladder_reading` → `cone_data` → `data_rungs`, levered at `at_consumed_extent`'s arm (and `topo::carrier_pair_arm`, read by `editor_core::coincide::replay`) | on **every** sign: a datum decided apart is `Distinct`, all Zero goes on to the sum | both, acting where they agree | no new unsoundness. On the Zero side, as above. On the definite side the arm was already from above (n7), and a shorter one over-claims `Distinct` less than the old did. I did not trace `replay`'s use: `unsure` there. |

The 21 reads "one ulp longer" are `max` against `+ 0` after `foot`'s
rounding; nothing to it.

One thing the docs do not say: `Reach::lever_between` in the same file
folds two feet's levers with **min** for two lines' tilt, and
`lever_from_either` folds two pivots' reaches with **max**. Both can be
right (one bounds a section's travel from either foot, one bounds a
displacement from both sides), but neither doc names the other (style S3).

**C3. No verdict moves.** <<C3>>

**C4.** The build is exact: the 1e-5 fold's carrier is the line through
`(0.9999499987499375, 0, 0)` along `(0, 1, 0)`, which is `x = c₀, z = 0`
to the bit (`review_c4_the_built_crease_is_the_closed_form_line`). The
base's arithmetic refuses it (mutant `i1`, the edge's own extent alone,
which equals the vertices' box on this fixture: red on the new row). One
face's region for both is red (`i2`). "Longer in 1 241 of 5 511, to 83×"
I did not re-count. The no-region refusal: n4. Which bound: m1.

**C5.** Holds, BY-INSPECTION, `sure`. `classify_seam(.., T::nurbs_lane(), ..)`
then `lane.and_then(|l| l.chart_arm(chart, p, reach.length()).ok())` is
the old `T::spline_chart_arm(chart, p, reach).ok()` on every impl: `f64`,
`Probe`, `Interval` and `Sym<T>` answer `Some(NurbsLane::certified())`
where their `spline_chart_arm` calls `NurbsLane::certified().chart_arm`,
and `Dual<T>` answers `None` where its `spline_chart_arm` answers
`NoLane`. The differential agrees (C3). See style S1 for the two methods.

**C6.** The value is unread: with the session box in place of poison,
6 444 tests pass (mutant `r17`), and the two readers of `RimArms::level`
(`curved.rs:1345`, `:1545`) match `RimLevel::Unit` only, where a cone's
rims are minted `Length` (`:3529`). Something reads it only if a
cone rim becomes `Unit`. Poison is the right value for a field nothing may
read, since a reader that appears escalates. But my answer to the
question is that the field should not exist for a cone: see style S4.

**C7.** Holds, BY-INSPECTION, `sure`: `placement(ExtentBall::point(p), ..)`
and `point_placement(p, ..)` both reach `stands` with a radius of
`T::zero()`, so the two margins are the same operations on the same
values, and neither is recorded. The differential agrees. Pinned: `g1`,
`g2` red on the new row; the three call sites red in
`geom-brep::all cylinder_pair_section` (`r15`, `r16`).

**C8.** The numbers hold. Door level, unit circle
(`review_c8_a_circle_arc_past_a_half_turn`):

| span | `edge_extent_inf` | nine samples | `edge_extent_sup` |
|---|---|---|---|
| 4.2 (240.6°) | 1.7264 | 2.0000 | 2.0000 |
| 4·atan 2 (253.7°) | **1.6000** | 1.9919 | 2.0000 |
| 5.0 | 1.8011 | 1.9999 | 2.0000 |
| 5.8 | 1.9710 | 1.9854 | 2.0000 |

So the door is as short as `1.6 r` at `tan(Δ/4) = 2`, the row's `1.726 r`
at θ = 4.2 is under its `1.9 r` floor, and `edge_extent_sup`'s circle arm
is the diameter. `inf ≤ nine ≤ sup` at every span, so the site's own
reading is honest: each sample is a point of the edge, it is a bound from
below, and it is the tighter of the two. The row's band is right: past a
half turn the arc's diameter is `2r`, and `1.9 r` is a floor a sound
reading from below can meet. The filed row says what the code does; P2
fits (sound, not tight). I did not run the sweep row under the
conversion. On `sweep_ball`: agreed, it is not a lever. Its ball is read
by the ruling sweep through `center()` and `radius()` only
(`ruling.rs:885`, `:1097`), as a region to place points in, and the pad is
a clearance. Whether an `ExtentBall` is the right type for a clearance
region is the open question, and it is fairly left.

**C9.** Every mutant the author lists is red (17 of 17; table in
`mutants.geom-brep-lib.log`). The same-number conversions are pinned
against the wrong pivot and bound, except `chart_curvature_arm` (m2).
`cone_boundary`'s arm is unpinnable as claimed (`r17`).

**C10.** Holds apart from M1 and m4: 59 → 45, nine lines for fourteen
references, nothing added, no pin above the base's, selftest green and
still covering number-taking doors (a bare radius, a `Measured` literal,
`lever_from_point_and_plane`, a curvature arm in the extent home).

**C11.** The filed row is true (C8). The dated section: m3. Its
corrections to the row are true: `cone_boundary`'s lever was never read
(C6); `seam_face_reading` needs the scalar's lane.

**C12.** True of the frozen head except n2, and with M1's caveat on "14
converted". CI is as the body says (step-level above). <<C12>>

## Ratings

- **Idiom and structure: 3/5.** The doors compute their own derived
  quantities now, which is the right direction; but two of them are the
  old shape renamed, `pair_bound` carries one datum twice, and the trait
  gained a second method for one fact.
- **Test quality: 3/5.** Every listed mutant is red and the new rows are
  the only pins on five doors' arithmetic; against that, the chart arm's
  pivot and reach are pinned by nothing, and the step-import row pins the
  cover's 12% slack as if it were the contract.
- **Doc and comment honesty: 4/5.** The PR body says what was not done,
  what was reverted and why, and raises M1's question itself; it loses a
  point for the row's misplaced "last two", the pin-line count, and a
  code comment describing a case that cannot occur.

## Style

Questions exercised: Q1 (prose sweep and a data sweep over the touched
files), Q2, Q3 (by mutants), Q4 (a grep for prose citing the old
premises), Q5 (the curvature home's header), Q6, Q7, Q8 (`extent.rs`, the
file this PR changes most; **not** the largest file touched, which is
`topo/src/props.rs`, of which I read only the five impls).

On the lane's warning that a fix mints the defect it closes: it did,
three times (S1, S2, and M1 itself, where the fix for "a caller's number
at a named door" is a new door that takes the caller's numbers).

- **S1 (Q1). Two trait methods for one fact.**
  `crates/topo/src/props.rs:6868` (`spline_chart_arm`) and `:6879`
  (`nurbs_lane`), five impls of each. `spline_chart_arm` is now
  `nurbs_lane().ok_or(NoLane)?.chart_arm(..)` on every impl and has one
  caller left (`boolean/vtxfac.rs:870`). The new doc comment reconciles
  them by hand ("as `Self::spline_chart_arm` answers `NoLane`"). An impl
  can now answer one and not the other. `sure`.
- **S2 (Q1, Q2). `holds` beside `against`, `point_placement` beside
  `placement`.** `cylinders.rs:876-886`: the new closure repeats the
  `pose.placed` match of the one above it, under the comment "likewise".
  The gate's objection was to the *name* `ExtentBall::point`; the fix is a
  parallel code path for a ball of no radius. `sure` that it is a copy,
  `unsure` that it is worse than what it replaced.
- **S3 (Q1). `lever_from_either` (max of two pivots) and
  `Reach::lever_between` (min of two feet)** live in one file, answer
  nearly the same question, fold in opposite directions, and do not
  mention each other (`extent.rs:243`, `:1162`). `unsure`: both may be
  right, and a reader cannot tell from the page.
- **S4 (Q7, Q2). `cone_boundary` takes the whole cone to compute a
  constant.** `curved.rs:3476`, `:3553`: a new `cone: &Surface<T>`
  parameter, beside the apex, axis, sine and cosine already destructured
  from it, exists so that `uniform_curvature_arm(cone)` can return what
  its doc says it returns for a cone, poison. It reads as a spelling
  chosen because the home's function is not a pinned name where
  `Lever::curvature_radius(poison())` would be. And `RimArms::uniform`'s
  doc now has to explain that for one of its three surfaces the argument
  is unread. A cone's `RimArms` wants no level arm at all. `likely`.
- **S5 (Q1, Q7). `pair_bound` holds the half-angle twice.**
  `intersect.rs:2657-2658`: the new `half_angle: T` beside a tuple that
  already carries `cos_a` and `sin_a`, added so the door can take the
  cosine again itself. The function also reads the same window
  (`range_along(apex, a)`, `:2670`) that the door reads again. Same bits
  today because both are `half_angle.sin_cos()`; nothing holds them
  together. `sure`.
- **S6 (Q1). `ruled_frame` recomputes what the door computed.**
  `intersect.rs:1271-1275` and `extent.rs:1047-1050`: `c = a·n` and the
  signed gap are each computed in the door and again in the frame from the
  returned pivot. The comment "The gap and the lever share the reach's
  pivot" is true only because the two spellings agree. `likely`.
- **S7 (Q1). The crossing's common perpendicular is read twice per pose**,
  once in `decide_stance` and once inside `of_cylinders_crossing`
  (`cylinders.rs:281`, `:372`). One reader, as the PR says, called twice
  with the same arguments. `sure`, and minor.
- **S8 (Q5). The curvature home's header no longer describes its
  readers.** `curvature_arm.rs:4-6`: "Each reader here takes the carrier
  it reads. A reader that also takes a point reads that point's distance
  from the carrier's axis". `chart_curvature_arm` takes a lane, a point
  that is not read as a distance from an axis, and a `Lever`. `likely`.
- **S9 (Q3). The step-import row pins the looseness.**
  `build.rs` `a_crease_is_read_over_its_faces_run` asserts the refusal's
  gap in `4.4e-7..4.5e-7`, the fold times the box diagonal 4.47. The two
  discs' diameter is 4.0. A tighter reading that is still a bound from
  above goes red on this row. `likely`.
- **S10 (Q6). A vacuous assertion beside the real one.**
  `extent.rs` `a_covers_diameter_is_its_boxs_diagonal`: `assert!(d >= 2.0)`
  one line above `(d − 2√3).abs() < 1e-9`. `sure`.
- **S11 (Q3). `a_charts_arm_is_its_lanes_bound…` asserts a range that a
  degraded bound still meets** (`arm > 0.0 && arm <= 0.05`); with m2 this
  row holds the `None` arm and little else. `likely`.
- **S12 (Q4). A sentence that tied two readers together was deleted, not
  answered.** The old comment at `build.rs:504` said the import's lever is
  read "as a wedge between them is (`topo::wedge_reach`)". The new reading
  is not, and `wedge_reach`'s docs argue for the opposite direction. I
  think this is the code drifting from something meant to hold, not a
  stale sentence (m1). `likely`. The sibling unit in topo meets the same
  choice at `wedge_reach.rs`'s own pin.
- **S13 (Q2). `extent.rs`'s module docs link `Reach::hinge_lever` and
  `Reach::turn_lever`**, which this PR makes private. The doc gate passed,
  so this is a rendering matter at most. `unsure`.
- **S14 (Q8). `extent.rs` read whole**: nothing else off that this PR
  touches. The module doc's rule ("no longer than the caller's own measure
  of that region") and the README's ("no longer than the site's own
  measure of it") are met by any measure a site cares to take, a cover's
  box included, so they do not bite on m1; only the "from below / from
  above" sentence does. `unsure` whether that is a gap in the contract.
- **S15 (Q1, class).** The class behind M1, S5 and S6 is *a door that
  recomputes a caller's datum so as not to be handed it, while the caller
  keeps its own copy*. Where else I would look: the sibling unit's
  conversions in topo, which share this gate and this pressure.
