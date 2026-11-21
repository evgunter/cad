# Review r2 of PR 7203 (`oneround/anchored-conic`, frozen head `8d59783007a93f62c21f3581dc651c8d70c81caf`)

Unit diff reviewed: `git diff ec8749552 8d5978300` (193 files). Base worktree at
`ec8749552`. Everything below is local, debug build, 4 cores.

I did not fetch, read or glimpse the other reviewer's branch or any summary of it.

## Verdict: APPROVE-WITH-FIXES

Nothing I could run shows the unit computing a wrong value on a production path.
The anchored evaluator, its running bound and its `Interval` enclosure hold
against a 60-digit oracle; the suite and the gates read as the author says; the
stale-binding class is absent from production code. What is owed is on the test
and the account side: several rows do not pin what they say they pin (eight
mutants survive everything I ran and four more are seen only by `sweep`'s
integration rows), one table in the PR body is taken on an older base
than the one the unit is stacked on, one claim of the author's (three rebuilding
doors) is short by about ten sites, and the module's own docs say two different
things about what the anchor is.

No MAJOR. 8 MINOR, 11 NOTE.

## What I executed, and what I only read

EXECUTED
- `ci` nextest profile, whole workspace, on the head, at ε 1e-9, 1e-6 and
  1e-12.
- A probe target of my own, one file compiled on BOTH trees
  (`crates/geom-brep/tests/review_r2_common.rs`): 107 far-centred cone sections
  (three cones, planes 1e-2 … 1e-8 short of the parabola, semi-axes 1 … 5e7),
  their points, running bounds, `Interval` enclosures, covers and chart images.
- An oracle of my own (`analysis/oneround/anchored-conic/r2/oracle.py`,
  `mpmath`, 60 digits): the distance of each `f64` point to the plane and the
  cone it was cut from; the stored form's exact value; enclosure membership.
- 27 mutants (`analysis/oneround/anchored-conic/r2/mutants.py`), the author's
  eight and nineteen of mine, in a separate worktree and target directory.
- The cone ring's two slow rows with the widths printed, head and base, at
  three ε. The near-gap row with its counts printed.
- `scripts/run-gates.sh` on a clean head worktree and a clean base worktree.
- A head-only probe (`crates/geom/tests/review_r2_head.rs`).

ONLY READ
- `conic.rs` whole (Q8), the README section, the rows, the PR body, the diffs
  of `curves.rs`, `implicit.rs`, `carrier_resolution.rs`, `intersect.rs`'s
  mint, `memo.rs`, `build.rs`, `signed_zero.rs`, `scalar_lift.rs`,
  `in_band_sweep.rs`, `pcurve_cone_section.rs`, `conic_oracle/exact.rs`.
- C4 and C10 were read-only sweeps (two delegated readers, whose findings I
  spot-checked at the sites cited here, not all of them).
- C6: I compared the centre arms of `conic.rs` with the base's by reading. I
  did NOT run the base's whole suite, so my differential is "the head's reds
  are the base's" plus the probe, not a row-by-row one.
- NOT RUN: clippy; the slow set beyond the two ring rows; the 23 slow rows as
  a set; `a_cone_apex_on_an_edge::…`; the analysis branch
  `analysis/oneround/conic-vertex-probes` (its numbers are taken as stated).

My probes add two test targets, so `every_suite_file_is_aggregated` is red in
`geom` and `geom-brep` on my branch. That red is mine, not the unit's.

## The claims

### C1 — holds (sure near the anchor; the bound holds on every sample)

Head, 5,076 samples, worst over each semi-axis decade, points within 500 m of
the anchor (parameters `d/a` and `d/b`, `d` up to 400 m):

| semi-axis | n | off the plane [m] | off the cone [m] | miss / u·(1+\|p\|) | error / running bound | base, same generator: off the cone [m] |
|---|---|---|---|---|---|---|
| 1e2 | 936 | 4.1e-14 | 4.7e-14 | 2.2 | 0.957 | 4.7e-13 |
| 1e3 | 554 | 2.8e-14 | 4.2e-14 | 3.1 | 0.928 | 8.0e-12 |
| 1e4 | 693 | 1.9e-14 | 2.5e-14 | 2.3 | 0.945 | 2.7e-11 |
| 1e5 | 582 | 1.4e-14 | 2.1e-14 | 1.9 | 0.953 | 2.2e-10 |
| 1e6 | 835 | 3.1e-14 | 3.9e-14 | 2.0 | 0.946 | 3.6e-9 |
| 1e7 | 185 | 1.3e-14 | 2.0e-14 | 2.8 | 0.798 | 1.3e-8 |

