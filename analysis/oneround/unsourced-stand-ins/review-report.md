# Review: PR 7268 (`oneround/unsourced-stand-ins`) at `229a355003435383c53b38fd8cd7b6f979a776db`

Reviewer lane, ONEROUND. Full review: the claims C1 to C8, then the style lane.

## Verdict: APPROVE-WITH-FIXES

There is no MAJOR. The seven conversions do what the PR says at every
reaching test I ran. The verdicts that move are the ones the author names.
All the claimed mutant kills reproduce. The fixes owed are about truth:

- the spiric conversion has an observable defect;
- two "unobservable" claims are false;
- one bit-identity claim is false;
- stale docs at two of the seven sites;
- the brief's "every job green" premise is wrong, because `gate ok` never
  ran and the interval job was skipped.

## What I executed, and what I only read

**My base, and why.** The frozen head has two merge bases with
`origin/oneround/unsourced-bits`: main `3900eb462` and bits `2e721a928`.
The head took PR 7227 at `2e721a928`, not at its current head `dc75d8ffe`,
which merges later main. So I built the base as the merge the head made,
`3900eb462` + `2e721a928`, resolved as the author did:

- main's side on `curve_face/line_quadric.rs`;
- 7227's side on the gate count files, which the comparison does not read.

The result (`8d20536ed`, local only) differs from the head in `crates/`
only by this PR's eleven files. Comparing against `dc75d8ffe` instead would
have mixed in main's later moves.

**Instrument.** A per-site log in `geom_core::k_stats::classify`
(`tools/k_stats-site-log.patch`, applied to both trees identically and not
committed to the head). It writes one line per decision of the eight site
names: the test, the site, the sign or ESC, and the `MarginDiag` Debug.
That Debug is shortest-round-trip, so it is bit-exact, and it marks the lane
(`Value` for f64, `Enclosure` for `Interval`). The fuzz seed is pinned with
`CAD_FUZZ_SEED=0x7268`.

My first unpinned run showed dozens of spurious "moves" from fuzz draws. The
brief's "seeds fixed" condition is load-bearing.

**EXECUTED (debug, `ci` profile, `--no-fail-fast`):**

- **geom-brep + topo, at ε = 1e-9 and 1e-12, both trees.**
  - 1e-9: head 6488 run, all pass; base 6486, all pass.
  - 1e-12: one red on both trees, the same test:
    `envelope_lemma_fuzz::projected::a_line_near_a_ruling_pinned_where_the_frame_cost_refuses`.
    It is red at seed 0x7268 on the base too. Main has a row naming it,
    `work/onehome/the-carrier-frame-fuzz-bodies-go-red-under-no-mutant.md`,
    so it is not this unit's.
- **sweep, verbs, editor-core, mesh, step-import, at 1e-9, both trees.**
  6951 run, all pass on each. I did not run these at 1e-12: about 15 min per
  tree per ε on this 4-core box.
- **Mutants.** 31 in all, in a separate worktree. Each is either a targeted
  run on the rows the author names, or the whole geom-brep + topo set. The
  table is in `tools/mutants.py`; the C4 table below gives each result.
- **One probe.** `probe-spiric-short-span.patch` and its output
  `.out`, run at the head and under the old-R mutant.
- **The gates on the frozen head:**
  - `lever-sources.sh`: OK, 47 sites, 862 fixtures;
  - `lever-doors.sh` with `GATE_BASE=origin/main`: OK, 58 pinned, none
    above main;
  - `from-f64-allowlist.sh`: OK;
  - `work.py lint`: ok.
- **The hosted CI run 45060254630, at step level** (jobs API).

**NOT executed:**

- the release slow set of geom-brep/topo/sweep, at either ε;
- the 1e-12 leg of the five other crates;
- the `Interval`-only suites of the interval job (`interval.yml`);
- C8's reds on the merge base. I only checked those against the tree's rows.

**ONLY READ:** the README lever bullet, the doors, every changed hunk, both
rows, the gate scripts, and the touched functions in full. I read no whole
file end to end; see Q8.

## Correctness findings

### MINOR-1: the spiric chart-axis read refuses a parallel-axis oval on its own torus as "perpendicular" over a short arc, and the old arm accepted it

