# Delta read of PR 7510 (`oneround/conic-readers`), reviewer r1

Frozen head `32d065158ffb963eb6f985437a97810752b9b31e`. I reviewed
`bd07a867e7`. Main as the lane merged it: `c9cf004f9` (the merge's
second parent); every "main" below is that commit.

A glimpse to disclose: the brief and the rewritten PR body both state
findings of the other review (the `f64` nearest-distance overshoot of
8.0e-10 m at `a = 2.2e7`; "ball by the smaller semi-axis" among the
mutants; the second oracle in `held_arc_sweep.rs`, which is in the
tree). I read nothing else of that lane: no branch, scratch, CI
artifact or PR comment.

## Verdict

**APPROVE-WITH-FIXES.** Both of my MAJORs are closed and I could not
reopen either. Nothing I found blocks; the fixes are text and one
gate hole.

- **H1 holds.** At `Interval` the arc-held extent is an enclosure of
  the arc's own size along the stored normal, beside it, scaled, and
  with the anchor at a crest or a trough: worst width 2.1e-14 of the
  scale over 4,000 cases and 13 direction families, no miss.
- **H3/H4 hold.** 21 mutants re-planted on the frozen head, all red
  on the head's own rows, the eight survivors of the first head among
  them.
- **A conic held at its centre moved by one or two ulps in its
  farthest bound and nowhere else**, in both directions; "can only
  rise" is not true of the bits (m1).
- **No verdict moved** against main at three ε (the table at the
  end).
- **H9's "not past ε at a buildable size" is false of STEP import's
  placement mint** (m2): on main's code, not this unit's.

## Executed, and only read

**Executed**, all local, on the frozen head unless a tree is named:

| what | result |
|---|---|
| `cargo nextest run -p geom`, default ε | 655 run, 654 passed, 1 red: `curves::on_carrier::rows::a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound` |
| the same on main `c9cf004f9` | 645 run, 644 passed, **the same test red**: it is main's (claim 10) |
| `-p geom-brep -p topo` filtered `cone cover arc_ball depth_gap section extent aabb`, seed pinned, main and head, at default / 1e-12 / 1e-6 | the table at the end |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings`, viewer included | clean |
| `scripts/run-gates.sh`; `conic-centre-reads.sh --selftest` | exit 0; self-test OK |
| my deprecation census, the eight doors, `--lib --bins --all-features` | claim 7 |
| the 31,887-reading bit dump, main against head | claim 2 |
| a centre-held sweep of the farthest bound, 8,000 cases, main and head | claim 2 |
| an `Interval` extent sweep along and beside the normal, 4,000 cases | claim 1 |
| 21 mutants on `geom` | claim 3 |
| 3 gate plants through clippy and the gate | claim 6 |
| a seam-squareness probe by mint | claim 8 |

**Only read**: the diff of `conic.rs`, `cover.rs`, the gate script, the
two new rows, the unit row's corrections, the PR body; the two new
test files in outline. Every changed `.rs` outside `crates/geom` was
compared against main mechanically (below), not read line by line.

**Not executed**: `demos/tour` clippy; hosted CI (not read); the full
unfiltered suites; `sweep`, `mesh`, `step-import` at any ε; a STEP
file through the importer end to end (m2 is measured at the mint).

## Findings

### MAJOR

None.

### MINOR

**m1. "The bound can only rise" is not true of a centre-held conic's
bits.** DEMONSTRATED, `sure`. `bulge_factor` in `Arc::far` is the one
centre-held formula that moved. Main against head, 17,343 centre-held
readings dumped:

- 17,061 identical. The 282 that moved are all distance ranges: 29 at
  `f64`, 253 at `Interval`. No nearest bound moved at `f64`.
- Of the 29 `f64` farthest bounds, **19 rose and 10 fell**, each by
  one or two ulps of its own value (at most 2.1e-16 relative; the
  largest fall is 7.5e-9 m on a 3.8e7 m reading). `2·sin²(h/2)` and
  `1 − cos h` round differently on a long window, either way.
- At `Interval` the upper end fell in 213 and rose in 11; both are
  enclosures.

Is it still a bound: over 8,000 centre-held circles and ellipses
(windows from 1e-9 rad to past a turn, six sites each, 401 samples)
the worst miss of a sampled point past the farthest bound is
1.33e-15 of the scale on main and on the head, the same case and the
same bits; nearest 2.65e-15 on both; ball 4.5e-16 on both. So no
stored point stands outside a range it was inside, to the samples'
rounding, and no verdict moved (the end). What is wrong is the
sentence in the PR body, and that the move has no centre-held row of
its own: the mutant that puts `1 − cos h` back reds two arc-held rows
only. (Dropping the centre-held bulge altogether reds six existing
rows, so the bound itself is pinned.)

**m2. H9: a mint on main stores a seam far enough off square for a
stored point to stand ε off its extent at 2 m.** DEMONSTRATED at the
mint, `sure`; not this unit's code.
`geom_core::RigidMotion::from_axis_and_reference`, called as
`step-import` calls it for every `AXIS2_PLACEMENT_3D`
(`entities.rs`, `placement_built`: the site `"step_placement_axes"`,
`Band::least()`), 4,000 draws a row:

| reference off the axis | refused | worst `|u_ref·axis|` | a stored point stands 1e-9 m off at |
|---|---|---|---|
| 1e-2 rad | 0 | 4.6e-14 | 2.2e4 m |
| 1e-4 rad | 0 | 4.8e-12 | 210 m |
| 1e-6 rad | 0 | 4.4e-10 | **2.3 m** |
| 1e-7 rad | 0 | 5.0e-9 | **0.2 m** |

The door's doc says only an exactly parallel pair refuses, and that
is what it does. `Frame::new` reads the same (5e-16 over the angle),
and `Curve3::ellipse_anchored`, the deciding door, accepts a seam
1e-7 rad off the axis (skew 1.9e-9). A re-seam adds nothing: 1.7e-16
as minted, after one quarter turn, and after a thousand. So the
claim holds for `plane_cone_section` and for re-seams, and fails for
a file whose reference direction lies within about 1e-6 rad of its
axis, which the format allows. The row
(`work/onecopy/a-conics-harmonic-takes-its-stored-seam-as-square-to-its-axis.md`)
should carry this table and lose "under ε" as its general statement;
whether that changes its band is the row owner's. I did not push a
STEP file through the importer.

**m3. The gate passes a read under the lint's retired name.**
DEMONSTRATED, `sure`. Planted in `crates/geom-brep/src/extent.rs`:

```rust
#[allow(renamed_and_removed_lints)]
#[allow(clippy::disallowed_method)]
pub fn f(c: &geom::Circle<f64>) -> Point3<f64> { c.frame().origin() }
```

`clippy::disallowed_method` is the lint's old name; clippy maps it to
the new one and warns only through `renamed_and_removed_lints`. The
gate's `awk` looks for the string `clippy::disallowed_methods`, which
`clippy::disallowed_method)` does not contain. Clippy with
`-D warnings` is clean and the gate exits 0
(`review-r1-delta/gate-plants-that-pass-on-32d065158.patch`). Not in
the header's list of what it cannot see.

Also passing, and of a kind the header half-states: a whole circle's
cover's ball stands at its centre
(`c.cover_over(full turn).ball().0`), one reading and no door. A read
inside a macro that **another workspace crate** exports is refused by
clippy, as it should be.

**m4. The H1 row's ceiling is absolute and its fixture is one frame.**
BY-INSPECTION, `sure`.
`the_interval_support_along_the_arcs_own_normal_is_of_the_arcs_size`
asserts `worst < 1e-6` m at every size from `a = 2` to `2.2e7`. The
reading is about 1e-15 m at `a = 2`, so the row passes an enclosure
degraded by nine orders there. Its one normal is a rounding off the
world z. It does go red on the defect it was written for (the ceiling
dropped: red, 1 row). My sweep over generic frames reads 4.5e-15 of
the scale along the normal and 2.1e-14 at worst anywhere; a ceiling
in units of the scale would hold that.

**m5. Rows carry their probe's text.** `sure`.
`held_arc_rows.rs:315`: the row's doc opens "**RED at the frozen
head.**" and describes the defect as present; it is my probe's
comment, and the row is green. `held_arc_sweep.rs:257` tells the
reader to run `cargo test -p geom review_probes`, a module that does
not exist. The rows print `PROBE …` and `AXIS …` lines and compute a
centre-held "before" that only the print reads.

**m6. Two citations quote a README sentence that main has replaced, and the module doc is half-fixed.**
`sure`. `scripts/gates/conic-centre-reads.sh:6–7` and
`clippy.toml:17–18` quote "Every reading of where a conic is goes
through the conic" as the rule they serve, from
`crates/geom-core/README.md`; the README on this head
says "A conic's position is the point it stores plus two vectors …"
and no longer contains that sentence. And `conic.rs:73–75` still says
"a position is read as the centre ([`Conic::center`], the origin of
[`Conic::frame`])", one paragraph above the new one that says a
reader asks the conic (my S5, half-fixed).

### NOTE

**n1. H6 stands as a deviation, and I could not falsify "no verdict
moves".** The filtered suites agree with main test for test at three
ε. By inspection the route by which it could move one is a lever: a
whole section ellipse's ball goes from `a` to `√(a² + b²)` (0.900 to
0.985 measured at the first head), and `ExtentBall::of_covered` feeds
`Margin::levered`. `unsure` that any band edge is near enough. P2
reads as the band `work/README.md` gives it: not a broken verb (P0),
not architecture that entrenches (P1), and it matters soon because it
reaches every plane-cone ellipse. `likely`.

**n2. Claim 4 holds.** `Piece` is private to `cover.rs`; `HeldArc` is
named in `support`, `distance_range`, `about` and `shape_ball`, and
the one wildcard (`circle_range`'s `_ => whole()`) reads it through
its own range and support. No reader takes it as nothing or as a ball
about a derived centre. `arc_of` reads no frame; the mutant that put
a frame back is no longer plantable.

**n3. Claim 7 reproduces.** On the head: 207 reads on 193 lines in 58
files, in **144 functions**, each file's count equal to its pin, no
read without a pin and no pin without a read; no listed function
reads a direction only (one direction read is left, beside a centre
read on its line).

**n4. Claim 9 holds.** Against main, every changed `.rs` outside
`crates/geom` differs only by the unit's three kinds of edit (78
files compared after normalising, 4 by eye), and the head adds 144
expectations where main has none. So nothing of main's was dropped in
the five conflicted files or elsewhere, and the re-derived edits are
the same edits.

**n5. Claim 10: main's.** `a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound`
is red on `c9cf004f9` with none of the unit's code. I did not run it
on `origin/main`'s newer tip (`d23a330e6`).

**n6. The disclosed gap stays a gap.** No planted red runs clippy;
the self-test (ten reds, one pass) tests the script's text. My
controls at the first head and the external-macro plant here are the
only evidence in either review that the lint fires.

## The claims

1. **H1.** Not falsified. `over_anchor`'s ceiling `A + |k|` bounds
   `A − k` for every sign of `k`, and the interval `min` of two
   enclosures of numbers with `x ≤ y` encloses `x`; the divisor's
   floor is reached only where `A` and `k` both enclose zero, where
   the ceiling is small. Measured: widths at most 2.1e-14 of the
   scale, misses at most 1.8e-16 (the samples' rounding), nothing
   unbounded, over the normal, its negative, 3.7 times it, the normal
   plus 1e-17 to 1e-8 of a seam direction, the anchor at a crest, at a
   trough, `k` near zero, `u_ref`, `v_ref` and a random direction. The
   ceiling's looseness: m4. Unsound ceilings (`A`, `A + k`) red four
   rows each.
2. **H2.** m1.
3. **H3, H4.** All 21 red on the head's rows: the cover and the
   extent back on the derived centre; a bulge halved; a piece dropped;
   the nearest without its bulge; the farthest at the ends; the
   trough's sign; `v_ref` negated on the arc-held path only; the `&C`
   forwarders dropped; the crest in its cancelling form; the ceiling
   dropped, at `A`, at `A + k`; the select reversed; the bulge factor
   halved, and as `1 − cos h`; the ball's reach uncapped past half a
   turn, and by the smaller semi-axis; the bulge read at the piece's
   start; an end's phase swapped; the centre-held bulge dropped.
   Patches and results: `review-r1-delta/mutants/`.
4. **H5.** n2.
5. **H6.** n1.
6. **H7.** m3. The group mentions are refused, the binary root is
   armed, the six newly listed functions are pinned.
7. **H8.** n3.
8. **H9.** m2.
9. **The merge.** n4, and the runs above.
10. **The red.** n5.

## Style

Questions exercised: Q2, Q3, Q4, Q5, Q6 on the delta; Q1 on
`over_anchor` / `bulge_factor` / `HeldArc`. Q8 not repeated.

- **S1** (`sure`). m5: a row whose doc says it is red, and a pointer
  to a module that is not there. Rows made from a reviewer's probes
  kept the probes' voice.
- **S2** (`sure`). m6: a gate that quotes a sentence its README no
  longer has. Q4's first sub-case: the doc moved and the citation did
  not.
- **S3** (`likely`). `held_arc_rows.rs` judges the cover against
  `Ellipse::point_at`, the evaluator under test, and subtracts twice
  the frame's skew from every miss. The header says "points of its
  stored curve". The independent oracle is the other file's; this
  one's bar excuses exactly the effect m2 measures.
- **S4** (`likely`). `HeldArc::pieces` and `Arc::far` still cut the
  same four pieces in two places, now sharing `bulge_factor` and
  nothing else. The row filed for H6 proposes choosing the count from
  the window, which would have to be done twice.
- **S5** (`unsure`). `Cover::circle` now carries an `#[expect]` whose
  reason speaks of "a point read about a centre an arc-held ellipse
  derives"; it builds a circle from a `Core` the caller holds and no
  ellipse is in sight. The third reason string is one sentence for
  three doors.