- The author's "under 7e-16 m" is true of points at the origin's scale; my
  anchors sit up to 150 m out, so my worst is `3 u·|p|`, which is the same
  statement. Not falsified.
- A quarter and a half turn from the anchor the miss is 1.1 … 2.6 `u·a`
  (2.4e-9 m at 1e6), as the base's is everywhere. Expected and stated.
- The running bound: the point's miss off the stored form's exact value
  (anchor, `u_ref`, `axis × u_ref` exactly, sizes, the phase `(1, 0)`) is at
  most 0.957 of the bound on all 15,228 coordinates. No term is uncharged in
  a way that makes it false. The margin is thin (4%), and one charge is
  doing another's work: see m4.
- `Interval`, the `f64` curve lifted: the enclosure holds the exact value on
  15,227 of 15,228 coordinates. The one miss is NOTE n4 (the base's, 33 there).
- `Interval`, the section MINTED at `Interval` (5,076 boxes): every box
  straddles both true surfaces. Near the anchor it is 7e-6 m wide at 1e6 where
  the base's is 1.2 m.

### C2 — holds; the brief's premise about the constant is wrong (n1)

Counts printed on the head: centred 7, 7, 7, 7, 1; anchored 40 in each family.
`RESOLUTION_ROUNDINGS` is `32.0` at the frozen head, as on the base.

### C3 — the invariants hold; the width table's base column is stale (m1)

- No path I found moves the anchor or turns one of `u_ref` / phase without the
  other on a production route. Mutants A1–A4 go red. `map_carrier`'s image
  under a reflection keeps the parameter, so keeping the phase is right there
  (BY-INSPECTION, likely).
- One door does turn the frame without the phase: `Ellipse::placed` handed a
  reversal. No production caller does it. m5.
- Widths reproduced on the head to the digit. Not on the base. m1.

### C4 — the author's "three doors" is short (m7)

### C5 — the four readers part the forms; three of the four are not pinned (m2); two sites outside `conic.rs` do ask (n2)

### C6 — holds as far as I took it

The centre arms of `point_at`, `azimuth`, `running_point`, `frame` read as the
base's (BY-INSPECTION). The head's reds at ε 1e-9 and 1e-6 are the base's one.
Raising the image ceiling is disclosed and scheduled (`work/pcert/…`, P3,
open). I measured the image where the row does not look: n8. The row itself
got weaker: m6.

### C7 — the `f64` cover does not hold the arc's points, on the base as on the head; at `Interval` the head holds and the base does not

`cover_over` of a short arc near the near vertex (windows 0.01, 1 and 50 m
long, three placements, 99 to 504 windows a decade), read through
`range_along` in 20 directions against 65 evaluator points, the dot product
taken as an enclosure. Worst miss, metres; negative is contained:

| semi-axis | head `f64` | head / u·a | head `Interval` | base `f64` | base `Interval` |
|---|---|---|---|---|---|
| 1e2 | 2.4e-13 | 2.8 | −8.9e-16 | 2.6e-13 | +1.1e-15 |
| 1e3 | 2.4e-12 | 2.6 | −4.3e-14 | 2.3e-12 | +1.4e-14 |
| 1e4 | 2.8e-11 | 2.9 | −9.9e-14 | 3.2e-11 | +4.3e-14 |
| 1e5 | 2.7e-10 | 4.1 | −9.0e-13 | 2.5e-10 | −6.1e-13 |
| 1e6 | 2.8e-9 | 2.7 | −5.5e-12 | 2.3e-9 | +1.5e-11 |
| 1e7 | 1.0e-8 | 1.5 | −4.1e-11 | 1.5e-8 | +3.6e-11 |

So: the `f64` miss is main's defect (the row
`the-arc-cover-misses-a-far-centred-arc-…` owns it), the same size on both
trees; the unit neither makes nor worsens it. What changes is its meaning: on
the base the evaluator and the cover were each `u·a` off the true arc, on the
head the evaluator is right and the cover alone is `u·a` off. At `Interval`
the unit repairs a containment the base lacks. The parameter reads back near
the anchor at the arc's scale (`b·Δθ` under 1e-12 m to 1e7). The chart image:
n8. I did not probe `extent.rs`.

### C8 — true as written

