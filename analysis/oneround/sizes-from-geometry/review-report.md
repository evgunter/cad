# Review: PR 7628, "sizes read from their geometry, not back out of a lever"

Unit `work/oneround/sizes-are-read-back-out-of-extent-levers.md`, frozen head
`fc2bf2d7cf12a607478664460f8e94c960231e51`, merge base `6246f581b7`.
Reviewer branch `oneround/sizes-from-geometry-review`; evidence under
`analysis/oneround/sizes-from-geometry/evidence/`, the instrument and every
mutant as one patch on the frozen head
(`analysis/oneround/sizes-from-geometry/instrument-and-mutants.patch`).
No helper agent was used.

## Verdict: APPROVE-WITH-FIXES

One fix is required before merge (M1: the frozen head reds the rustdoc gate
that CI's `lint` job runs). Everything else the unit claims about numbers
holds: every converted read except the two slabs is the same bits as before,
by construction and by my own instrument, and the slabs are read by nothing
on the lane they are built for. What I found beyond M1 is in what the tests
and the PR body do not hold, and in what the sweep waved through.

## What I executed, and what I only read

Executed (all at the frozen head unless a base is named; one target
directory per tree):

- `cargo nextest run --profile ci -p geom-brep -p topo -p editor-core -p sweep`,
  default ε: **12 855 / 12 855 passed** (2 523 skipped by the profile). Judged
  against the body's "12 855/12 855 after merging main": agrees.
- The same four crates on the instrumented tree at default ε (twice) and at
  `CAD_TOLERANCE_EPS=1e-12`. Default: 12 855 / 12 855. 1e-12:
  **12 853 / 12 855**, two reds (N2).
- Both 1e-12 reds on the merge base `6246f581b7` (a separate worktree and
  target directory), and the second also on the clean frozen head.
- `GATE_BASE=6246f581b7 scripts/run-gates.sh`: exit 0, 67 `OK` lines. Judged
  against the body's "exit 0, every gate OK (67 lines)": agrees.
- `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` (the command
  `ci.yml`'s `lint` job runs): **exit 1** (M1).
- `cargo fmt --all -- --check`: clean.
  `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`: clean.
  `python3 scripts/work.py lint`: ok.
- The typed sweep (`Lever::length` marked `#[deprecated]`):
  `cargo check --workspace --exclude viewer --lib --bins`, then
  `--all-targets --all-features`.
- 27 mutants, each armed alone by an environment switch in one build, each
  run over the tests my instrument saw reach its site (the table in C7).

Only read: `conic_bend.rs`, `intersect.rs`, `vtxfac.rs`, `bound_side.rs`,
`star/walk.rs` (the sites left); `ssi.rs`, `ssi/certify.rs`,
`ssi/implicit_chart.rs` (the slab's and the extent's consumers);
`line_quadric.rs` (`line_cone_roots`); `face_cover`'s cone and cylinder arms.
Not re-taken: ε = 1e-6 and 1e-9; `demos/`; the excluded roots were searched
by text only (no `.length()` call in any of them).

## The claims

**C1 (`min_radius_of_curvature_toward`): holds.** The old `_` arm caught
exactly sphere/cylinder/cone/torus `Inner` and both spline kinds;
`curvature_lever_arm(..).length()` gave the radius, the parallel radius at
`p`, the tube radius, poison. The new arms give the same expressions, and
the match is now exhaustive. Instrument: 15 691 reads at `f64` (419
processes), 172 at `Interval` (10), 19 at `Dual<f64>` (2), 0 differing; the
same at 1e-12 (15 662 / 170 / 19). The three consumers
(`min_radius_of_curvature`, `sectors.rs:785`, `solid_contain.rs:3677`,
`battery.rs:660`) get the quantity they got. Mutants in the unsound
direction (a radius twice too large on the cone, on the sphere and
cylinder, on the torus) are each red; the cone read at the apex is red on
91 tests of the reaching set. The cone's `ρ` being local to `p` while the
function's doc calls the bound global is older than the unit and stated in
the doc. The new door is in the Style section (Q1).

**C2 (the partial cone and wall): the numbers hold; the tests reach a
quarter of it.**

- (a) The consumer, read in the code. `face_holds`: on the carrier and not
  `Positive` refuses; `Positive` goes on as "q is off this face".
  `ray_crossings`: `Positive` drops the root, `Zero` grazes (the next ray),
  `Negative` blocks (a refusal the walk outlives). So the one sign acted on
  as a claim is `Positive`, on both rows and both arms, and a ball that
  holds the face (from above) is the bound it is owed. BY-INSPECTION, sure.
  The window `run_on` hands `line_cone_roots` also levers that door's lead
  rung by the run's length, and that rung's non-Zero is acted on; it did so
  before the unit with the same number. BY-INSPECTION, unsure that this is
  a defect, and not the unit's either way.
- (b) `face_ball_about` is `face_reach_from`'s body with `.lever()` taken
  off, and `face_reach_from` now levers the ball: the dependency runs the
  right way. The cone arm's cover is the convex hull of the boundary edges'
  covers (and the apex where a loop winds the axis), so a curved edge that
  bulges past its vertices is held by its own cover; a face with no cover
  is `CorruptFace`. Unchanged by the unit. BY-INSPECTION, likely.
- (c) `stand_off` and `run_on` are readings of the region (a signed
  distance; a parameter window that over-states the chord). Neither hands
  back the radius, but `ExtentBall::radius()` is public and older, so a
  caller could always take the length. `run_on` has a near-twin in the same
  file (Style Q1).
- (d) The wall: m1 below.
- The record. `FaceGeo` is `pub(crate)`; `PartialCone` and `PartialWall`
  are matched only in `solid_contain.rs` (two readers, two wildcard arms).
  The record held the centre already (`axis.origin`; `anchor`) and the
  radius (a `Lever`; a bare `T`), built by the same call in the same
  function. No reader gains a view of geometry it did not hold, and no
  module comes to read a region. This is the innocent kind. The condition
  on `unsourced-levers-await-a-source` asks the PR to say "what now flows
  where"; the body says what the record holds and does not make that
  statement (N5).

**C3 (`Link::of`): holds.** Every wedge's segment distance is at most its
farther end's, and `reach` is the max over all ends, so the old fold's
identity never won except on a zero-length wedge, where the chord is poison
and `Real::min` propagates NaN either way. Instrument: 5 274 reads at `f64`
(312 processes), 504 at `Interval` (32), 69 at `Dual<f64>` (3), 0
differing. The no-wedge arm was read 0 times (m5).

**C4 (the slabs): the claim "from above" is true and unread.**
`SsiDomain::half_extent` has three uses in `geom-brep`: `check()`,
`slab()`, and `pass_key`. `slab()` is called only by the ℝ³ marches
(`march_cylinder_sphere`, `march_cylinder_cylinder`, `implicit_pair_r3`,
`march_seeds`, `finish_r3`, `idealized_trace_r3`); `ImplicitSection::of_charted`
and `Section::of_charted` never call it. So no section point can leave the
new slab, because nothing asks. The geometry is right as far as it goes: a
wall with positive weights lies in its net's hull, the cube of that
half-width about the pivot holds the ball, and a pair's section lies in
both walls, so the lesser reach bounds it. The pass key cannot newly
collide: the new half-extent is a function of the wall and the centre,
both already in the key, and `offset_derive` banks no pass. Measured: 24
plane reads (6 processes) moved from 5.58 to 8.00 down to 2.00 to 3.00; 42
pair reads (8 processes) moved from 1.0 or 1.0857 up to 2.06 or 2.19; the
positive-finite check agreed old and new on all 66 (70 at 1e-12). See m2.

**C5 (the session box): same bits; one site means a region.**
`Lever::session_box().length()` is `T::from_f64(SESSION_BOX_EXTENT)`
through a `repr(transparent)` newtype. Instrument at `box_decided`: 63 052
reads at `f64`, and reads at `Interval`, `Dual<f64>`, `Dual<Interval>`,
`Sym<f64>` and `Sym<Interval>`, 0 differing. The constant is the box's
definition and the right source for `box_decided`. `mate/value.rs` is m6.

**C6 ("nothing moved"): holds, with a reach account that hides two
things.** My counts above agree with the body's process counts almost to
the unit (419/10/2 against "~418/10/2"; 312/32/3 against "~311/~32/3";
6 and 8; 2 and 2). The body's "partial_stand_off 2, partial_run 2" and its
gap ("not reached on the Interval lane: the partial-cone and partial-wall
arms") are m1.

**C7 (the mutants).** Every claim re-planted came out as claimed:

| mutant | result | where |
|---|---|---|
| M1 `stand_off`, radius added | red | both door rows, both sweep rows |
| M2 `stand_off` from the origin | red | the two door rows only |
| M3 `run_on` centred on the line's origin | red | its door row only |
| M4 `run_on` half-run from the distance | red | its door row only |
| M5 `face_ball_about` = the cover's own ball | red | the two cap rows |
| M6 `face_ball_about` at the near distance | red | 8 tests, the two sweep rows among them |
| M7 `net_reach_from` ×2 + 1 | red | its row only |
| M8 `net_reach_from` from the net's centre | red | its row only |
| M9 partial cone's ball about a boundary point | survives | |
| M10 cone read at the apex | red | 91 of 495 |

M9's reason is true: the stand-off and the run are sound for any ball that
holds the face. "Bit-identical by construction" is true of the lever put
back at all four sites, and after the unit none of the four follows the
lever's source any more (the partial balls now feed the lever, not the
reverse), so a later change of a lever's source no longer moves this
geometry. No row would go red if someone re-coupled them, and none can by
value.

Mine, in the unsound direction:

| mutant | result | reaching set |
|---|---|---|
| partial cone's ball halved | red (the two sweep rows) | 152 |
| **partial wall's ball halved** | **survives** | 152; the site is never built (m1) |
| cone ray's stand-off forced Positive | red (the two sweep rows) | 152 |
| cone ray's stand-off forced Negative | survives | 152 (N4) |
| `run_on` half-run halved | red on its door row only | 152 |
| **plane slab × 1e-6** | **survives** | 49 |
| **pair slab × 1e-6** | **survives** | 49 |
| **pair slab: the greater reach** | **survives** | 49 |
| plane slab NaN / pair slab NaN | red (4 / 6) | 49, proves the reach |
| cone / sphere+cylinder / torus radius ×2 | red (3 / 5 / 3) | 495 |
| link ball without the clears | red (1 row) | 344 |
| **link ball × 2** | **survives** | 344, every test that reads it |
| session box × 2 in `box_decided` | red (17) | 733 |
| **plane mate's box extent halved** | **survives** | all 3 155 of editor-core |

**C8 (the sweep).** My typed sweep at the frozen head finds **90** call
sites in lib and bins, file by file the body's list less its ten F sites
(`evidence/typed-sweep-lib-bins.txt`); `--all-targets --all-features` adds
144 hits, every one in a test module or a `*_rows.rs` / `*_fuzz.rs` file,
so the code under `cfg` features holds no production call the list misses.
I read 46 of the body's M hits in 26 files of six crates
(`replay.rs` 385; `dihedral.rs` 975, 1042, 1105, 2242; `dihedral_side.rs`
353, 673; `edge_nurbs.rs` 1200, 2152; `enters.rs` 530, 647, 806;
`extent.rs` 1060, 1149, 1175; `locus.rs` 580; `torus.rs` 148; `march.rs`
1216; `ssi.rs` 286; `rigid_motion.rs` 2152; `unit_vec.rs` 583, 623;
`vtxfac.rs` 304, 872, 2332, 2442; `bound_side.rs` 726, 861, 1158;
`ring_path.rs` 1648, 1770, 1837; `spline_wall.rs` 163; `battery.rs` 992,
1678; `section_cert.rs` 1255, 1326; `link.rs` 260; `offset_derive.rs`
1433, 1540; `solid_contain.rs` 3686; `build.rs` 786; `rules.rs` 219;
`walk.rs` 407; `fan.rs` 454; `containment.rs` 1527). 44 are a margin's
business as the body says: a positivity gate, a hand-multiplied levered
charge, a refusal's payload, a lever handed to a lever. Two are not, and a
third hit of the body's list is the same call (m3).
`two_planes` is a hand-levered sine (`|n1 × n2| · lever`), a margin's. The
five rows are true of the tree at every site I checked (all of L2, L3, L4
and eleven of L1's). Their banding is m4.

**C9 (the body).** True of the frozen head except N1, N2, m1's reach
account and M1's omission. The gates print what the pin files hold (exit 0
against the merge base).

## MAJOR

**M1. The frozen head fails the rustdoc gate CI's `lint` job runs.**
DEMONSTRATED, sure. `crates/topo/src/solid_contain.rs:874`, the doc of
`FaceGeo::PartialCone`, still links ``[`partial_reach`]``; the unit renamed
that function `partial_ball` (line 1158).
`scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` exits 1 with
exactly one rustdoc error, `unresolved link to partial_reach`
(`evidence/doc-gate-excerpt.txt`); `ci.yml` runs that command at lines 348
and 795. Clippy is blind to it, and the body's "Ran locally" list has no
doc gate. It is a one-word fix, and it is the only thing between this head
and a green `lint`.

## MINOR

**m1. The `PartialWall` conversion is reached by no test on any lane, and
the body's reach account says otherwise.** DEMONSTRATED, sure. Over the
four crates' `ci` profile at default ε and at 1e-12, `FaceGeo::PartialWall`
was constructed 0 times (`PartialCone` 80 times, in 2 processes, at `f64`).
Reads per site: the cone's ray stand-off 308 (28 raw-positive, 280
negative) and `run_on` 340, both in those 2 processes; the cone's
`face_holds` stand-off 0; the wall's `face_holds` stand-off 0; the wall's
ray stand-off 0. A wall ball of half the radius, which does not hold the
face, survives. The body reports "partial_stand_off 2, partial_run 2"
under "reached at every ε" and names the gap as "not reached on the
Interval lane: the partial-cone and partial-wall arms". The truth is that
three of the five converted reads in `solid_contain` (site 5 whole, and
half of site 2) are unpinned on every lane. The conversion there is
correct by inspection (the same centre, the same radius); nothing would
say so if it were not.

**m2. The slabs' new geometry is held by nothing at its two call sites.**
DEMONSTRATED, sure. A plane slab or a pair slab a million times too small,
and a pair slab read as the greater of the two reaches, each pass all 49
tests of `offset_derive` and the `offd` suites; the NaN mutants prove
those tests reach both sites. The new row pins `net_reach_from` alone.
That is because nothing on the chart lane reads the number (C4), which the
body discloses; what it leaves standing is a site comment that says "The
window holds the wall" (`offset_derive.rs:1236`) over a field whose own doc
says "on a chart lane, which a wall's rectangle bounds, only checked"
(`ssi.rs:2093`), and a "bound: from above" that no behaviour can confirm
or refute. One refusal does move, unreached by any test: a net with a
non-finite point used to be dropped by the `f64::max` fold and now makes
the half-extent NaN, so the pair is refused as
`DomainUnusable { field: HalfExtent }`, which names the slab for the
net's fault. BY-INSPECTION, likely.

**m3. Three "margin's business" hits size a tube, and the body did not
follow them** (I read two in my sample and the third after). BY-INSPECTION, sure of the code, likely as a finding.
`edge_nurbs.rs:1200`, `:1865` and `:2152` hand `extent.length()` (and
`edge_extent_inf(..).length()`) to `certify_branch`, where
`let widest = Bounds::hi(extent)` feeds
`tube_ladder(widest, band)`: `r = SSI_TUBE_RADIUS_MAX * extent / 2^k`
(`ssi/certify.rs:172-178`, `:2589`). That is a lever's length used as the
radius ladder of the certified tube, "a domain's size" in the row's own
words. Every rung is proved, so nothing is unsound, exactly as with the
slab the unit did convert. The body lists the three as M and says in the
same section that it did not follow "the `certify` limbs". The same holds
of `offset_derive.rs:1242` and `:1345`
(`SsiDomain::extent`, which the body calls "the lever field"): on the
chart lane it also sizes the march's idealized step
(`ssi/march.rs:1320`, `SSI_IDEALIZED_STEP * ctx.extent`). None of this is
on a row the unit filed; `unsourced-levers-await-a-source` knows
`SsiDomain::extent` as a field, not the tube ladder.

**m4. The five rows the unit files carry no band and no cost, and one of
them misfiles two sites.** BY-INSPECTION, sure. `work/README.md`: "Every
row should carry a band"; an unbanded row "sorts after P4". None of
`pierce-and-rim-balls-are-built-from-an-arm`,
`ray-crossings-read-the-selections-lever-as-the-rays-window`,
`a-levers-length-used-outside-a-margin-has-no-gate`,
`sector-bounds-pack-a-direction-and-its-lever-into-one-vector`,
`apex-reaches-read-a-lever-as-a-window-along-the-axis` has `priority` or
`cost`, so what the unit discloses as left is scheduled behind everything.
The sector-bounds row states an open three-way design fork and carries no
`design: true`. And two of its sites are not waiting on that fork
(likely): `link.rs:2897` (`face_reach_from(..).length()` as the fan's
`far`) and `census.rs:9615` (`.map(Lever::length)`) are a face's far
distance read back out of a lever, the shape of the unit's own site 2, and
the unit minted the door that serves them (`face_ball_about`).

**m5. `Link::of`'s radius is unpinned upward, and its new no-wedge arm is
dead under the suite.** DEMONSTRATED, sure. A link ball twice too large
passes all 344 tests that read it; only dropping the clears is caught
(one row). The `map_or_else(T::zero, ..)` arm was read 0 times in 5 847
reads. The gap is older than the unit; the unit rewrote the expression,
states "bound: none, it is the ball's radius", and adds no row for it.
The new arm also differs from the old on one input, by inspection: no
wedge and a cleared sector whose `clear` is poison read NaN before and 0
now (unsure that such a sector can exist).

**m6. The plane mate's site means a region and ends as an arm, and no test
holds its size.** DEMONSTRATED (the survivor), sure of the code.
`mate/value.rs:1648-1651`: `3f64.sqrt() * extent + origin` is the session
cube's far corner from the frame's origin, a region's far distance spelled
by hand, and it goes straight to `arm_of`, which makes an `Arm` of it. So
the old read was a lever's length handed on to a lever, and the new one is
a bare constant handed to a lever: the bits are the same and the question
"is a constant the right source" has the answer "for `box_decided`, yes;
here the box is a region read about a point". The site is read 11 times by
3 editor-core tests, and halving the extent (an arm shorter than the
region it governs) passes all 3 155.

## NOTE

**N1. "90 call sites" is the count at the frozen head, not of the list it
heads.** DEMONSTRATED, sure. The typed sweep gives 90 at the head. The
body's list is "at the merge base" and includes the ten F sites, so it
holds 100 calls (98 line references, two lines with two calls each:
`snap.rs:296`, and `validate.rs:9081` at the head). The dispositions are right; the
total is one tree's and the list another's.

**N2. At 1e-12 the frozen head has two reds, both the merge base's; the
body names one, with a margin that is no longer its.** DEMONSTRATED, sure.
`sweep a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`
is red on the merge base and on the head with the same payload
(`props_chart_junction_closed`, margin `2.813857544654365e-12`); the body
quotes `2.8154559647670245e-12` "to the bit", which was main's before the
merge. `geom-brep pcurve_cache::ruling_reading_rows::a_true_ruling_keeps_the_closed_form_on_both_nappes`
is red at 1e-12 on the clean head and on the merge base with the same
payload (`pcurve_envelope`, hi `1.1122737889672985e-12`), and is filed as
`work/green/main-red-a-true-rulings-pinned-escalation-window-is-missed-at-1e-12.md`.
The body's 1e-12 run predates the merge of main; after it only default ε
was re-run.

**N3. Two premises of the brief.** "C2 and C4 change numbers": C2 changes
no number (`Lever` is a transparent newtype and the ball is built by the
call that built the lever); only C4 does. "Converts four of them": the
body says five of the row's ten (it counts `solid_contain` twice, as the
row does), which is what the tree shows.