- **S6** (`likely`). The trait's default `extent_along` and
  `cover_over` still read `self.frame()`; the doc now says so ("this
  body is a conic held at its centre's; a kind that can be held
  elsewhere states its own"). That is a comment asserting an invariant
  the type does not enforce, though the forwarders now have a row.

### Ratings

- **Idiom and structure: 4.** `HeldArc` is its own piece and reads no
  frame; `over_anchor` is one formula.
- **Test quality: 4.** 21 of 21 mutants red, both lanes, both ends;
  the H1 ceiling is loose and one file's oracle is the evaluator.
- **Doc and comment honesty: 3.** The PR body states the loosening
  and the unmeasured paths plainly; "can only rise", a row that calls
  itself red and a quoted sentence that is gone are still in.

## The seeded differential

`cargo nextest run -p geom-brep -p topo` filtered
`cone cover arc_ball depth_gap section extent aabb`, every fuzz seed
pinned to `0x5eedc0de00000007`, main `c9cf004f9` against the head:

| ε | main | head | the reds |
|---|---|---|---|
| default | 1003 passed | 1003 passed | none |
| 1e-12 | 1002 passed, 1 failed | 1002 passed, 1 failed | `topo boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` on both |
| 1e-6 | 1001 passed, 2 failed | 1001 passed, 2 failed | `geom-brep::all pair_section::one_loop_keeps_its_order_near_the_node` and `geom-brep pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual` on both |

The pass and fail sets are identical, test for test, at each ε, so
the three inherited reds are main's by a run and not by name. The
second 1e-6 red, `a_circles_projected_envelope_covers_its_residual`,
is not in the lane's account; it is red on main at this seed too, as
it was at the first head. At the default ε the square-wall test
passes at this seed on both trees.

Judged against the same binary's verdict on main at the same seed and
ε. It is a differential of verdicts; the bits are claim 2's dump.