`SectionConic::seamed_near` (`topo/src/chord_join.rs:1457`, from `:2067`) is
the one production caller of a re-seam (grep over `seamed_near`,
`seamed_toward`, `quarter_turned_where`; `section_frame` and `major_frame`
turn a copy they only read sizes and a direction from). I did not re-measure
the 1.45 … 3.6 `u·a`; my quarter- and half-turn rows (1.1 … 2.6 `u·a` on an
un-re-seamed section) are the same order. The module docs do not say what the
PR body says: Style S1.

### C9 — the author's eight: seven red, one survives as I planted it; of mine, seven survive and four are red only in `sweep`

| mutant | result |
|---|---|
| A1 anchor moved by a turned seam | RED (3 rows) |
| A2 quarter-turn phase dropped | RED (2) |
| A3 half-turn phase dropped | RED (1) |
| A4 reversal keeps the sine | RED (1) |
| A5 half-angle square's own rounding uncharged, the sine's share kept | **SURVIVES** (m4) |
| A6 key's anchor mark removed | RED (`mesh`) |
| A7 every sine uncharged | RED (2) |
| A8 vertex read as centre less semi-axis | RED |
| R1 anchored evaluator reads the derived centre | RED (4) |
| R2 the pair swapped in the evaluator | RED (3) |
| R3 / R4 `same_content` ignores the form (ellipse / circle) | **SURVIVE** (R3 also the `topo` and `sweep` rows below) |
| R5 `same_content` ignores the phase | **SURVIVES** (also the `topo` and `sweep` rows below) |
| R6 `Ellipse::map_scalar` drops the phase | survives `geom`, `geom-brep` and 528 `topo` rows; RED in 5 `sweep` rows |
| R7 `Ellipse::placed` drops the phase | survives `geom`, `geom-brep` and 528 `topo` rows; RED in 5 `sweep` rows |
| R8 `Curve3::data()` omits the phase | **SURVIVES** (`geom`, `mesh`, `step-import` whole; the `topo` and `sweep` rows below) |
| R9 / R10 anchored `azimuth` ignores the phase's sine / cosine | survive `geom`, `geom-brep` and 528 `topo` rows; RED in 2 / 7 `sweep` rows |
| R11 derived centre ignores the phase's sine | RED |
| R12 `centred_at` sets an anchored origin | RED (7, `geom-brep`) |
| R13 anchored circle's azimuth read from the origin | RED |
| R14 `carrier_sig`'s anchor mark removed | **SURVIVES** (`step-import` whole) |
| R15 first offset ignores the phase | RED |
| R16 `sin θ`'s charge dropped in `running_offsets` | **SURVIVES** |
| R17 `Circle::map_scalar` drops the form | **SURVIVES** |
| R18 `zero_point` returns the anchor | red only in `r2_turn_probes::turn_defect_speeds_…`, which builds no anchored conic: I read that as that row's own flake, so SURVIVES (unsure) |
| R19 the mint handed back centre-stored | RED |

Scope of a "survives": `geom` whole and `geom-brep`'s rows matching
`anchored | running | far_centred | resolution | conic | cone_section`, unless
a wider scope is named. The wider scope, run for R3, R5, R6, R7, R8, R9, R10:
`topo` rows matching `cone | seam | chord | section | ellipse` (528), and
`sweep`'s `a_ring_on_a_cone_face` and `cone_join_lane` (10 rows, slow ones
included). I did not run a mutant against the whole workspace.

### C10 — no wrong binding in production (likely)

88 changed files under `src/`, every hunk, plus a search for a dropped pattern
binding whose bare name survives further down and now resolves to an outer
one. None. The multi-frame sites checked by name: `offset_axial.rs:2165`,
`:2521`; `junction.rs:609`; `arms.rs:1590`; `injective.rs:3839`, `:3872`;
`validate.rs:9834`; `ring_path.rs:1226`, `:2079`, `:2136`;
`pcurve_cache.rs:12736`; `join.rs:1546`. Not examined: the 96 changed files
under `tests/` and `examples/`.

### C11 — reproduced where run

| run | mine | the author's |
|---|---|---|
| `ci`, ε 1e-9 | 17,465 of 17,467; less my probe's own red, 17,466 | 17,466 |
| `ci`, ε 1e-6 | 17,464; less my two, 17,466 | 17,466 |
| `ci`, ε 1e-12 | 17,463; less my two, 17,465 | 17,465 |
| gates | two red, `engine-leaves` and `pin-cites-an-open-row`, the same two lines on the base | the same |

- `pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`: red on the
  base, `MapResidual … margin 1.25e-9`.
