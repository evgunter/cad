# Review r1: PR 7203 (`oneround/anchored-conic`) at `8d59783007a93f62c21f3581dc651c8d70c81caf`

Reviewer lane r1, ONEROUND. The unit is `git diff ec8749552 8d5978300`,
stacked on `onehome/carrier-frames` at `ec8749552` (that branch has
since moved to `7fddccc6a`; I reviewed against `ec8749552`, as the brief
says). Probes, oracle, mutant set and run logs' summaries are on this
branch under `analysis/oneround/anchored-conic/review-r1/` and in
`crates/geom-brep/tests/review_anchored_conic_r1.rs` (all `#[ignore]`).
I did not look at the other reviewer's branch or any summary of it.

## Verdict: APPROVE-WITH-FIXES

The core of the unit holds up under an independent oracle. A minted cone
section's points near its anchor are right at their own scale. The
running bound held at every point I checked, re-seamed, reversed and
half-turned ones included. The anchor's bits survive every re-seam and
reversal, and the local suite reproduces at head exactly as the PR
reports, red for red against the base. Nothing I found is MAJOR.

What needs fixing before merge:

- **The re-seamed anchored path is mostly unpinned by the unit's own
  rows.** Six of my twelve mutants survive every anchored row in
  `geom`, `geom-brep` and `mesh`. Against the whole `ci` suite,
  pre-existing cone rows in `mesh` and `sweep` catch two of them
  (`map_scalar` dropping the phase, the azimuth ignoring it). Three
  survive all 17,467 rows: `same_content` ignoring the form, `data()`
  omitting the phase, and `zero_point` returning the anchor (M1).
- **Two pieces of the PR's evidence are wrong.** The "centre-stored base"
  widths in the C3 table, which the closed row repeats, are not what
  `ec8749552` gives. The gate account says two red; there are three.
- **Several comments now describe code that is gone, or no longer hold
  for an anchored conic.**

## What I executed and what I only read

Executed (local debug builds; own target dirs `/home/user/tgt-head`,
`/home/user/tgt-base`; base worktree at `ec8749552`, head worktree at
`8d5978300`):

- `ci` nextest profile, whole workspace, base and head, at ε = 1e-9
  (default) and 1e-12.
- The two cone-ring slow rows (`sweep a_ring_on_a_cone_face::the_rings_volume_brackets_*`)
  with a width print (`review-r1/ring-width-print.patch`): base and head
  at default ε, head also at ε = 1e-6.
- `scripts/run-gates.sh` on clean base and head worktrees.
- Probe `dump_far_cone_sections`: 48 far-centred `plane_cone_section`
  ellipses (cone 1e-2 … 1e-7 short of parabolic), each also re-seamed a
  quarter turn, a half turn, and re-seamed then reversed. 27 parameters
  per carrier: near the anchor's anomaly, a quarter turn from it and a
  half turn from it. 5,184 points. Each point is dumped as `f64`, with
  its running bound and its `Interval` enclosure.
- An exact-decimal oracle (`review-r1/oracle.py`, 70 digits, Taylor
  sines). It computes the stored formula exactly from the stored bits,
  and each point's miss off the exact plane and cone.
- Probe `the_arc_cover_holds_the_arcs_points` (C7): an anchored ellipse
  and the centre-born ellipse at its derived centre, `a` = 1e2 … 1e7,
  windows ±1/a and ±10/a, 201 samples × 5 directions, at `f64` and at
  `Interval`.
- Probe `re_seams_keep_the_anchor` (C3).
- Twelve mutants (`review-r1/mutants.py`, `run_mutants.sh`), each against
  the anchored rows of `geom`, `geom-brep` and `mesh`. The six that
  survived were then run against the whole `ci` suite at ε 1e-9, as
  three nested sets (6, 4 and 3 mutants) to attribute the reds (C9).
- A mechanical audit of the merge's production rebinding
  (`review-r1/shadow_audit.py`, C10). Greps for C4 and C5.