DEMONSTRATED, `sure` of the behaviour, `likely` that production reach is
small.

The site is `crates/geom-brep/src/pcurve_cache.rs:13828` (the arm) and
`:13889` (the reading). The arm is `edge_extent_inf` over `place.span`. The
cosine is ±1 for every torus that holds the oval, so the reading
`cos · arc` lands in band once the arc is under about Kε. The refusal then
says something false:

- the oval is refused as `Incidence(Decided { why: "the chart torus's axis
  is perpendicular to the carrier's…" })`;
- that is a definite off-chart verdict about axes that are parallel.

**The probe.** Six random spirics, each on its own torus, `band()` of
`closed_image_rows`:

- at the head, every arc shorter than about 1e-8 m is refused this way
  (for example arc 9.56e-9, 8.45e-9 and 2.5e-9 are refused, 2.3e-8 is OK);
- under the old arm (`spiric_old_R`), all 48 cases are OK.

The comment at `:13878` still argues "a reading Zero or in the sliver puts
the axes within `K·ε` over the arc's extent of perpendicular". That is false
when the arc itself is under Kε: the in-band reading then says the arc is
short, not that the axes are tilted.

Whether a sub-Kε spiric edge ever reaches `chart_pcurve` in a real build is
unproven. Edge certification likely refuses such ends first, hence `likely`.

**Two consequences:**

- C1(c)'s "no verdict reads the difference" (the row's *Read against the
  tree* bullet) is false.
- C4's "old R is unobservable" is false. The probe is a killing row for the
  old arm, and none exists in the tree: `spiric_old_R` survives all 6488
  geom-brep + topo tests.

### MINOR-2: "no mutant killable" at the Villarceau tilt is false in one direction, and the direction that matters is unpinned

DEMONSTRATED, `sure`.

The site is `pcurve_cache.rs:13083`. Over the whole geom-brep + topo set:

| Mutant | Result |
|---|---|
| tilt arm at the minor radius `r` | survives 6488 |
| tilt arm at `R·1e-6` | survives 6488 |
| tilt arm at `R·1e6` | red on `pcurve_villarceau::a_tube_rate_gates_rejected_zero_is_a_class_reading` |

The unsound direction is a shorter arm, which makes the gate's Zero (the
acted-on reading) easier. A million-fold shorter arm turns nothing red. So
the tilt gate's Zero is never what decides a test's outcome; the incidence
test decides first, as the author says.

That makes the gate either dead (masked everywhere) or unpinned. The tree
does not say which. The comment block at `:13087`–`:13108` asserts the
masking "as measured" but holds it with no row. Under the style lane's Q6, a
measured claim owes a guard or a written reason it can have none.

### MINOR-3: "same bits, radius = R at every reaching test" is false

DEMONSTRATED, `sure`.

The claim covers C1(a) and (b) and the row bullet "the circle's radius and
`R` agree to the bit at every reaching test". In
`envelope_lemma_fuzz::a_moved_villarceau_circle_is_refused_typed_and_never_off_within_the_band`
(seed 0x7268), all three of the following move bits but no verdict:

- `pcurve_torus_villarceau_tilt`;
- `_tube_rate`;
- `pcurve_chart_orientation`.

An example tilt margin is `-9.200001840181443e-17` against
`-9.200001844106454e-17`; 12 of 150 tilt readings differ.

The moved circles have `radius ≠ R` within the band, which `_radius` allows.
The claim holds only for unmoved Villarceau circles. It also means the
`unitsign` "old R survives as equivalent" is equivalence on the suites, not
bit-identity.

### MINOR-4: C1(f), the joint room passes the curvature door with a reading that is not a tilt

BY-INSPECTION, `likely`.

The site is `crates/topo/src/pcurves/decision.rs:1883`. The decided quantity
is a quarter of the parallel's chord, `d·sin(step/4)/2`. Here `step` is a
fixed π or 2π, not a small measured turn. So `sin(step/4)/2` is a constant
geometric factor (0.354 or 0.5), and `d` is the length being decided against
4ε.

Under the README, "a length that is the reading itself … is the margin" and
"a dimensionless factor belongs to the reading". Its own lever bullet adds
that a lever is "the length at which a tilt … displaces the geometry".
Nothing tilts here.