**N4. No test needs the one verdict the bound exists for.**
DEMONSTRATED, sure. With the cone ray's stand-off forced Negative (every
root blocks, never "outside"), all 152 tests pass. So the soundness of
`Positive` is pinned (the halved ball and the forced Positive are red),
and that the door ever answers past an untrimmable cone is not. Likewise
M3, M4 and my halved run are red only on `run_on`'s own row: the window's
use in `ray_crossings` is pinned by nothing but the door.

**N5. The records condition.** BY-INSPECTION, sure. The change is the
innocent kind (C2). The body does not state it against the condition
("which readers gain a view of geometry they did not hold before"), which
that row asks of a unit that changes what a record holds.

**N6. The unit's own row is unchanged by the diff.** The corrections to it
(the chain behind `BoundSupport::point`, `PartialWall` as an unnamed
sibling, the four siblings found) live only in the PR body. Sure.

## Style

Questions exercised: Q1 (prose sweep and a data sweep over the touched
files, and the whole method list of `extent.rs`), Q2, Q3 (by mutant), Q4,
Q5, Q6, Q7. Q8 only in part: I read `curvature_arm.rs`,
`cover_reach_rows.rs` and the non-test half of `session_box.rs` whole,
`topo/src/cover.rs` to line 440, and of `solid_contain.rs` (5 829 lines,
the largest touched) the module doc and the touched regions, not the file
end to end.