- The certified-scalar ring row at 1e-12: base `VolumeUncomputable`
  (1.094e-12), head `SurfaceResidual`, sample 7, ±1.058e-12, in the join. The
  row and the PR body say so. It is not "the same payload", which is the
  brief's wording and not the author's: n6.
- `mesh reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound` at
  1e-12: red on the base with the head's message ("builds at ε 1e-12, where it
  refused CurvedPierceUnsupported").

### C12 — the rows say what the code does, with one stale column (m1)

`bare-rounding-allowlist` passes on the head (`conic.rs` prod 14, test 6;
`implicit.rs` prod 64, test 5).

## Findings

### MINOR

**m1. The width table's "centre-stored base" is not the base the unit is
stacked on.** DEMONSTRATED, sure. PR body "What the phase bought";
`work/oneround/an-anchored-cone-section-encloses-wider-at-interval.md`
("Closed by the phase"). The row's first table names its commit, `3df3b1d41`;
the PR body and the closing table do not. At `ec8749552`, ε 1e-9 and 1e-6:

| row, op | base, as stated | base `ec8749552`, measured | head, measured |
|---|---|---|---|
| certified scalar, cone ∪ box | 4.86e-11 | 3.79e-11 | 3.22e-11 |
| certified scalar, box ∪ cone | 7.78e-11 | 5.82e-11 | 5.39e-11 |
| certified scalar, cone ∖ box | 4.83e-12 | 3.01e-12 | 2.68e-12 |
| point maps, cone ∪ box | 2.02e-11 | 1.33e-11 | 1.15e-11 |
| point maps, box ∪ cone | 3.00e-11 | 1.73e-11 | 1.64e-11 |
| point maps, cone ∖ box | 2.09e-12 | 1.03e-12 | 9.31e-13 |

The head is narrower than its base on all six, by 5 to 15%, not by 31 to 55%.
The claim "narrower" stands; the size of it is the base's own change.

**m2. `same_content`, the import's signature and `data()`'s phase are not
pinned.** DEMONSTRATED (R3, R4, R5, R8, R14 survive), sure for the scopes
named. `crates/geom/src/curves/conic.rs:589`, `:861`, `:852`;
`crates/step-import/src/build.rs:1753`. The one `same_content` assertion on
two forms (`conic.rs:1320`) compares an anchored conic with a centred one at
the DERIVED centre, so the two placements already differ and the `born` arm is
never the deciding one. A circle centred at P against one anchored at P is
asserted for the mesh key only. `step-import` has no row at all for the mark,
though the brief and the PR body each cite "a row" for both keys.