**This PR is inconsistent with itself.** It writes the same shape at
`off_axis` (C1e) as `Margin::of(ρ·cos α)`: a residual length times a
dimensionless factor, put in the margin. At the joint room it writes
`Margin::levered(q, parallel_curvature_arm)`. The bits and verdicts are the
same either way, but the door's stated argument ("a curvature arm") is not
what the number is.

The padding half is right:

- the stand-off goes to `Band::padded`, whose doc is exactly this case;
- the arithmetic holds: `2εq ≤ ε` for `q ≤ ½`;
- new is at least as strict as old (`dq > E + ε` against
  `dq > E + 2εq`). The 6.6ε sphere row shows old-passes/new-refuses, as
  claimed.

### MINOR-5: the brief's "every job green" premise is wrong; CI run 45060254630 is not green

DEMONSTRATED, `sure`.

The run's conclusion is **cancelled**. `gate ok` was created at 19:58, never
got a runner (runner_id 0), and was cancelled at 20:43; the origin branch
has since moved to `736271c9b`. So no `gate ok` exists for the frozen head.

These jobs succeeded:

- change filter;
- test (kernel);
- lint;
- mesh budget;
- test (app);
- corrupt input.

**`interval` was skipped**, as were tools, demos, viewer, python and
step-import (freecad). This PR moves verdicts on the `Interval` lane: my log
shows all eight sites reached with `Enclosure` margins inside geom-brep and
topo. The dedicated interval job did not run on it.

In `test (kernel)`, these steps ran and succeeded:

- the new-tests list;
- build;
- the opt0 check;
- default ε (about 2 min);
- the ε-extra step with `eps_extra=package(geom-brep) | package(topo)`
  (1e-6 and 1e-12);
- the slow-set step "this diff adds or edits" (28 s);
- doc-tests;
- the slow-set names check.

"The slow set did not run" and "failing tests" were skipped.

### NOTE-1: C2 arithmetic, β

BY-INSPECTION, `likely`.

β is minted as `ecc/(1 + |rb|/|ra|)` (`pcurve_cache.rs:11954`). So
`1 − |β| ≈ b/a`, the axis ratio, and `1 − e ≈ (b/a)²/2`.

"Needs e within about 2·10⁻¹⁶ of 1" is the value for a room near 2e-8, that
is, the band at ε = 1e-9 with an azimuth arm near 1 m/rad. The scale-free
statement is that the old gate refused where `(b/a)·arm < Kε`. Since `arm`
is about the section's own size, that means the minor semi-axis is within
the band, so the conclusion stands.

I found no β the old gate refused that a correct section produces and that
now goes wrong unrefused. Where `arm < a`, the old gate refused genuine thin
sections (b > Kε), which now pass: toward the truth. Of the three test
values, 0.999 is an ordinary section's β (b/a = 1e-3), which the row
handles correctly by stating it on a section it does not belong to.

My differential confirms β ≥ 1 reads Negative where it read Zero (and, at
1e-12, ESC×4 → decided). It moves nothing else.

### NOTE-2: C1(g) reads `BoolSector::arm` after all, for a bisector-bounded sector

BY-INSPECTION, `likely`.

`reached_bounds()` returns `BoundSupport::lever()`, and for
`BoundSupport::Bisector { arm }` that is the sector's arm
(`crates/topo/src/bound_side.rs:113`). "It does NOT read `BoolSector::arm`"
holds only where both bounds are edges or probes.

### NOTE-3: C3, the differential, against my base

DEMONSTRATED.

All counts below are judged against my base `8d20536ed` at seed 0x7268.

**geom-brep + topo, 1e-9 and 1e-12.** Verdict moves occur only in:

- the two new rows: `bisector_rows`, and the β "any room" row;
- the edited β row `a_focal_section_whose_beta_is_not_below_one_ends_as_its_defect`
  (Zero 18 / Neg 6 → Zero 12 / Neg 12; at 1e-12 Zero 14 / ESC 4 / Neg 6 →
  Zero 12 / Neg 12);
- the two `room_fence` rows' new assertions.

`props_cone_circle_off_axis` is bit-identical everywhere: 71 test-site
pairs, 4381 decisions, 1517 of them on `Interval`.