- **Q1. `run_on` is `Reach::range_along`'s ball arm under a second name.**
  `extent.rs:257` against `extent.rs:1080-1086`. For a unit direction,
  `Reach::Ball(b).range_along(line.origin, dir)` is
  `(c − r·|dir|, c + r·|dir|)` with `c = (centre − origin)·dir`, which is
  `run_on`'s `(param(centre) − r/|dir|, …)`. The two differ only in how
  they treat a non-unit direction. The filed apex row itself names
  `Reach::range_along` as "the window along the axis". Sure.
- **Q1, the trap the brief names: the diff writes a fresh copy of the read
  it gives a door.** `solid_contain.rs:3464-3469`. The unit lifted
  `(line.param(apex), reach / |dir|)` out of `cone_door` into
  `ExtentBall::run_on`, and in the same commit inlined the same two lines
  at `cone_door`'s other caller, the trimmable cone arm, over
  `region.reach()`. That value is `RuledFaceRegion::scale`, documented at
  `ruled_region.rs:338` as "the lever of a face's crossings" and held as a
  bare `T`, here read as the radius of a ball about the apex. It is an
  instance of the unit's own class, written by the unit, invisible to a
  sweep keyed on `Lever::length`, and on no row. Sure of the code; likely
  one of several: I would look at every bare `T` named `reach`, `scale` or
  `arm` (`splitting/pieced_arc.rs:44` `PieceBall { center, reach }`,
  `curve_pair/span.rs:943`, `chart_singularity.rs:800`).
