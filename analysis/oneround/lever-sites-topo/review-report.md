# Review — PR 7228, `oneround/lever-sites-topo`, frozen head `ad1b9352`

Unit row: `work/oneround/the-lever-sites-that-hold-their-geometry-read-it-in-topo.md`.
Reviewed commit: `ad1b9352af1c036b9af6b4982b3ab22fa1a3b919`, against the PR's base
`71068072e6d4046cddc4e67334d7cb1cc2c996a9` (the head's second parent; `git diff 71068072 ad1b9352`
is 24 files, +478 −235, the PR's own numbers).

## Verdict

**NOT-MERGEABLE-AS-IS.**

Thirteen of the fifteen pin falls are what the unit says they are, no suite verdict moved in my
differential, and the rim row's re-baseline is a right answer. Two things stop it:

- **M1.** The axial frame's one lever was moved from a bound from below to a bound from above on
  the ground that "its alignment verdicts act on Zero". Four of the verdicts that same lever arms
  act on a definite sign, and one of them then divides by the quantity it decided. For those the
  unit moved the reading *away* from the bound the README assigns, by up to the author's own
  measured 2.04×, and the comment, the reader line and the PR body all say the opposite.
- **M2.** Two of the fifteen pins fell by respelling the same caller's points through a door the
  gate cannot see, which is the one thing claim C1 says did not happen.

Both have bounded fixes; neither is a redesign. M1 needs a ruling first, because the conversion
was made on one.

## What I executed, and what I only read

**Executed** (all in detached worktrees at the frozen head and at the base, each with its own
`CARGO_TARGET_DIR`):

- `scripts/gates/lever-doors.sh`, `cover-kind-match.sh`, `lever-sources.sh` on the frozen head
  with `GATE_BASE=71068072`: all three pass. `LINES_FOR=ALL` lists 24 topo sites.
- A gate plant (`probes/gate-plant.patch`): five production calls that hand a made-up number or
  point to the doors this unit newly uses.
- Three probe rows of my own and a sweep of the rim row (`probes/mutant-switches-and-probe-rows.patch`).
- A mutant sweep: 28 mutants behind an environment switch in one build, each run over the per-PR
  topo suite (`--profile ci`, 4022 tests, the slow set not in it).
- The differential: see C4 for exactly which runs finished.
- The hosted run 44862635526 resolved to step level through the jobs API.

**Only read**: `fate.rs`, `overlap.rs`, `join.rs` (the frame functions and `bool_connect`'s germ
closures, not the whole 8634 lines), `offset_axial.rs` (`axial_frame`, `classify`, every use of
`frame.extent`, `corner_arms`), `offset_together.rs`, `ring_path.rs` and `spline_wall.rs` (the
converted arms and their consumers), `separating.rs`, `solid_contain.rs::selection_reach` and its
callers, `containment.rs::extent_from` and its callers, `geom-brep/src/extent.rs` (the module
docs, `ExtentBall`, the extent readers, `LeverBounds`), `topo/src/cover.rs` (end to end), the
three rows, the gate with its header and counter, the README section. The C7 classification was
checked by reading thirteen callers. I did not run the app scope (`mesh`, `editor-core`), so
C11's seven app reds are neither confirmed nor refuted here.

## Wrong premises in the brief

- **N1 (NOTE, sure, DEMONSTRATED).** "Your branch is … created from the frozen head." It was not:
  `origin/oneround/lever-sites-topo-review` stood at `699a1d2a`, main's tip, which is neither an
  ancestor nor a descendant of `ad1b9352`. A fast-forward was refused. I did not force it. The
  review ran in detached worktrees at the frozen head; this branch carries the report and the
  probes as patches that apply to `ad1b9352`.
- **N2 (NOTE, sure).** C3 says the axial frame's "alignment verdicts act on Zero". That is true of
  some of the verdicts on that lever and false of four. It is M1.
- **N3 (NOTE, sure).** The brief speaks of "the author's six corrections to the pinned-sites row";
  the PR body lists five.
- **N4 (NOTE, sure).** The PR body's differential was taken against "#7143's branch merged with
  `main` at `3c300524`", not against the frozen head's base. Its pass/fail paragraph is no longer
  true of the frozen head (m6).

## MAJOR

### M1. The axial frame's lever is now read from above for four verdicts that claim a definite sign — BY-INSPECTION, sure

`crates/topo/src/offset_axial.rs:1085-1094` builds `Frame.extent` as the largest
`cover::face_reach_from` over the scope's faces: a bound from above. The comment at `:1074-1084`,
the reader line in `scripts/gates/cover-kind-match.readers` and the PR table give one reason:
the alignment sines act on Zero. `Frame.extent` arms every one of these:

| Site | Predicate | Acts on | Owed (README) | Reads |
|---|---|---|---|---|
| `:1121` `classify`'s `sine` | `offset_axial_alignment`, `offset_axial_axis_parallel` | Zero (names a chart aligned) | above | above ✓ |
| `:2684-2688` `latitude_posture` | `offset_axial_latitude_tilt` | Zero (`Ok(None)`, the posture holds) | above | above ✓ |
| `:1491-1505` | `offset_axial_pole_station` | **Positive**, then `c / n.1` at `:1506`; Zero refuses | below | **above ✗** |
| `:2348-2360` | `offset_axial_generator_sense` | **Positive / Negative** choose the line's sense; Zero refuses | below | **above ✗** |
| `:2471-2484` | `offset_axial_rim_sense` | **Positive / Negative** choose the rim's sense; Zero refuses | below | **above ✗** |
| `:2406` | `geom_brep::plane_torus_section(.., frame.extent, ..)` | a served class on its definite side | both | **above only ✗** |

Before this unit the extent was the farthest *vertex* of the scope from the axis point. Every
vertex is a point of the scope, so that was a bound from below: right for the three sense and
station verdicts, wrong for the alignment sines. The unit swaps which group is wrong. The README
sentence the brief quotes covers exactly this case: "one that acts on every sign reads both and
acts only where they agree", and the next bullet: "a record that verdicts read from different
points holds its region, not a lever". `Frame` is a record holding one `Lever`.

The author measured the move ("grows by 1× to 2.04× over 8809 frames, and it is never shorter")
and logged the other four margins ("the four other levered margins, 2398 … the same sign at every
call"), so the four were seen; the body does not say which way they are acted on. No suite verdict
moves because the two sense cosines are ±1 on every body the workspace builds. `pole_station` is
the exposed one: it is a component of a profile normal, and a Positive it should not have claimed
becomes a divisor.

This is the defect the unit itself files at P1 against the turn-lever doors
(`work/oneround/a-turns-lever-is-read-from-below-for-every-caller.md`: "one door cannot choose for
all of them"), minted fresh in the one site it converted by ruling. The style brief's warning names
the shape.

The reader line for `axial_frame` states only the Zero direction ("a looser cover lengthens the arm
and leaves Zero sooner, refusing an alignment"). For the three definite-sign verdicts a looser
cover *admits*.

### M2. Two pins fell because the same points were respelled out of the gate's sight — DEMONSTRATED, sure

Claim C1 is that no pin fell "because a door was renamed out of the gate's sight". Two did.

- **`splitting::containment::extent_from`** (`containment.rs:1570-1583`). The vertex term was
  `ExtentBall::point(*v).lever_from(from)` per vertex. It is now
  `ExtentBall::of_cover_from(Some(&Cover::points(verts.to_vec())), from).lever_from(from)`. `verts`
  is still the `&[Point3<T>]` its caller hands. The pinned-sites row's own section "Doors the pins
  do not see → Hand-built geometry" names this spelling as a way a caller's length passes the gate
  (`ExtentBall::of_cover(&Cover::segment(p, q)?)`, and `Cover::points` by name). The author's
  differential agrees it is the same number: "`extent_from`: 240345, all the same bits".
- **`ring_path::spline_crossings`**, segment and ruling arm (`ring_path.rs:1814-1816`). Was
  `ExtentBall::point(from).lever_from(to)`; is `edge_extent_inf(&Curve3::line(from, to - from), 0, 1)`,
  which for a line is `eval(0).distance(eval(1))`. `from` and `to` are the piece's stored corners.
  "4490, all the same bits." This one has a defence the other lacks: the arm does consume that line
  over `[0, 1]` as the crossing's carrier. But the unit *left* `Quadric::ruled_paths` pinned,
  eighty lines up the same file, for the reason that "its points are computed corners"; these are
  the same corners.

**The plant.** `probes/gate-plant.patch` adds one production function to `separating.rs` that
hands a made-up `f64` to: `ExtentBall::of_cover(&Cover::points(..)).diameter()` (the new door),
`edge_extent_inf` of a line built from two made-up points, `of_cover_from` of a points cover,
`cover::piece_reach_bounds` with made-up `ends`, and `edge_extent_inf` over a made-up span.
`lever-doors.sh` and `lever-sources.sh`: **OK, 44 pinned sites**, silent on all five.
`cover-kind-match.sh` goes red on one of the five, `piece_reach_bounds`, and only as an
unregistered reader: it asks for an argument line, not for the geometry.

So for each new or newly used door: `ExtentBall::diameter` takes no number of its own but levers
whatever ball it is handed; `edge_extent_inf`, `Cover::points`, `of_cover_from`,
`piece_reach_bounds`, `edge_reach_bounds(.., pivot)`, `face_reach_from(.., at, ..)` and
`hull_of_covers` each take a carrier, a span, a point or a cover the caller could have made up,
and none is on the gate's list. The gate's header discloses the class (hand-built geometry, a
caller's point). What is not disclosed is that two of this unit's fifteen falls *are* that class.

The other thirteen falls are honest reads: `fate::leaving`, `fate::piece_beside` and
`overlap::half_plane` read the certified edge's own carrier and span; `offset_axial::corner_arms`'
fallback and `IsoPiece::travel` read the carrier and span they hold; `offset_together::corner_arms`
reads the body's edge from the body's own vertex point (the dropped `here` was that same point,
`offset_together.rs:269`); `planar_axis`, `axial_frame`, `frame_reading` and `surface_pair_reach`
read `face_cover`; the three seeds are gone.

## MINOR

### m1. `planar_axis` levers a definite-sign verdict at a bound from above — BY-INSPECTION, sure of the facts, likely of the reading

`separating.rs:169-176`: `UnitVec3::levered(n, PAIR_NORMAL, band, ExtentBall::of_cover(&cover).diameter())`.
The verdict is "the normal has a length", decided positive; `None` only drops the axis or refuses
(`face_axes` continues, `vertex_face_side` refuses). That claims a definite sign, so the README
assigns the bound from below. The new door's own doc says what it is: "a bound from above on the
diameter of the region it holds" (`extent.rs:298-299`), and the reader line says what it does: "a
looser cover lengthens the arm and admits a shorter normal".

The move is in the right direction (√5 → 2, shorter), so the unit made this better, not worse. But
it is not the owed bound, and the unit's own filed row
(`a-germ-planes-normal-is-levered-from-its-stored-origin.md`) calls exactly this, on exactly this
door, the wrong direction: "`UnitVec3::levered` decides `|n|·arm` positive, so a longer arm admits
a shorter normal. That is the direction the comment's bound from below exists to prevent." The new
row `a_planar_faces_normal_is_levered_at_its_diameter` pins the from-above reading: its first
assertion goes red on a *shorter* arm (the radius mutant), which is the README's direction.

### m2. The axial frame's lever is not read where its datum is — DEMONSTRATED (a probe), sure; not this unit's regression

The PR table gives `axial_frame`'s pivot as "the axis point; the axis datum is read there". The
lever is measured from `origin`, the foot of the *world origin* on the seed axis
(`offset_axial.rs:1073`). The datum each chart is aligned by is read somewhere else:
`centre_on_axis` (`:2648-2661`) reads the chart's own stored `origin`, `apex` or `center`, and a
plane's `Station` reads the plane's stored `origin` (`:1136`). A tilt pinned at a point `c` moves a
scope point `x` by `θ·|x − c|`, and `|x − c|` is not bounded by the far distance from `origin`.

Probe `review_probe_the_alignment_datum_is_read_at_the_charts_stored_origin` (passes, in the probe
patch): a frame whose scope reaches 1 m, a cylinder whose stored origin lies on the frame's axis
100 m up it, tilted by `sin θ = 0.5·ε`. `classify` answers `Wall(1)`, coaxial. The wall's own axis
stands **50·ε** off the frame's axis where the scope is; K is 10.

The vertex fold had the same pivot, so this is not new. It is recorded because the PR states the
opposite, because "from above" is only a bound from above *about the pivot the datum is read at*,
and because it is the defect the unit filed for germ planes and did not file here.

### m3. `IsoPiece::travel` changed a predicate's datum, and three texts still say "chord" — BY-INSPECTION, sure

`travel().0` is not only an arm. `SplineWall::paths` (`spline_wall.rs:155-160`) reads its *length*
as the margin of `split_ring_path_wall_arrival`: `Margin::of(charge(chord.length(), slack))`, and
offers the path where that is not Zero. That margin was the chord of the arriving piece's ends and
is now the piece's extent from below, so a piece whose ends coincide but which runs round a closed
wall changes from "no path" to "a path". The README: "A length that is the reading itself … is the
margin … neither is an arm."

Still saying chord: `spline_wall.rs:114` (the predicate's own doc), `:224-236` (`reach`'s doc,
whose argument rests on "its own length, which its chord does not exceed"),
`docs/predicate-dimension-audit.md:761`, and the local `chord` at `spline_wall.rs:155` and `:215`.
The arm's two other readers act on a definite sign (`split_ring_path_spline_across` counts a
crossing on Positive only, `ring_path.rs:1863-1870`; `split_ring_path_lean` reads a winding on
Positive or Negative, `chord_join.rs:4453-4460`), so the bound from below is the owed one there and
the move 0.309 → 1.994 is in the README's direction: the extent from below is still no longer than
the piece.

### m4. The rows are not true of the head in five places — BY-INSPECTION, sure

- `lever-doors-pinned-sites-await-a-geometry-read.md`: the title and the first two paragraphs still
  say the gate pins 59 references. It pins 44.
- Same row, "Wrappers": `topo::separating::planar_axis(body, face, reach, band)` "takes a closure
  returning `(lo, hi)`. 4" and `topo::boolean::join::frame_reading(sa, on_a, on_b)` "takes two point
  lists. 1". Neither signature exists at the head.
- Same row: `edge_reach` is left for "about 35 callers"; the row it points to, and the PR body, say
  about 41.
- `a-germ-planes-normal-is-levered-from-its-stored-origin.md` cites the `germ_normal` closure at
  "about line 1384" and `plane_of` at "about line 1470". They are at `join.rs:1495` and `:1579`.
- The same row has no `priority` or `cost`. What it says of the code is true.

### m5. Three "definite sign" callers on the turn-lever row act on their Zero — BY-INSPECTION, likely

Thirteen callers checked against the code (list under C7). Ten are classified as the code acts.

- `boolean::join::stands_on` (`join.rs:1242-1248`) and `boolean::reduce::sheets_part`
  (`reduce.rs:3944-3953`, two calls) both end `== Sign::Negative`. Zero is returned as `false` and
  the caller acts on it. The row notes this ("Zero reads as … unrefused") and still files them under
  "definite sign", then concludes "Only the definite-sign sites are served by the bound from below
  that every site reads today". For these three a short lever turns a true Negative into a silent
  "not opposed" / "not parted". They are not served.
- `chord_join::chord_spec` and `split::split_edge_onto` are filed "Zero". `must_carry_description`
  also carries the second-order rung, where a determinate bend and an under-determined one are both
  served. `unsure` whether that makes them "both".

The two callers outside topo are listed and not classified, as the row says.

### m6. The PR body is not true of the frozen head in three places — DEMONSTRATED, sure

See C11 for the evidence.

### m7. Seven conversions have no row, and their mutants survive — DEMONSTRATED, sure

See C8. The body discloses "No row for the last-bits sites". Nothing schedules one.

### m8. New refusal arms with no row, two of them unreachable — BY-INSPECTION, likely

- `offset_together::corner_arms` → `CarrierLaneUnsupported` (`offset_together.rs:716-720`). No row
  names it; the same variant and text is raised later in the same door (`:356`), so a row asserting
  the variant cannot tell the two apart.
- `fate::piece_beside`: two new desyncs (`fate.rs:1289`, `:1293`). `edge_is_line` has already
  passed three lines up, so the first cannot fire on a body that reads.
- `solid_contain::selection_reach` → `ZeroVolumeBody` (`solid_contain.rs:2408`).
  `point_in_faces` returns that variant for an empty selection at `:2040`, before any predicate, so
  from there the new arm is dead.
- `offset_axial::axial_frame`'s `.ok_or(EmptyGroup)` (`:1094`): `first` at `:1028-1032` already
  returned `EmptyGroup` for a scope with no face.
- `extent_from` now refuses an empty `verts` as `CorruptLoop` where it read zero plus the covers.
  `loop_reach` guards `verts.first()`; `carrier_walk` does not. `unsure` whether a `CarrierLoop`
  can be vertexless.

## NOTE

- **The rim re-baseline is a right answer** (C4). See the sweep there.
- **A clockwise torus face did not under-read in my probe** (C3).
- **Hosted CI** (C11): every job that ran is green; the slow set, the Interval job and five others
  did not run.
- `axial_frame` now reads `face_cover` of every in-scope face on each of its two entries
  (`is_axial_in`, `offset_charts_together_staged`). Not timed. `unsure` whether it matters.
- A face with no cover in the scope now poisons every margin on the frame, exact zeros included
  (`0 · NaN`). Before, the fold never read a face. The `axial_skip_uncovered` mutant says whether a
  row holds that (C8).

## The claims

### C1 — each pin fell because the site reads geometry it holds

**Thirteen of fifteen: holds. Two: falsified (M2).** The plant and the per-door answer are under M2.

### C2 — the bound each site reads is the one its verdict is owed

| Site | How its verdict is acted on | Owed | Reads | |
|---|---|---|---|---|
| `fate::leaving`, line arm | every sign, through `LeverBounds` and `decide_levered` | both | `Exact` (a line's cover is the edge) | ✓ |
| `fate::piece_beside` | every sign (`corner_holds`) | both | `Exact` | ✓ |
| `overlap::half_plane`, line arm | every sign | both | `Exact` | ✓ |
| `offset_axial::corner_arms`, fallback | definite (a corner resolves on one) | below | `edge_extent_inf` | ✓, reached by no test |
| `offset_together::corner_arms` | definite | below | `.below()` | ✓ |
| `ring_path::spline_crossings`, segment | definite (`!= Positive` says nothing) | below | the chord, exact | ✓ |
| `IsoPiece::travel` | definite twice, and once as a margin | below | `edge_extent_inf` | ✓ as an arm; m3 as a margin |
| `separating::planar_axis` | definite | below | the ball's diameter, **above** | ✗ m1; moved the right way |
| `offset_axial::axial_frame` | Zero at two sites, definite at three, a served class at one | both | **above** | ✗ M1 |
| `solid_contain::selection_reach` | every sign (the reader line: Zero skips, nonzero reads a crossing) | both | above only | unchanged by this unit: only the seed went |
| `containment::extent_from` | the schedule gate's lever | — | unchanged, bit for bit | M2 |
| `join::frame_reading`, `surface_pair_reach` | the frame's pose, either sign | both | above only, from a new point | unchanged in kind; C5 |

The three moves the brief asks about: `IsoPiece::travel` 0.309 → 1.994 is toward the README (a
longer bound from below, for definite-sign readers). `planar_axis` √5 → 2 is toward the README (a
shorter bound from above, for a definite-sign reader) and stops short of it. `axial_frame` up to
2.04× longer is toward the README for two verdict sites and **away from it for four**.

### C3 — `axial_frame`

- *Alignment verdicts act on Zero, so from above.* True of two sites, false of four: **M1**.
- *The axis point is the pivot the datum is read at.* **No: m2**, with a passing probe.
- *"From above" is what `face_reach_from` gives for every face kind.* By reading `cover.rs`: a
  plane, cylinder or cone face is its boundary's convex hull met with its carrier's wall or frustum;
  a sphere is the ball met with windows only where proved; a spindle and a spline are the whole
  carrier; a torus is the tube met with its chart window. The open BOXES row
  (`torus-box-window-does-not-bound-a-clockwise-face.md`) says the window does not bound a
  clockwise face. Probe `review_probe_axial_extent_on_a_clockwise_torus_face`
  (`torus_wall_clockwise`, R = 2, r = 1.5, centre 5 m up the axis, window `v ∈ ±0.1`): 
  `face_reach_from` reads **6.885165**, the complement's own sampled far distance is **6.885158**.
  It read the whole tube, not the patch window, so I could not reproduce an under-read on this
  fixture. The BOXES row stays a by-reading caveat: `likely` latent, `unsure` whether still live.
- *A face with no cover reads poison and escalates.* By reading, yes: `Lever::max` is
  `Real::max`, which propagates poison (`predicate.rs:892-899`).
- *No seed, an empty scope is `EmptyGroup`.* Yes, and one line earlier than the new `.ok_or` (m8).
- *`T: Real` → `Decide`, no public signature moved.* Yes: both callers already held a `Band`.
- *The row is red on the vertices-only fold.* By arithmetic yes (2.380 against `[2.5, 2.5 + 1e-9]`).
  Probe `review_probe_the_axial_rows_fixture_face_by_face`: both of the fixture's faces read 2.5,
  the patch and its hand-made complement alike, so the row pins the patch's equator and says
  nothing about which face supplied it.

### C4 — no verdict moved but the rim row

DIFFERENTIAL_PLACEHOLDER

**The rim row.** `review_probe_rim_sweep` runs the row's fixture at 201 values of `k` from 0.5 to
10.5, in both face orders, through `frame_lever`. Below the band (`k < 1`): served. From `k = 1.00`:
`pc_parallel_gap`. From `k = 2.00` to the top of the band: `pc_axis_plane_parallel`. Both orders
agree at every step, and no in-band `k` is served. So the re-baseline is the same outcome, an
escalation, under a different name for a reason I can name: the plane passes through the rim's
vertex `(1, 0, 1)`, and the reading point moved from that vertex, where the gap is exactly zero and
the tilt's lever is 2, to the middle of the face's window, where the gap is `k·ε/2` and the lever
is about √2. `decide_across` reads gap plus levered tilt, `k·ε/2·(1 + L)`, which is at least the
true offset `k·ε` at the rim's far end for any `L ≥ 1`. The row's switch at `frac < 0.2` is that
`k = 2` boundary.

### C5 — the germ frame's datum moved with its lever

**Holds, by reading.** `frame_lever` (`join.rs:2898-2913`) computes `at` once and hands it to
`frame_extent`, and returns it. Its two production callers: `germ_loops` (`:2889-2890`) passes the
same `at` and extent to `pair_section_frame_at`; `pair_reading` (`:3624`) discards `at` and keeps
the extent, which for a plane×cone pair is a `Reach::Face` that carries `at` inside it
(`cover::cone_face_reach`, `cover.rs:485-489`). `surface_pair_reach` builds `Reach::of_face(at, ..)`
from the one `at` its measures were taken from. `pair_section_frame` (the `Radii` entry) has no
production caller. The `frame_swap` mutant is in C8.

### C6 — the sites left

Each reason is true of the code as I read it:

- `germ_reach` (`join.rs:1477`, `:1484`): the antipode is mirrored through `lp.axis.origin` because
  `SectionLoop` holds an axis and an optional ball, no conic. ✓
- `Reach::of_face` at `join.rs:3113`, `chord_join.rs:1687`, `cover.rs:485`, and the `Reach::Face`
  pattern at `join.rs:3098`: all four build or destructure the variant the open BOXES row
  `reach-readme-says-no-vouched-length-but-two-variants-are-one` is about. ✓
- `section_cert::cone_plane` (`:1515`): called at `:1475` with `w.normalize()`, a computed plane. ✓
- `ring_path` `ruled_paths` (`:450`): `Piece::Segment` has a `lever` field. ✓ (and see M2 on why
  its neighbour was not left for the same reason)
- `cover::meet_within` (`:208`): both callers hand `boxes::sweep_pad(band)`. ✓
- `WedgeReach::reach`, `pcurves::decision::edge_reach`: filed (C7). ✓

**Scheduling.** The `Reach::Face` family has its BOXES row, the two turn-lever doors their new P1
row, and the germ pivot its new row. `cone_plane`, `ruled_paths`, `meet_within` and
`germ_reach`'s own record change (`SectionLoop` holding its conic) are scheduled only by a "Left"
note on the umbrella row. `likely` that is thinner than Q6 asks.

### C7 — the turn-lever row

My grep finds 39 call lines in topo production plus the method calls, so "about 41" stands. Checked:
`attach::set_face_surfaces_certifying` (definite ✓), `attach::reask_on` and `reask_chart_on` (every
sign ✓), `combine::graft_solids_impl` (definite ✓), `edge_join::joinable` (every sign, then
definite ✓), `join::stands_on` (**✗ m5**), `reduce::sheets_part` ×2 (**✗ m5**), `route_rim` ×2
(Zero ✓: a `Seam` is routed, the rest refuse), `ops::describe_edges` (every sign ✓),
`slit::walls_inside` (every sign ✓: Negative, Positive and a second-order arm on Zero),
`chord_join::chord_spec` and `split::split_edge_onto` (Zero, `unsure`, m5). The premise in the
title, that the doors read from below, is true: `points_lever_from` over the carrier's two ends
and the faces' boundary vertices from the span's middle (`wedge_reach.rs:111-129`,
`decision.rs:292-300`).

### C8 — mutants

MUTANT_PLACEHOLDER

### C9 — the counts

**Holds.** `lever-doors.counts` loses 15 and gains nothing: `fate` 2, `join` 4 → 2, `overlap` 1,
`offset_axial` 3, `offset_together` 1, `ring_path` 2 → 1, `spline_wall` 1, `separating` 1
(`box_diagonal`), `solid_contain` 1, `containment` 4 → 2. Topo's lines sum to 24 and the file to
44. The gate passes against the base. `cover-kind-match.readers` gains seven lines for the seven
new reads (`piece_beside`, `frame_lever` ×2, `surface_pair_reach` ×2 doors, `axial_frame`,
`corner_arms`, `planar_axis`), each with a direction; the gate passes, so none is missing. Two of
the directions are the ones M1 and m1 quarrel with.

### C10 — the two filed rows, and the corrections

The germ row says what the code does (`join.rs:1495-1499`, `:1579-1581`), with stale line numbers
and no priority (m4). The turn-lever row: C7. The five corrections in the PR body are each true of
the code: `surface_pair_reach` is one point and one `of_face`; all four of `planar_axis`' base
callers handed `census::face_reach` of the same face; a loop's vertices are not redundant *by
argument* (no row holds it: the `extent_noverts` mutant survives, C8); `offset_together` holds each
edge; only the line arms of `leaving` and `half_plane` were points. What the row still gets wrong is
m4.

### C11 — the PR body, and the reds that are not this unit's

CI_PLACEHOLDER

## Ratings

- **Idiom and structure: 3/5.** Most conversions delete a special case and take the door every
  other kind already took (`fate.rs:500`, `overlap.rs:563`). Against that: one `Lever` on `Frame`
  for verdicts of both kinds (M1), and three different answers to "a face with no cover" in three
  folds written in one unit.
- **Test quality: 2/5.** Five rows with tight two-sided windows, each of which I saw go red on a
  mutant. But seven conversions have none, and eleven of my mutants at them survive 4022 tests,
  two of them deleting a whole term.
- **Doc and comment honesty: 2/5.** The PR body is unusually exact about what it measured. But the
  comment that justifies M1 is false of four of the six sites it governs, a true "unpinnable"
  disclosure was deleted with nothing pinning what it disclosed, and the chord, the box diagonal
  and the 59 survive in seven places.

## Style

Questions exercised: **Q1, Q2, Q3, Q4, Q5, Q6, Q7** and **Q8** (the file read end to end was
`crates/topo/src/cover.rs`, 1321 lines; of the files the diff touches I read `separating.rs` and
`spline_wall.rs` whole and the rest in part: `join.rs` at 8634 lines I did not read whole). The
prose sweeps of Q1 and Q2 were run over the twelve touched files; the constants sweep was not.

- **S1 (Q1, sure).** `offset_axial.rs:1218-1272` and `offset_together.rs:680-723`. Two
  `corner_arms`, declared a deliberate copy at `offset_together.rs:686`. After this unit they arm
  one predicate four ways: a parameter span (`unsourced`), an arc length (`unsourced`, longer than
  the arc's extent), `edge_extent_inf`, and `edge_reach_bounds(..).below()` from the vertex. A
  definite-sign verdict gets a bound from above on a circle and from below on every other kind.
- **S2 (Q4, sure).** `offset_axial.rs:1218-1227`. The axial copy's doc still says "The planar door
  levers by each edge's CHORD" and "The arc length is the length that was always meant". The planar
  door now levers by a cover's reach, and this function's own fallback arm by an extent.
- **S3 (Q1, likely).** `solid_contain.rs:2401-2408`, `offset_axial.rs:1085-1094`,
  `containment.rs:1571-1583`. Three seedless "largest reach from a point" folds written in one PR.
  A face or edge with no cover is an error naming the face in the first, a poison lever in the
  second, and an error naming the loop in the third. Class: I would look at every other
  `reduce(Lever::max)` and `try_fold` over `face_reach_from`.
- **S4 (Q2, Q6, sure).** `offset_axial.rs`, the comment removed above `:1085`. The deleted paragraph
  said the scoping of the extent's walk is unpinnable and why. The scoping is still there
  (`scope.faces_in_scope()`), and the new row builds `Scope::whole`. Whether anything pins it now is
  the `axial_allfaces` mutant in C8.
- **S5 (Q4, Q5, sure).** `separating.rs:42-45` (`PAIR_NORMAL`: "levered by the face's reach
  diagonal"), `:102` ("a face with no reach"), `census/in_plane.rs:172-173`,
  `boolean/reduce.rs:658` and the refusal text at `:673` ("over its reach"). All cite the lever
  `planar_axis` no longer reads.
- **S6 (Q7, sure).** `census.rs:9858-9869`. `backstop_apart` still takes `(fa, ra)` and `(fb, rb)`,
  builds the pairs, and throws both reaches away in `|(f, _)|`. The closure's removal left two dead
  parameters and whatever their callers do to produce them.
- **S7 (Q1, likely).** `census/in_plane.rs:174-177`. `normal` now reads the same face twice in two
  lines: its cover's ball diameter inside `planar_axis`, and its world box's diagonal as the bare
  `T` it returns, which `:196` multiplies by hand into `REGION_SLOPE`. Two measures of one face's
  size, one line apart.
- **S8 (Q1, unsure).** `extent.rs:298-303` beside `:384-391`. `ExtentBall::diameter` and
  `ExtentBall::<f64>::certified_diameter` are the same number through two doors, one generic and
  one that enters the Interval lane.
- **S9 (Q2, sure).** `spline_wall.rs:215`, `:155`. A variable named `chord` that holds an extent.
- **S10 (Q7, likely).** `join.rs:2941-2951`. `frame_reading` takes two `FnOnce` covers and calls
  one. Its rows pass the same closure twice (`cone_frame_rows.rs:396`, `torus_frame_rows.rs:590`),
  and two rows hand it `Cover::points` of vertices (`join.rs:6782-6784`), which is the old pivot's
  point set under the new door, not what production hands it.
- **S11 (Q3, sure).** `offset_axial.rs:4062-4094`. The axial row cannot tell which face its 2.5
  came from: both faces of its fixture read 2.5 (probe). It goes red on the vertex fold; it would
  not go red on a fold that read only the first face.
- **S12 (Q3, likely).** `cone_frame_rows.rs:955-964`. The window `[0.5, 1.01]` is read with
  `reach.lever_from((1, 0, 0))`, a pivot the row chooses, not the reach's own `at`. I did not work
  out what a wrong `at` inside the face does to it.
- **S13 (Q6, sure).** PR body, "No row for the last-bits sites". Disclosed, not scheduled: no row
  under `work/` asks for one.
- **S14 (Q5, sure).** `offset_axial.rs:313-317`. `Frame`'s doc: "the scope's own radial extent — the
  length that levers every direction test here". It is not radial (it is a distance from a point on
  the axis), and "every direction test" is the fact M1 turns on.
- **S15 (Q2, likely).** `cover.rs:1150-1154`. `piece_reach_bounds`' doc names one caller
  ("`overlap`'s half-turns"). It has two now.
- **S16 (Q7, unsure).** `ring_path.rs:1866`. `slack / length.length()` then
  `Margin::levered(.., length)`: the lever's length divided out by hand and multiplied back, which
  the gate's header lists as something it cannot see. Older than this unit; the conversion changed
  the `length` it does it with.