**Reach of the sites.** Every site is reached on both lanes at 1e-9. The
head counts are:

| Site | F lane | I lane |
|---|---|---|
| `bool_fate_corner` | 19476 | 158 |
| `loop_windings_joint_room` | 309 | 19 |
| `pcurve_spiric_chart_axis` | 212 | 99 |
| `pcurve_focal_section_beta` | 485 | 179 |
| `pcurve_torus_villarceau_tilt` | 702 | 73 |
| `pcurve_torus_villarceau_tube_rate` | 1387 | 146 |

The log does not split `bool_fate_corner` between `bisector` and
`tip_wraps`.

**Five other crates, 1e-9.** No verdict moves. Bits move at
`loop_windings_joint_room` (476 pairs), `bool_fate_corner` (171), spiric
(31) and β (16).

**`mesh d9_mesh_goldens`.** The premise "it reaches only `off_axis`" is
false: both tests also reach `pcurve_chart_orientation` (276 decisions
each). At the pinned seed both are green on both trees with no move, so the
scheduling attribution could be neither confirmed nor refuted here. What
the attribution needs (that both sites are bit-identical) holds.

### NOTE-4: C5, the counts match the tree

DEMONSTRATED by the gates.

Each pin I read has a true reason (pose.rs 2: `Graze::scale`, and the
parameter-run length; rim_wedge 1; region.rs `share`).

But the row `work/oneround/unsourced-levers-the-test-sites-and-the-bit-only-movers.md:22-27`
on this head says `line_cone_roots`' span was converted. On this tree the
same read stands unconverted at `crates/geom-brep/src/curve_face/pose.rs:993`
(`Lever::unsourced(dir.norm() * (hi - lo))`, pinned). That is the side
effect of taking main's hunk. PR 7227's current head has re-done it (as
`curve_face::pose::line_cone_read`), so merging that head will settle it.
Until then, the row sentence is false of this tree.

### NOTE-5: C8, the reds named as main's

BY-INSPECTION, `unsure`. I ran none of them on the merge base.

I found open rows on main naming each family:

- `offer_rows` and `a_corner_reads_only_its_own_sectors`:
  `work/scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type.md`;
- `recourse_roster`:
  `work/green/main-red-recourse-roster-misses-chain-parity-and-spline-trim-decide-sites.md`;
- `germ_tilted_rod`:
  `work/sectarms/main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window.md`;
- `cylinder_sphere_frame`:
  `work/spiricarm/the-cylinder-sphere-frame-pad-table-drifted-past-its-guard-on-main.md`;
- `verbs_cylcyl_r1_review_probes`:
  `work/ssiarith/main-red-a-tall-thin-cylinder-pairs-section-reads-limb-2-in-band-at-1e-12.md`.

For `square_search`, every main-red row naming it is closed. Open rows
mention it, but none is clearly the red. **Its holding row is unconfirmed.**

### C1, the seven, one by one

- **(a) the tilt.** The argument is true: a turn of the circle's plane about
  its centre moves its points by `R·δ`, and the README reads a datum fixed
  by a centre there. The arm is right. Bits: see MINOR-3. Masking: see
  MINOR-2.
- **(b) `unit_sign`.** Defensible either way. `R·part` is literally
  `dC/dt·along`, the old row's "speed". It is also a tangent turn at the
  circle's arm. Since the datum is the centre, the arm is R from both above
  and below, so the claim reading is satisfied. `unsure` whether "a rate
  through `levered`" (which the README forbids) is the truer reading.
- **(c) spiric.** It reads from below for a sign claim, per the README. The
  extent is computed before the kind dispatch, for plane caps and for the
  unreachable arms too (style). See MINOR-1.
- **(d) β.** True: the `atan2` argument's modulus is at least `1 − |β|`.
  The precedent `chart_singularity::below` (`:623`) exists, and `sign_of`
  has 15 callers.
- **(e) `off_axis`.** True. The elevation of the rim off the cone from a
  radial centre offset ρ is at most `ρ·cos α`. It is bit-identical over
  4381 decisions.
- **(f) the joint room.** See MINOR-4.
- **(g) the bisector.** From below, at the nearer reach, is right for a
  claimed Positive. See NOTE-2.