- **Q1. `net_reach_from` is `ExtentBall::of_cover_from(net, at).radius()`
  by hand**, and the seventh `Cover::net(chart.control().to_vec())` in
  the tree (`offset_derive.rs:1201`; `face_normal.rs:826`,
  `sector_face.rs:509`, `dihedral.rs:972`, `dihedral_side.rs:1272`,
  `spline_implicit.rs:228`, `spline_pair.rs:133`), beside the carrier's
  own reading at `geom/src/cover/readings.rs:319`. Sure.
- **Q1. `parallel_radius` and the radius table.** `curvature_arm.rs:97`
  and `implicit.rs:413-418`. The direction is now right (the lever is
  built from the radius), but `min_radius_of_curvature_toward` restates
  per kind the radii `uniform_curvature_arm` holds (sphere and cylinder
  `radius`, torus `minor_radius`), and only the cone goes through the
  home, because `cover-kind-match` caught that arm alone. Two tables of
  one fact; the doc at `curvature_arm.rs:35-41` warns the two functions
  must not be swapped and nothing ties their shared rows. Likely.
- **Q1. The wall keeps a second window.** `solid_contain.rs:4210`.
  The `PartialWall` arm has its ball in hand and `run_on` beside it, and
  still hands `line_wall_roots` the selection's `reach` (a lever's
  length), where the cone arm hands `ball.run_on`. Filed
  (`ray-crossings-read-…`), so disclosed; the unit leaves the two sibling
  arms it converted reading their windows two ways. Sure.