**m3. Nothing in `geom` or `geom-brep` reads a parameter back, lifts a
scalar or maps a placement on a RE-SEAMED anchored ellipse.** DEMONSTRATED
(R6, R7, R9, R10 survive there), sure for that scope. `conic.rs:824`, `:800`,
`:1101`. These are the arms `chord_join`'s re-seam reaches in production: the
re-seamed section's `azimuth`, its lift to `Interval`, its image under a
motion. They do go red further out, in `sweep`'s cone rows
(`cone_join_lane::each_poses_body_is_its_closed_form_in_every_op`,
`::the_tilted_planes_chords_are_its_ellipse_in_both_member_orders`, and the
ring's rows), as a body that fails to build, three crates from the arm. 528
`topo` rows on cones, seams and chords do not see them.

**m4. `the_anchored_offsets_bound_charges_the_squares_rounding` does not go
red when the square's rounding is uncharged.** DEMONSTRATED (A5), sure.
`conic.rs:1264`, `:499`–`:507`. With the square's error set to the sine's
share alone, the row passes: `doubled − (1 − c0)` is charged `u·2sh²` by
`Rounded::sub` though it is exact at `c0 = 1`, and that covers `2·rounding`.
The bound is not false. The row's sentence "Red where the square is not
charged" is, and R16 (the plain sine's charge) survives the same way. The
author's mutant must have dropped more than the square's own rounding.

**m5. `Ellipse::placed` names a reversal among its maps, and a reversal
through it draws another ellipse.** DEMONSTRATED, sure; latent.
`conic.rs:795`–`:808`. On an anchored ellipse seamed a quarter turn on,
`placed(Frame::reversed)` at `−t` is `2a` from the ellipse at `t` (60 m at
`a = 30`, 3,000 m at 1,500); `Ellipse::reversed` reads zero. No production
caller passes one (the four callers pass a translation or `map_frame`).
`Circle::placed(Frame::reversed)` is right, which is how `Curve3::reversed`
uses it one line above the ellipse's own door (`curves.rs:1537`).

**m6. The flat cone's image row is weaker than the row it replaced.**
BY-INSPECTION from the row's own numbers, likely.
`crates/geom-brep/tests/pcurve_cone_section.rs:887`–`:907`. The ceiling went
48 → 104 `u` on readings of 50, 81, 95. Its stated mutant reads 42, 160, 82:
BELOW the unmutated reading at two poses of three, and red at one. On the base
its counterpart (the centre moved the same 30 `u`) read 54, 63, 70 against
48, red at all three. The comment
dropped the two other things that used to red it.

**m7. More than three production doors rebuild a conic through a centre
constructor.** BY-INSPECTION, sure that the sites exist; likely that none
meets an anchored conic today. The three named doors are as described. Also:
- same locus, form lost: `ring_path.rs:1286` (`torus_circle_arc`);
  the `Rim` round trip `rim_wedge.rs:1038` → `rim_wedge.rs:1127`,
  `boolean/mod.rs:8268`, `:8388`; `MeetSegment::Arc`
  `validate.rs:9979` → `:10317`; `on_surface.rs:442`; `swept.rs:798`
  (`circle_at`, to evaluate);
- moved or another curve: `pole_form.rs:58` (`half_turn_curve`),
  `junction.rs:685`, `end_face.rs:898`;
- from an ELLIPSE's derived centre frame, a different conic:
  `ring_path.rs:1896` (`ConicArc::distance_floor`),
  `sweep/src/blend/surgery.rs:2599` (`piece_along`).
Every same-locus site takes a circle, and no production door mints an anchored
circle, so nothing is lost today. They are the sites that lose the form the
day one does. `step-export/src/writer.rs:416` writes the derived centre, which
is Ev's decision on PR 4812.

**m8. The module says two things about what an anchor is.** BY-INSPECTION,
sure. Style S1; listed here because C8 turns on it.

### NOTE

- **n1.** The brief's C2 premise is wrong: `RESOLUTION_ROUNDINGS` is 32 on the
  base and at the frozen head (`carrier_resolution.rs:40`). It went to 40 at
  `801623f7` and back at `4545b953`. Sure.
- **n2.** C5's "nothing outside `conic.rs` asks which form" is the brief's
  wording. `step-import/src/build.rs:1753` asks `anchor().is_some()` and
  `mesh/src/memo.rs:1098` matches `CurveDatum::Anchor`. The PR body says both.
  Sure.
- **n3.** The near-gap row's centred counts are floors (`>=`), so "unchanged"
  is not what it pins; printed, they are 7, 7, 7, 7, 1. Sure.
- **n4.** One lifted enclosure misses the stored form's exact value: semi-axis
  3,980.85, `t = π`, coordinate y, by 8.9e-14 m. The same section misses on 33
  samples on the base. I take it as the base's frame re-minted at `Interval`
  (`Frame::map_scalar`), ONEHOME's open question, not the unit's. Likely.
- **n5.** `point_at(0)` is not the anchor "bit for bit" where an anchor
  coordinate is `−0.0`: it reads `+0.0` (`conic.rs:1072`). Sure, trivial.
- **n6.** At ε 1e-12 the head refuses the ring's join where the base built it
  and refused in the volume. So one residual on that edge is wider at
  `Interval` on the head than on the base (±1.058e-12 against a pass at
  1e-12). Disclosed in the row. "Narrower" is true of the six volumes, not of
  every reading. Sure.
- **n7.** `carrier_resolution::resolution` charges an anchored conic `u·a` as
  it does a centred one, and its module doc still says an ellipse resolves
  its arc "only to about `u·A`". Conservative. Unsure whether any reader
  should now read less.
- **n8.** The chart image of a far-centred anchored section still stands
  `u·a` off its carrier near the anchor. Certificate envelope, worst per
  decade, the window `−20/a … 30/a` at the near vertex, head / base: 1e4 1.0e-10 / 8.5e-11;
  1e5 6.2e-10 / 7.3e-10; 1e6 8.1e-9 / 9.8e-9; 1e7 5.2e-8 / 7.8e-8. Certified
  at ε 1e-9: the same counts within noise (1e6: 25 of 42 against 17). So no
  regression at far sizes, and none of the anchor's precision reaches the
  image. Scheduled on the pcert row. Sure.
- **n9.** `signed_zero::plus_zero_curve` restates the placement and leaves the
  phase as it is; a half turn stores `(−1.0, −0.0)`, which `data()` walks into
  both keys. Unsure it matters: every route I tried to one seam gives the same
  signs.
- **n10.** A seam turn decided on a straddling `Interval` hulls the phase to
  `([0, 1], [−1, 0])`; the derived centre is then about a semi-axis wide where
  a centred ellipse kept its centre exact. Not probed. Unsure.
- **n11.** The bare-rounding list's seven new `conic.rs` entries carry the
  reason `far-centre`. They are the anchored chain's own running bound, which
  is the cure for a far centre, not a form owed a re-posing. Unsure.

## Style

Questions exercised: Q1 (prose sweep and a constants read over the diff), Q2,
Q3 (by mutation), Q4, Q5, Q6, Q7, Q8 (`conic.rs` end to end). Q2's `git blame`
split of comment against code: not done.

- **S1 (Q5, Q4).** `conic.rs:28`–`:56`, `:137`–`:144`, `:256`–`:263`;
  `curves.rs:610`. "The two stored forms" says an anchored conic's stored
  point is "its point at `θ = 0`" and its centre `anchor − u_ref·a`. Twenty
  lines on, the re-seam paragraph says the anchor stays and parameter zero
  moves. `Born::Anchored`, `Conic::anchor`, `Conic::zero_point` and
  `CurveDatum::Anchor` all repeat the first. The code follows the second. Sure.
- **S2 (Q5).** `conic.rs:104`–`:106`: "A caller seams an ellipse at the
  principal vertex nearest the part of it in use, so the parameter there …
  resolves to the scale of that part." For an anchored ellipse the unit's own
  row pins the opposite (`a_re_seamed_ellipse_resolves_near_its_anchor_at_the_semi_axis`).
  The doc rotted; the code is as intended. Sure.
- **S3 (Q5).** `conic.rs:959`–`:961`: `quarter_turned_where` is "Built through
  `Self::new`". It is a struct literal that passes the string `"Ellipse::new"`
  to `stored_size` so the panic reads as before (`:988`–`:997`). Sure.
- **S4 (Q7).** `conic.rs:158`–`:164`: `born` and `phase` are two fields of
  which the second means something only under one value of the first.
  `same_placement` special-cases it (`:869`), `stored_phase` guards it
  (`:853`), `quarter_turned_where` permutes it on a centred ellipse anyway, and
  `Conic::phase()` hands a centred, re-seamed ellipse's `(0, −1)` to any
  caller. `Anchored { phase }` is how I would have held it. Likely.
- **S5 (Q1).** `Circle::stored` / `Ellipse::stored` (`:579`, `:841`) and the
  two `centred_at` (`:601`, `:878`) are the same body twice; `offsets` and
  `running_offsets` (`:473`, `:496`) are one formula in two arithmetics, kept
  in step by a bit-equality row. The topo row has a second, test-local `Born`
  enum (`in_band_sweep.rs`) beside the private one. Where else: the two
  `same_placement`. Likely.
- **S6 (Q3).** m2, m3, m4, m6 and n3 are one class: a row whose fixture
  cannot reach the arm its sentence names. Where else I would look: every row
  the unit added whose doc carries a "Red where …" sentence. Sure of the
  instances.
- **S7 (Q6).** Scheduled, each with an open row: one form (P1), the general
  anchor (P2), the image (P3), the centre-born far conic (P3). The re-seamed
  ellipse's `u·a` has a two-sided pin. Nothing disclosed is unscheduled. Sure.
- **S8 (Q2).** `conic.rs:944`–`:961`: eighteen lines of doc on
  `quarter_turned_where` for a body whose anchored arm is three. Mild.
- **S9 (Q4).** Structural-fix trap: the unit closes "a point is summed from
  the far centre" for the evaluator, and leaves the cover, the image and the
  distance floors summing from a centre that is now DERIVED, through
  `frame()`. Named in the PR body and scheduled, and still the shape. Sure.
- **S10.** Typos: "a ellipse" (`conic.rs:875`). Sure.

Ratings
- **Idiom / structure: 4.** One private placement behind named reads, and the
  re-seam is an exact permutation; the parallel `born` / `phase` fields cost
  it a point.
- **Test quality: 3.** The evaluator, the bound and the anchor's bits are
  pinned hard (A1–A4, A7, A8, R1, R2 red); eight mutants on the form's
  identity and the bound's charges survive, and the re-seamed arms are held
  only from `sweep`.
- **Doc / comment honesty: 3.** The PR body and the rows are candid about what
  is not done; the module doc contradicts itself on the anchor and one table
  carries a stale base.