### C4, the mutants

The re-plants, every one red as claimed:

| Site | Mutant | Red on |
|---|---|---|
| joint room | old arm | the sphere row |
| joint room | pad dropped | both rows |
| joint room | `curvature_lever_arm` | the sphere row |
| joint room | half pad (mine) | both rows |
| bisector | unit arm | `bisector_rows` |
| bisector | far reach | `bisector_rows` |
| bisector | session box | `bisector_rows` |
| bisector | `BoolSector::arm` | `bisector_rows` |
| β | old arm | the new β row |
| β | run band | the new β row |
| β | session box | the new β row |
| β | `abs` dropped (mine) | the new β row |
| `off_axis` | cos dropped | `iso_parse_incidence::…` and `props_cone_incidence::…` |
| `off_axis` | sin for cos | `iso_parse_incidence::…` and `props_cone_incidence::…` |
| `off_axis` | ×1e-3 (mine) | `props_cone_incidence` only |
| `unit_sign` | minor radius (and both parts at `r`, mine) | `a_derived_circle_image_is_on_its_circle_where_its_azimuth_shows_most` and `a_tube_rate_gates_rejected_zero_is_a_class_reading` |

Mine beyond those, over the whole set:

| Site | Mutant | Result |
|---|---|---|
| tilt | `r` | survives |
| tilt | ×1e-6 | survives |
| tilt | ×1e6 | red |
| spiric | old R | survives the suite, red on my probe |
| spiric | arm 1e-9 | 15 red |
| spiric | session box | survives 6488 |

So the two "unobservable" sites:

- **The spiric** is observable. A row is owed: MINOR-1. The session-box
  arm also survives the whole suite, so the unsound side (an arm too long
  for a sign claim) is unpinned too.
- **The tilt** is masked toward the unsound side and unpinned: MINOR-2.

### C6, C7

**C6: true.** Ten `No door names it yet` lines name the row:

- dihedral, 2;
- intersect, 5;
- fate, 1;
- join, 1;
- validate, 1.

The row's four kinds match the code: Kε·sinθ in `(ε, Kε)`; `2ε·cot ψ` and
`cot ψ ≤ ½`; `K·ρ ≤ ε` ⇔ `ρ ≤ ε/K`. Its facts list also checks out:

- `padded` and `less` take `f64`;
- `linear_at` is `doc(hidden)`;
- the `chart_region.rs:1012` precedent exists.

One tension (`likely`): kind 3's intersect quotients are a stand-off length
times a dimensionless factor. The row says no door names that, while this
PR itself put `off_axis`, the same shape, through `Margin::of`. I am not
re-opening the design; I note that the row's "no existing door names it" is
contradicted by the PR's own treatment of (e).

**C7: true.** The three sites exist as cited:

- `topo/pcurves/decision.rs:1543`;
- `geom-brep/pair_section/ruling.rs:1625`;
- `topo/chord_join/wall_ring_rows.rs:777`.

### Stale text at the converted sites

These are doc honesty and Q4 findings, all BY-INSPECTION, `sure`.

- **`pcurve_cache.rs:13797-13798`.** `spiric_chart_pcurve`'s doc still says
  the sense is "the cosine of the chart axis against the carrier's
  **levered at the chart's major radius** (`pcurve_spiric_chart_axis`)".
- **`crates/topo/src/boolean/fate.rs:546-547`.** `sign`'s pin comment still
  says "a unit arm (every caller's `1`) on a unit vector's length or a count
  of half-turns". The unit vector's length is gone: `bisector` no longer
  calls it, and only `tip_wraps`' two half-turn calls remain.
- **`pcurve_cache.rs:13878`.** "within `K·ε` over the arc's extent of
  perpendicular" is garbled English, and false for short arcs (MINOR-1).

## Style

**Questions exercised:** Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was only partly
exercised: I read `room_fence_tests.rs` whole, and `fate.rs` around
`sign`/`bisector`, `decision.rs`'s `axis_distance` and `joint_has_room`, and
the touched `pcurve_cache.rs` functions in full. I did not read the 18 076
lines of `pcurve_cache.rs` end to end.