- **Q2. A comment claims what the field's own doc denies**
  (`offset_derive.rs:1236-1238`, m2), and one does not parse:
  `solid_contain.rs:3461-3464`, "The face lies within its reach of the
  apex, never under it (…), so off the run centred on the line's closest
  approach to the apex and that long either side." Sure.
- **Q3. The four new rows can each go red both ways** (exact values; the
  cap row asserts held-within-1e-12 and out-by-0.4). What cannot go red is
  listed in m1, m2, m5, m6 and N4: the rows pin doors, and the conversions
  at the doors' consumers are pinned for the cone's ray arm alone.
- **Q4. The rename left one citer** (M1). I grepped `partial_reach`,
  `face_reach_from` and `reach:` over `crates`, `scripts`, `docs` and
  `work`; the ledger line and every other doc moved with it.
- **Q5. `curvature_arm.rs`'s module doc** says the file is the home of
  "a lever read off a carrier's own curvature" and that each reader takes
  the carrier; it now also exports a bare length. Small, sure.
- **Q6. "Nothing moved, measured" is a measurement with no guard.** The
  instrument is uncommitted and no row or register re-takes it; for the
  five same-bits sites none is owed (the identity is structural), and for
  the slabs the claim that the check passes old and new rests on 66 reads.
  Unsure that anything is owed. The rows without bands are m4.
- **Q7.** I would not have computed a hull distance for a field nothing
  reads; the honest alternatives (say so at the site, or stop building the
  slab on the chart lane) are a design question, and the body's own words
  ("only checked") make the case. Two public methods on a public type
  (`ExtentBall::stand_off`, `run_on`) serve one private file. Taste,
  unsure.

## Ratings

- **Idiom and structure: 4.** Each conversion is the smallest change that
  moves the read to its geometry, and `face_reach_from` now derives from
  `face_ball_about`; marked down for the inlined copy of `run_on` and the
  `range_along` twin.
- **Test quality: 3.** The four rows pin their doors in both directions and
  every claimed mutant is red, but six of my mutants in the unsound
  direction survive, and one converted sibling is never built by any test.
- **Doc and comment honesty: 3.** The PR body's measured counts reproduce
  almost to the unit, which is rare; against that, a stale link reds a
  gate the body does not list, the reach account hides the wall, and the
  sweep's total and the 1e-12 margin are each one tree out.