Only read: `CLAUDE.md`; `docs/prompts/reviewer-style-lane.md`;
`crates/geom-core/README.md`'s "Float rounding"; both unit rows; the PR
body; `conic.rs` whole; the `curves.rs`, `intersect.rs`, `implicit.rs`,
`carrier_resolution.rs`, `in_band_sweep.rs`, `transform.rs`,
`replace_face.rs` and `pcurve_cone_section.rs` changes; the four rows
the PR filed or closed; the pcert row; the gate list diff.

Not run:

- clippy;
- `scripts/doc-gate.sh`;
- the slow set whole, and the other 21 of the PR's 23 cone slow rows;
- `a_cone_apex_on_an_edge::an_edge_across_the_apex_holds_half_the_cones_top`;
- ε = 1e-6 over the whole suite;
- the demo roots.

Two disk-full events hit builds, not test runs. Every result below
comes from a run that completed.

## Claims

**C1, holds (DEMONSTRATED).** Exact oracle, 1,296 points on minted
sections (phase `(1, 0)`):

- Near the anchor, the worst miss off the exact plane and cone is
  3.3e-16, 1.8e-16, 3.3e-16 and 1.9e-16 m at a = 1e3, 1e4, 1e5, 1e6
  (0.77–1.53 `u·(1+|p|)`). The PR says under 7e-16.
- A quarter turn from the anchor the miss is 1.3–1.8 `u·a`, and a half
  turn 1.0–3.6 `u·a`. That is the centre form's scale, which is
  expected, since those points are `a` from the anchor.
- **Running bound: 0 violations in 5,184 points**, re-seamed (phases
  `(0, ±1)`, `(−1, 0)`) and reversed included, against the exact sum
  `anchor + u·a(cos θ − c₀) + (n×u)·b(sin θ − s₀)`. I found no
  uncharged term.
- Two weaker probes also show the bound is not slack in the wrong
  direction. Halving the sines' charge goes red. Making the evaluator
  read the derived centre goes red in the running-bound row.
- `Interval`: near the anchor the enclosures are 0.7–7e-15 wide. None
  excludes the exact cone. 32 of 5,184 exclude the exact plane, by at
  most 1.3e-16 m, which is the stored anchor's own `f64` rounding off
  the plane: the enclosure is of the stored curve, as intended. See N4
  for the enclosure-versus-stated-vectors note.

**C2, holds; premise wrong (NOTE, sure).** The near-gap row's floors are
7/7/7/7/1 centred and 40/40 anchored, and it passes at head.
`RESOLUTION_ROUNDINGS` is **32 at both base and head**
(`crates/geom-brep/src/carrier_resolution.rs:40`). It went 32 → 40 in
`801623f7` and back to 32 seven minutes later in `4545b953`, both inside
the branch, so the net diff does not change it. The constant is fitted
("not a derived count", `:35-39`). It is guarded by
`the_resolution_bounds_each_section_tables_carrier` at a tenfold margin,
and that row went red under two of my mutants (`quarter-wrong-way`,
`half-keeps-pair`), so the guard is live. Neither anchored floor of 40
is fitted: it is the family's size. The anchored circle family is
test-only (S4).

**C3, holds in substance; the base column is wrong (MINOR,
DEMONSTRATED, sure).**

- Anchor bits are kept through re-seams at θ = 0 … 5.5 and their
  reversals (probe, every case `assert_eq` on bits). The pair is the
  exact permutation and the reversal flips the sine.
- After an `f64` quarter turn the **derived centre moves** by 1.3e-10 m
  at a = 1e6 (about 1.2 `u·a`), because the turned frame is re-squared
  (`turned_to(normalize(v))`, `conic.rs:968`). After a half turn it is
  bit-exact. See S9.
- Widths, m³. Head reproduces the PR to every digit. Base `ec8749552`
  does not:

| row, op | PR "centre-stored base" | base `ec8749552`, measured | head, measured | narrowing actually |
|---|---|---|---|---|
| certified, cone ∪ box | 4.86e-11 | 3.79e-11 | 3.22e-11 | −15% (PR −34%) |
| certified, box ∪ cone | 7.78e-11 | 5.82e-11 | 5.39e-11 | −7% (PR −31%) |
| certified, cone ∖ box | 4.83e-12 | 3.01e-12 | 2.68e-12 | −11% |
| point maps, cone ∪ box | 2.02e-11 | 1.33e-11 | 1.15e-11 | −14% |
| point maps, box ∪ cone | 3.00e-11 | 1.73e-11 | 1.64e-11 | −5% |
| point maps, cone ∖ box | 2.09e-12 | 1.03e-12 | 9.31e-13 | −10% |

  The base column was probably taken on an earlier base. Head is still
  narrower in every row, so the claim's direction stands. The same
  column is in the closed row `an-anchored-cone-section-encloses-wider-at-interval`
  and in the PR body's "What the phase bought" table. Both rows pass at
  head at ε = 1e-9 and 1e-6.
- Paths that move the anchor or drop the pair: none in production by
  inspection. There are three mutant-proven test gaps:
  `map_scalar` dropping the pair, the azimuth ignoring the pair, and
  `zero_point` ignoring it (C9).

**C4, holds vacuously; premise needs correcting (NOTE, sure).** Each of
the three doors rebuilds **circles only** (`replace_face.rs:1277`, `:1320`, `:1378`, `:1389`,
`:1471`; `offset_axial.rs:2218`, `:2251`). Their ellipse
arms return `None`. No production path mints an anchored circle: the
only `Circle::anchored` calls are in tests
(`in_band_sweep.rs:754`, `memo.rs:1788`, `implicit.rs:3383`, `:3469`, `:3471`).
So no door turns an anchored conic into a centre-stored one today.

My grep of every production constructor within 20 lines of a conic read
found one more rebuild: `geom/src/pole_form.rs:58-63`
(`half_turn_curve`, `Curve3::circle(place(c.frame().origin()), …)`). It
is also circle-only and so vacuous today. It is a fourth door of the
same shape that the PR does not list. It also re-squares the frame.

That grep is blind to a rebuild whose constructor sits more than 20
lines from its read, and to rebuilds through a helper taking the parts.

**C5, partly false (MINOR, by inspection, sure).**

- `crates/step-import/src/build.rs:1753`, production, asks
  `carrier.conic().is_some_and(|k| k.anchor().is_some())`. That is a
  reader outside `conic.rs` asking which form a conic has, contrary to
  "no reader outside that file matches on it". `Conic::anchor()` is a
  public trait method returning `Option` (`conic.rs:263`), so the form is public API, not
  private to `conic.rs`.
- `mesh/src/memo.rs:1098` reads the datum name, which is the honest
  route.
- `same_content`, `data()` and the two keys do part the forms. However,
  `same_content`'s form check and `data()`'s phase datum are each
  unpinned (C9).

**C6, holds for pass/fail (DEMONSTRATED); the ceiling is scheduled.**

- Base at ε 1e-9: 17,459 of 17,460, red `pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`.
- Head at ε 1e-9: 17,466 of 17,467, the same red.
- Base at ε 1e-12: 17,458 of 17,460, red villarceau and `mesh reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound`.
- Head at ε 1e-12: 17,465 of 17,467, the same two.

So no row changed verdict. I did not diff bits or digests beyond
pass/fail.

The 48 → 104 `u` ceiling (`pcurve_cone_section.rs:887`) is fitted
to 94.6, and the row that owns it is open
(`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`,
P3). Raising it rather than refusing is consistent with that schedule.
See S6 for how weak the ceiling now is as a guard.

**C7, the cover misses on the base too: main's defect, not this unit's
(NOTE, DEMONSTRATED, sure).** At `f64`, `cover_over` of a short arc
next to the anchor does **not** contain the evaluator's own points. The
worst excess past `range_along` over 5 directions:

| a | anchored | centre-born at the derived centre (the base's code path) |
|---|---|---|
| 1e2 | 7.1e-15 | 5.3e-15 |
| 1e3 | 1.7e-13 | 2.2e-13 |
| 1e4 | 9.4e-13 | 1.8e-12 |
| 1e5 | 1.9e-11 | 1.2e-11 |
| 1e6 | 7.2e-11 | 1.2e-10 |
| 1e7 | 1.75e-9 | 1.58e-9 |

At `Interval`, every point enclosure meets the cover's range at every
`a`. That check is weak: it tests that the two meet, not that the exact
point is inside the cover.

Since the centre form misses by the same amount, this is
`work/oneround/the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it.md`.
The unit neither makes it nor widens it. It does expose it: an anchored
section's points are now near-exact while its cover is still read from
the derived centre, so the cover misses the true arc where before cover
and evaluator missed it together.

**C8, holds (by inspection, sure).** The production callers of
`seamed_near`, `seamed_toward` and `quarter_turned_where` are
`topo/src/chord_join.rs:1458` and `:2067`, both `SectionConic::seamed_near`.
Everything else is tests, or `section_frame`/`major_frame`, which build
a turned value only to read sizes and a direction. The row pins both
sides: my probe measured re-seamed near-anchor misses of 1.05–1.8
`u·A`, consistent with it.

**C9, the author's mutants were not re-planted (I planted my own).**
Results, each against the anchored rows of `geom`, `geom-brep` and
`mesh`:

| mutant | red in |
|---|---|
| anchored evaluator reads the derived centre | geom `a_re_seamed_ellipse_resolves…`; geom-brep `a_far_centred_section_stands…`, `an_anchored_points_running_bound…`, `the_running_residuals_are_the_plain_chain` |
| quarter turn rotates the pair the wrong way | geom `a_re_seam_moves…`, `a_re_seamed_ellipse_resolves…`; geom-brep `the_resolution_bounds_each_section_tables_carrier` |
| half turn keeps the pair | geom `a_re_seam_moves…`; geom-brep `the_resolution_bounds…` |
| derived centre's sine term sign flipped | geom `a_re_seam_moves…` |
| sines charged half | geom `the_anchored_offsets_bound_charges_the_squares_rounding` |
| anchored circle's azimuth reads the anchor as centre | geom `an_anchored_conic_reads_from_its_anchor` |
| **`same_content` ignores the form** | **survives** |
| **`map_scalar` drops the phase (resets to `(1, 0)`)** | **survives** |
| **`data()` omits the `Phase` datum** | **survives** |
| **anchored azimuth ignores the phase's sine** | **survives** |
| **anchored azimuth ignores the phase's cosine** | **survives** |
| **`zero_point` of an anchored ellipse is the anchor** | **survives** |

I then ran the survivors against the whole `ci` suite at ε 1e-9. The
base red, villarceau, is excluded from every count below:

- **All six together: 13 new reds.** They are all in pre-existing
  `mesh` and `sweep` cone rows: `reach_cone_split` ×4,
  `cone_join_lane` ×2, `a_plane_moved_to_a_cone_apex`,
  `a_plane_near_a_needle_cones_apex`,
  `a_ring_on_a_cone_face::a_ringed_cone_faces_sense_flip_fails_tier_3`,
  mesh `reach_probes` ×2, `split_rulings` and `trimmed`.
- **Without the azimuth pair** (`zero_point`, `map_scalar`, `data()`,
  `same_content`): 10 new reds, the same rows less `cone_join_lane` ×2
  and the needle-cone row.
- **`zero_point` + `data()` phase + `same_content` form: 0 new reds**,
  17,466 of 17,467.

So the whole suite catches `map_scalar` dropping the phase (by
subtraction, the 10). It catches the azimuth ignoring the phase (the
3 more, and possibly some of the 10). It does not catch the other three
anywhere. `zero_point` has no production reader at all: its only call
outside its own impls is the `&C` forward at `conic.rs:385`.

What this means for C9: the unit's own rows do not pin the re-seamed
readers, but pre-existing integration rows two crates up pin two of
them. Three properties the PR names as the contract (C5: two conics
differing only in form are never taken for one; the key reads the
phase) are pinned by nothing in the `ci` suite.

**C10, no wrong binding found in production (by inspection plus a
mechanical sweep, likely).**

- `shadow_audit.py` lists every added `let frame = …` in a production
  file of the diff whose function holds another `frame` binding. There
  are 13 sites: `on_surface.rs:786`, `pcurve_cache.rs:11463`,
  `props/curved.rs:845`, `:3236`, `:3250`, `step-import/normalize.rs:635`,
  `join.rs:1587`, `:4118`, `:4155`, `readback.rs:599`, `:612`,
  `replace_face.rs:1315` and `ring_path.rs:3033`.
- I read each. In every one the new binding is consumed in its own arm
  or block before any outer `frame` is read again. For example, in
  `normalize.rs:619/635` the surface's `frame.axis()` is read at `:622`,
  before the shadowing.
- Blind to: a rebinding under a name other than `frame`; a use that
  binds to a frame of the right name from an enclosing closure; and
  test files, which I did not sweep.

**C11, reproduced (DEMONSTRATED), except the gate count (MINOR, sure).**

- The run counts and reds are above, identical red for red to the base.
- Gates: **three** are red at head, not two: `cover-kind-match`
  (`RAISED ball crates/geom-brep/src/curve_face/line_quadric.rs: 0 -> 2`),
  `engine-leaves` (`census.rs: 52 -> 53`) and `pin-cites-an-open-row`.
  All three are red on the base `ec8749552` with the same payload, and
  `line_quadric.rs` is not in this unit's diff. So it is not the unit's
  red, but the PR's account omits one.
- Not run: clippy, the 21 other cone slow rows, and ε 1e-6 over the
  suite.

**C12, mostly holds; one closed row carries the wrong base numbers
(MINOR, sure).**

- `bare-rounding-sites.tsv` is what the tree holds, since
  `bare-rounding-allowlist OK` at head with 618 sites (234 in
  production) against base's 607 / 231. Its `conic.rs` (prod 14, test 6)
  and `implicit.rs` (prod 64, test 5) entries match the PR.
- `cover-kind-match.counts` changes `join.rs` 18 → 17.
- `an-anchored-cone-section-encloses-wider-at-interval` is closed on the
  width table whose base column C3 does not reproduce.
- The other three rows (`one-stored-form-…`, `a-section-conic-is-anchored-at-a-general-point-…`,
  `a-far-centred-conic-born-with-a-centre-…`) describe the code as I
  read it.

## Findings

### MINOR

**M1. The re-seamed anchored ellipse's readers are unpinned (DEMONSTRATED, sure).**

- The surviving mutants: `crates/geom/src/curves/conic.rs:1111-1112`
  (azimuth, both phase terms), `:1119-1124` (`zero_point`), `:832`
  (`map_scalar`'s phase), `crates/geom/src/curves.rs:1168-1174` (the
  `Phase` datum) and `conic.rs:594`, `:867` (`same_placement`'s form
  check).
- The rows exercise those readers only at phase `(1, 0)`, where each
  mutant is a no-op. `an_anchored_conic_reads_from_its_anchor` reads
  `param_in` back on un-re-seamed conics only (`:1347`).
  `a_re_seam_moves_parameter_zero_and_never_the_anchor` checks
  `point_at(0)` against the vertex but never `zero_point()` (`:1442`);
  it checks the `Interval` centre's width, never its value, and it
  re-seams after lifting, never lifts after re-seaming (`:1458`).
- `an_anchored_conic_reads_from_its_anchor`'s `!carrier.same_content(&centred)`
  (`:1320`) compares two conics with different stored origins, so it
  cannot see the form (Q3).
- Production reaches the re-seamed path through
  `SectionConic::seamed_near` on every minted cone section that the
  chord join seams.
- Whole suite: the azimuth and `map_scalar` mutants are caught only by
  pre-existing `mesh`/`sweep` cone rows, nothing in the unit's own rows.
  **`same_content` ignoring the form, `data()` omitting the phase and
  `zero_point` returning the anchor survive all 17,467 `ci` rows.** So
  C5's "two conics that differ only in form are never taken for one" is
  enforced by code that no row checks. The PR's
  `the_curve_key_parts_a_centre_from_an_anchor` pins the key's anchor
  mark. Neither the phase's place in the key nor `same_content`'s form
  check is pinned by any row.

**M2. The PR's and the closed row's base widths are not the base's
(DEMONSTRATED, sure).** See the C3 table. Every narrowing is real and
smaller than claimed: 5–15%, not 31–55%. The body's "What the phase
bought" table and `work/oneround/an-anchored-cone-section-encloses-wider-at-interval.md`
both carry the stale column.

**M3. A reader outside `conic.rs` asks the form (by inspection, sure).**
`crates/step-import/src/build.rs:1753` and the public
`Conic::anchor()` (`conic.rs:263`) contradict "no reader outside
that file matches on it". `data()` already names the datum `Anchor`
(`curves.rs:1149`), which is what `mesh` reads. See S5 for the
duplication.

**M4. The verification account omits a red gate (DEMONSTRATED, sure).**
`cover-kind-match` is red at head and at base (C11). It is not the
unit's, but the account says two.

### NOTE

**N1. C2's premise:** `RESOLUTION_ROUNDINGS` did not change net (sure).

**N2. C4's premise:** the three doors see circles only, and no
production path mints an anchored circle. `pole_form::half_turn_curve`
(`geom/src/pole_form.rs:58`) is a fourth rebuild door of the same shape
(sure).

**N3. C7 is main's defect, which this unit exposes** (sure): see the
table.

**N4. `Interval` enclosures do not contain the stated `f64` vectors'
exact point (unsure).** 48 of 5,184 enclosures exclude the exact stored
formula computed on the stated `f64` axis and seam, by up to 40% of the
box width. These are quarter and half turns, e.g. 1e-12 outside a
2.7e-12 box at a = 1e5. They do not exclude the exact surfaces.

`RigidMotion::map_scalar` to an `Exact` kind encloses a nearby exact
rotation (`enclose_rotation`, `geom-core/src/linalg/rigid_motion.rs:1346-1352`),
not the stated vectors, so this is likely the base's design rather than
this unit's. I could not decide whether "the curve" at `Interval` is
meant to be the stated one.

**N5. A re-seam moves the derived centre (DEMONSTRATED, sure).** It
moves by about 1.2 `u·a` across an `f64` quarter turn. See S9.

**N6. The ceiling `FLAT_IMAGE_ULPS = 104` is a fitted bar** (sure). Its
own mutant, per the pcert row, reads 42 / 160 / 82 across the three
poses, so only one pose would catch it.

## Style

Questions exercised: Q1 (the prose sweep and a constants and
duplication read of `conic.rs`), Q2, Q3, Q4, Q5, Q6, Q7 and Q8
(`conic.rs` read end to end, 1,647 lines). Q2's reconcile grep turned
up nothing in the touched files beyond D9's vocabulary.

- **S1 (Q4, sure).** `crates/geom-brep/src/intersect.rs:2574-2578`. The
  comment "The centre is the apex's foot on the plane moved along the
  major axis … reads a vector that cancels to K along the normal and
  then divides it by K" describes `tilt.centre(apex_gap)`, which this
  diff removed (`-let center = apex - n * apex_gap + u_major * tilt.centre(apex_gap)`).
  The new paragraph was appended beneath it. The doc rotted; the code
  is right.
- **S2 (Q5, sure).** Docs that are false of an anchored conic:
  - `conic.rs:8`: "The placement, [`Frame`]: centred on the conic".
  - `conic.rs:120` and `:147`: `Circle`'s and `Ellipse`'s headline
    formulas `P(θ) = origin + …`.
  - `conic.rs:39-49`: the module's "Born anchored" bullet gives
    `anchor + u_ref·a·(cos θ − 1) + v_ref·b·sin θ` and the centre
    `anchor − u_ref·a`. Both hold only at phase `(1, 0)`; the next
    section generalises without saying the first was a special case.
  - `conic.rs:215`: `jet_at` "from one `sin_cos`". Both kinds' arms
    now call `point_at`, which on the anchored form reads two more
    sines (`:663`, `:1082`).
  - `conic.rs:959`: `quarter_turned_where` "Built through
    [`Self::new`]". It is a struct literal now, and its panic text names
    `Ellipse::new` for an anchored ellipse too (`:990`, `:995`).
- **S3 (Q1, sure).** `Circle` and `Ellipse` carry near-verbatim
  copies:
  - `centred_at` (`conic.rs:601` and `:878`, the second's doc with "a
    ellipse");
  - `stored` (`:579` and `:841`);
  - `same_placement` (`:594` and `:867`);
  - `anchor()` (`:697` and `:1126`);
  - the `Born` match in `frame`, `point_at`, `azimuth`, `zero_point`
    and `running_point` of each.

  Across the two impls, 19 sites match on `born` or `anchor()`. A class: the
  next form will touch every one.
- **S4 (Q7, sure).** The anchored circle is production code with no
  production caller. `Circle::anchored` and the anchored arms of
  `Circle::frame`, `point_at`, `azimuth` and `running_point`
  (`conic.rs:541`, `:635`, `:648`, `:679`, `:716`)
  are reached only from tests. The near-gap row's 40/40 "circle" result
  is about a form nothing mints.
- **S5 (Q1, likely).** Two hand-written "anchor mark" bits,
  `mesh/src/memo.rs:1098` and `step-import/src/build.rs:1753`
  (`tag | (anchored << 8)`), each restate what `data()` already says by
  the datum's name. They also read the fact two different ways. Any
  other consumer that serialises `data()`'s scalars without names
  conflates a circle centred at P with one anchored at P. Where else to
  look: every `.data()` reader. I checked
  `pcurve_cache.rs:4303`, which keys on the datum names and so parts
  them, and `validate.rs:7558`, which needs no key.
- **S6 (Q6/Q3, likely).** `crates/geom-brep/tests/pcurve_cone_section.rs:884-888`.
  "Fitted to the readings below, not derived": a measured ceiling
  raised 2.2× to sit 10% over its worst reading. Its guard is the row
  itself, and its mutant reaches it at one pose of three. The
  reason-at-site that Q6 asks for is half there (the pcert row is
  cited in the PR, not at the constant).
- **S7 (Q3, sure).** `conic.rs:1321`: see M1, a
  "not the same content" assertion that cannot fail on the property it
  names.
- **S8 (Q2, likely).** The unit argues for itself at length. The module
  docs plus the `quarter_turned_where` and `seamed_toward` comments run
  about 90 lines over about 60 lines of re-seam code. This is mild
  evidence, per the lane, and it is where S2's stale statements live.
- **S9 (Q5, sure).** `conic.rs:72-73` says "the centre … derived once
  from the anchor whatever seam the ellipse is read at". At `f64` a
  quarter turn re-squares the frame (`:968`), and the derived centre
  then moves by about 1.2 `u·a` (C3 probe). "Derived once" reads as
  invariance, and it is not.
- **S10 (Q7, unsure).** The phase is stored as a `(T, T)` pair that only
  ever holds one of four exact values, `(±1, 0)` or `(0, ±1)`, and is
  then mapped through `f` like a size (`:832`, e.g. a widening `f`
  widens it). A four-valued quarter-turn index would make M1's
  `map_scalar` survivor unrepresentable.

- **S11 (Q7, sure).** `Conic::zero_point` (`conic.rs:259`, impls `:690`,
  `:1119`) is a trait method with no production reader. A mutant that
  makes it wrong after every re-seam survives the whole suite. That is
  a reading added beside the readers that matter, defended by its doc
  alone.

### Ratings

- **Idiom and structure: 3.** Clean form split and a single evaluator
  path. However, the form is matched by hand at 19 sites across
  two impls, plus a production-dead anchored circle (S3, S4).
- **Test quality: 3.** The minted path is pinned well: all 3 of my
  minted-path mutants went red, and so did the three re-seam mutants on
  `point_at` and the centre. The re-seamed path's other readers, and
  the form and phase in identity, are not pinned: 6 of 12 survive (M1).
- **Doc and comment honesty: 3.** The PR body's first section is
  admirably frank and matches the code. However, there are stale docs
  in `conic.rs` and a dead comment in `intersect.rs` (S1, S2, S9), and
  the evidence has a stale base column and a miscounted gate (M2, M4).