- **Q1: (e) and (f) treat one shape two ways.** `curved.rs:2711` against
  `decision.rs:1883`, same as MINOR-4. A residual times a constant factor
  goes in the margin at one site and through a curvature arm at the other.
  This is one PR, so it is the trap the lane names: a unit that closes
  "numbers under the wrong door" mints a new one. `likely`.
- **Q1: two spellings of "the vertex's distance from the axis" in one
  function.** `decision.rs:1872` and `:1886`. `axis_distance(chart, vertex)`
  survives only as the `is_none()` guard; `parallel_curvature_arm` reads the
  value. On a cylinder they differ: the stored radius against the measured
  point distance. The guard's set (Plane, Nurbs, Approx → None) is kept in
  step with `Surface::axis_line`'s by nothing. `likely`.
- **Q3: the bisector row's refusal case sits on the zero threshold to about
  three ulps.** `fate.rs:2616`. The case reads `9.999999999999996e-10`
  against zero `1e-9`. One ulp up, it escalates: `corner_refused` gives a
  `Coincide` error, not `ClassificationInvariant`. The row then goes red for
  no semantic reason, and the in-band outcome itself is pinned nowhere.
  (The cone room row's `2e-9` against the padded zero `2e-9` is also on the
  edge, but there both outcomes assert `!room`.) `sure`.
- **Q3: the β row pins only modified βs on drawn sections.**
  `closed_image_rows.rs:1536`. No row states a genuine very thin section,
  the case the old gate actually refused and the new one passes. "Toward
  the truth" is argued, not pinned. `likely`.
- **Q6: the tilt-masking claim rests on a measurement held by no row.**
  `pcurve_cache.rs:13087-13108`; see MINOR-2. `sure`.
- **Q5: `Margin::sign_of`'s doc names only a certified volume's enclosure
  (D10).** `crates/geom-core/src/predicate.rs:1189-1196`. It now has 15
  callers that read other quantities; this PR adds one (β), which is a
  stored f64 coefficient, not an enclosure. The precedent is real but the
  door's doc no longer says what it is for. Pre-existing, made one wider.
  `likely`.
- **Q7: the spiric extent is computed for every chart kind.**
  `pcurve_cache.rs:13828`. It runs ahead of the `let … else unreachable!`
  destructure and the match, so plane caps and the refused
  cylinder/cone/sphere arms pay a 17-point farthest-pair schedule they never
  read. I would have read it inside the torus arm. `likely`.
- **Q2: the spiric argument postdates its code.** `pcurve_cache.rs:13876-13883`.
  The argument for why an in-band axis read is "definitely off" was edited
  to fit the new arm ("over the arc's extent") without re-checking that it
  still holds. MINOR-1 shows it does not. `sure`.
- **Q4: sibling prose citing the old arms.** These are still accurate
  enough: `pcurve_cache.rs:13029` ("each levered at `R`") and the test doc
  at `:16697` ("mid-band off perpendicular", now relative to an arc
  extent). `unsure`.

**Class to sweep:** sites converted to a geometric arm whose old argument
text was edited in place rather than re-derived. Look at the remaining ten
when the later unit converts them.

## Ratings

- **Idiom/structure: 3.** The doors chosen are real and mostly right. But
  one PR applies two treatments to one shape ((e) against (f)), and the
  spiric arm is read before dispatch.
- **Test quality: 3.** Every claimed mutant is killed, and the joint-room
  and β rows discriminate well. But the bisector row sits on an ulp edge,
  and two decisions (the tilt, the spiric arm) stay pinned by nothing.
- **Doc/comment honesty: 3.** The row corrections are careful and mostly
  true. But two stale arm descriptions remain at converted sites, and three
  claims are false as measured: bit-identity, "no verdict reads the
  difference", "unobservable".

## Artifacts on this branch

All under `analysis/oneround/unsourced-stand-ins/`:

- `probe-spiric-short-span.patch` and `.out`: the MINOR-1 probe, a test
  appended to `closed_image_rows.rs`;
- `tools/k_stats-site-log.patch`: the differential instrument;
- `tools/diff.sh` and `tools/diff2.sh`: the runners, seed 0x7268;
- `tools/cmp.py`: per test × site verdict and bit comparison;
- `tools/mutants.py` and `tools/mutrun.sh`: the mutant table and its
  runner.
