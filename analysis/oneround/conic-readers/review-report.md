# Review: PR 7510 `oneround/conic-readers` at `bd07a867e7` (reviewer r2)

Full review: the claims C1 to C10, then the style lane. Review branch
`oneround/conic-readers-review-r2`. The probes, the base twin, the
mutant runner and the raw sweep tables are under
`analysis/oneround/conic-readers/review-r2/`, and the probe module is
`crates/geom/src/cover/review_probes.rs` (`cfg(test)`, wired into
`cover.rs` on this branch only). I did not read the other reviewer's
branch, scratch files or any PR comment that summarises it, and I saw
none of it by accident.

## Verdict: NOT-MERGEABLE-AS-IS

What the unit adds to `Conic` is right in real arithmetic on both
placements, and a centre-held conic runs the formulas it ran before.
The arc-held cover at `Interval` holds every stored point I tried
(0 misses over 144 configurations). The gate holds the 208 reads it
lists. Two defects stop it:

- the `Interval` support of an arc-held cover along its own plane
  normal is about 1e280 to 1e293 m wide (it was 0 wide on main);
- the `f64` nearest distance is stated across a `1 − cos h`
  cancellation and overshoots a point of the arc by up to 8e-10 m
  (the very centre-scale loss the unit exists to remove).

Neither is seen by the unit's own row: its Interval half asserts only
`is_bounded`, and its one site stands 0.5 m off the plane. Five
unsound mutants of the new cover survive the author's row and the whole
`geom` suite.

## What I executed and what I only read

EXECUTED, each on its own `CARGO_TARGET_DIR` (head checkout, a head
worktree, a merge-base worktree):

- **The cover sweep** (`review_sweep_arc_held_cover`). Base twin:
  `review-r2/review_probes_base.rs`. My own double-double oracle, with
  an exact division and a range-reduced sin/cos, shares nothing with
  `conic.rs` but the stored data it reads. It covers 4 sizes
  (22 m to 2.2e7 m) × 4 ratios b/a (0.8, 0.1, 1, 3) × 9 windows (short
  at the anchor, reversed, off the anchor, 1 m arcs, long, past a half
  turn, whole, past whole) × 3 sites (the author's off-plane site, and
  two points of the arc), 513 samples each, at f64 and at Interval.
  Each run is on head and on the merge base.
- **Red probes**: `review_red_near_overshoots_on_a_short_far_centred_arc`
  (red on head, and red on base too with a smaller value);
  `review_interval_support_per_direction`; and
  `review_red_interval_box_of_a_world_plane_arc_held_ellipse` (green:
  with axis-exact data the support is tight).
- **Twelve mutants** (`review-r2/mutants.py`), each against the
  author's row and my sweep; the survivors were also run against the
  whole `cargo nextest run -p geom`.
- **The census, re-taken with clippy itself.** On base: the five doors
  in `clippy.toml`, a `warn` arming in every `crates/*/src/lib.rs`,
  `conic.rs` allowed. On head: every `#[expect]` turned to `#[warn]`.
  Both ran `cargo clippy --keep-going --workspace --all-targets
  --all-features`.
- **Gate bypass attempts** under clippy and
  `scripts/gates/conic-centre-reads.sh`, in a head worktree. The
  surviving file is `review-r2/gate_bypass_probe.rs`.
- **The gate on its own**: `--selftest`, then a plain run.
- **The differential, merge base against head**, `-p geom -p geom-brep
  -p topo` filtered to `cone|cover|arc_ball|depth_gap|section|extent|aabb`,
  at ε 1e-9 (the default, `DEFAULT_EPS`) and 1e-12 (C5/C6 below).
- **A classifier over every shared `.rs` file**, checking that each
  edit is one of the three named kinds (C10).

ONLY READ: the hosted CI run, which I did not look at; the rebuild
doors and the 142 listed functions, outside sampling; demos, benches
and tools.

## Findings

### MAJOR 1. The Interval support along the conic's own plane normal is useless

**DEMONSTRATED, sure.** For an arc-held ellipse with `n = (1,2,2)/3`,
`Cover::support(ivec(n))` at `Interval` reads
`[10.5, 1.009e280]` at a = 22 m and `[10.5, 1.009e292]` at a = 2.2e7 m.
At f64, and on the merge base at Interval, it reads `10.5` exactly.
Along `n` tilted by 1e-3 it is tight again. My sweep measures widths
of 1e278 to 1e293 in every configuration, against 8.6e-8 to 2.3e-6 on
base.

- **Cause** (`crates/geom/src/curves/conic.rs`, `offset_extent`). Along
  the normal, `p` and `q` are enclosures that straddle zero, so
  `amp + |k|` reaches 0. The divisor is floored at `f64::MIN_POSITIVE`,
  so `ratio = (p·s0 − q·c0)² / (A + |k|)` reaches `ε²/2.2e-308`. The
  comment there says the floor is reached "only where p and q are both
  zero and the ratio is 0": true of points, false of enclosures.
- **Who reads it.** It reaches every Interval reader of the arc-held
  extent:
  - `Cover::support`, `box_in` and `aabb`;
  - `conic_arc_extent_along_bracketed` and `ellipse_arc_aabb`
    (`curves/boxes.rs`), the certified lane, for an arc whose plane
    normal is read off the same data.
- **Not triggered** when the normal is axis-exact (the world-plane probe
  is green).
- **The row cannot see it.** `cover::tests::a_far_sections_cover_holds_its_stored_arc`
  reads exactly this direction (`dirs = [n, …]`) and passes, because it
  asserts only `x.is_bounded()`.
- **Impact on production callers is likely, not shown**: I did not find
  a production reader that goes red on it.

### MAJOR 2. The f64 nearest distance overshoots a point of its own arc at the centre's scale

**DEMONSTRATED, sure (value) / likely (severity).** `Arc::chord_range`
(`crates/geom/src/cover.rs`) shifts each piece by
`shift = 1 − cos(h)`. For the short pieces a far-centred arc is cut
into, `h` is below about 1e-7, so the subtraction keeps one or two
digits, and below 1.05e-8 it is exactly zero. The bulge it scales has
length about `a·h²/2`, so the nearest bound loses `u·a`: the derived
centre's rounding, which the arc-held form exists to avoid.

Read from points of the stored arc (true distance at most 3e-15 m):

- `cover near` = 7.97e-10 m at a = 2.2e7 (red probe);
- 9.6e-12 m at 2.2e5;
- 6.2e-14 m at 2.2e3 with b = a (my sweep, `short@anchor`);
- against the author's own bar of 3.6e-14 m.

**Attribution.** Written as `2·sin²(h/2)`, the same probe reads at most
3.6e-15 (run, then reverted). So the cause is the cancellation and not
the chord construction.

**Base also overshoots**, from its centre-frame circle:

| semi-axis | base | head |
|---|---|---|
| 2.2e6 | 1.16e-10 | 0 |
| 2.2e5 | 1.5e-11 | 9.5e-12 |
| 2.2e7 | 0 | 5.7e-10 to 8.0e-10 |

So the defect moved size rather than closing. The PR body's "the
nearest-distance bound never overshoots in either" is true only of its
one site, 0.5 m off the plane, which no chord comes near.

At f64 the module's convention is that nothing charges rounding
(`cover.rs` module doc, `[simple.conditioned]`). That covers rounding of
a conditioned formula, not a cancelling one: `offsets` in `conic.rs`
rewrites `cos θ − 1` as `−2·sin²(θ/2)` for exactly this reason. The
existing `Arc::far` has the same `1 − cos` (the class, see Style S2),
but there it only feeds a farthest bound met with the circle bound; the
new code feeds it to the nearest bound alone.

### MINOR 3. Unsound mutants of the new cover survive the row and the whole `geom` suite

**DEMONSTRATED, sure.** Each run was the author's row, then my sweep,
then `cargo nextest run -p geom` (583 run, without my probes):

| mutant | author row | my sweep (Interval misses) | whole geom |
|---|---|---|---|
| M1 cover on the derived centre frame | red | 0 (f64 red) | n/a |
| M2 extent on the derived centre | red | 0 (f64 red) | n/a |
| M3 bulge halved | **green** | 3.9e6 | **green** |
| M4 one of the four pieces dropped | **green** | 1.6e7 | **green** |
| M5 support at the ends only | red | 7.0e7 | n/a |
| M6 near without the bulge | **green** | 8.6e6 | **green** |
| M7 far without the bulged corners | **green** | 5.5e6 | **green** |
| M8 ball by the smaller semi-axis | **green** | 5.8e7 | **green** |
| M9 `v_ref` sign flipped (arc-held `Conic`) | **green** | 5.1e7 | red (5 rows) |
| M10 `u_ref`/`v_ref` swapped | red | 3.9e7 | n/a |
| M11 crest ratio → cancelling `A − k` | green | 0 | green |
| M12 chord ball halved | red | 4.3e7 | n/a |

The row pins one fixture: b = 0.8a, phase (1, 0), one anchor, one
symmetric short window, one site off the plane. Every piece of the new
cover (bulge, piece count, near, far, ball) can be weakened unsoundly
with nothing in `geom` going red. M11 is a conditioning mutant that no
fixture here separates. I did not run the survivors against
`geom-brep`/`topo`.

### MINOR 4. The gate passes a new production read under a lint group

**DEMONSTRATED, sure.** In a head worktree I added to `crates/geom/src`
(production, no `#[expect]` of the lint):

- `#[allow(clippy::style)] pub fn f(e) { Conic::frame(e).origin() }`
- `#[expect(clippy::style, reason = "…")] pub fn g(e) { e.center() }`

`cargo clippy -p geom --lib` is green, and so is
`scripts/gates/conic-centre-reads.sh`. `disallowed_methods` is in
`clippy::style`, and the gate's census only matches the lint's own
name. Clippy did catch every other form I tried:

- `Conic::frame`, `center`, UFCS, a function pointer, `.map(Ellipse::frame)`;
- a local `macro_rules!`, a generic `C: Conic`, `dyn Conic`, a closure;
- `Deref` to `Ellipse`, `Curve3::frame`.

Also green under both (BY-INSPECTION as to intent, DEMONSTRATED as to
passing):

- `Circle::placed(|f| …)` and `Circle::map_scalar(|f| …, …)` hand the
  centre frame to a caller's closure (`conic.rs:633`, `:642`);
  `Ellipse::mapped`/`map_scalar` hand the stored frame, which is the
  anchor's for an arc-held ellipse, so not the derived centre.
- `crates/viewer/src/bin/viewer.rs` is a crate root with no deny line;
  it reads no conic today.

The count file catches a new `#[expect]` function. It does not catch a
second read in a listed function (stated), nor any of the above. The
six planted reds are all shape cases of the expectation text; none
plants a read the lint does not name.

### MINOR 5. `Conic::unit_in_plane` reads the derived centre and is on no list

**BY-INSPECTION, sure.** The default `unit_in_plane` (`conic.rs:297`)
is `self.unit_of(p − self.frame().origin())`. `Ellipse` does not
override it, so on the arc-held path it reads the derived centre inside
the conic's module, where the lint is allowed. Three production readers
call it and are neither listed nor counted:

- `topo/src/splitting/section_loops.rs:447`
- `sweep/src/blend/open/end_face.rs:390`
- `geom-brep/src/curve_pair/pose/window.rs:442`

A deprecation census of the five doors cannot see a door inside
`conic.rs` that consumes the derived centre. The same blind spot covers
the derived frame `Cover::arc_of` stores. The brief's question ("is any
derived centre read on the arc-held path, through a helper?") is
answered yes here for `unit_in_plane`, and no for the new doors
(below).

### MINOR 6. The arc-held cover is far looser than the one it replaces on long windows

**DEMONSTRATED, sure.** `Cover::arc_of` does not meet the whole
conic's piece (the disclosed deviation), and `chord_ball` centres on
the arc's middle point with radius `speed_hi·2·sin(reach/2)`. Slack at
f64, base → head, on a = 2.2e7, b = 0.8a:

| window | ball | far |
|---|---|---|
| whole | 0 → 6.2e6 m | 0 → 4.4e6 m |
| past half a turn | 0 → 6.4e6 m | unchanged |

There is no change on short windows, where head is tighter (Interval
ball width 3e-14 against 4.7e-8 on base). Nothing is unsound. Who
relied on the meet I could not establish; today only
`plane_cone_section` makes arc-held ellipses. The deviation is
disclosed but no row schedules it.

### MINOR 7. The census counts do not reproduce under `--all-features`

**DEMONSTRATED, sure (counts) / unsure (consequence).** Judged against
`cargo clippy --keep-going --workspace --all-targets --all-features`,
counting the lint's hits outside `conic.rs` with production armed and
`cfg(test)` not:

- **On the merge base: 339 reads on 291 lines in 63 files**, against
  the author's 323/276/60.
  - Of the 339: `Circle::frame` 202, `Conic::frame` 81,
    `Ellipse::frame` 47, `Curve3::frame` 9.
  - They include the `test_support` feature code (5 hits).
- **On head: 208 reads on 193 lines in 57 files**, matching the
  author's 208/57. Per crate this is geom 15, geom-brep 67, topo 80,
  sweep 32, step-import 11, step-export 2, mesh 1. The functions match
  exactly (142; geom 8, geom-brep 49, topo 52, sweep 22, step-import 9,
  step-export 1, mesh 1).
- **So base → head removed 131 reads, not 115.** Classified by the
  method after the door, direction reads (`.axis()`, `.u_ref()`,
  `.v_ref()`) went 134 → 5. That is **129 directions converted, not
  112**, plus the cover/extent's 2 (one more on the
  `.origin`/argument side is the new `Cover::arc_of`).

The author's `cargo check --lib` likely did not build feature-gated
code. The head side agrees, so the gate's list is right; the headline
numbers in the PR body are not the tree's.

### NOTE 8. "112 read a direction only" includes directions read beside a centre to rebuild a placement

**BY-INSPECTION, likely.** Of the direction reads on the diff's added
lines, 65 of 141 sit on a line that also reads `frame().origin()`, as
in `(center, axis, u_ref) = (c.frame().origin(), c.axis(), c.u_ref())`
in about 20 functions. Those directions feed a placement rebuilt at the
centre, so by the brief's own test they are position reads. The
functions are listed (the centre read keeps the `#[expect]`), so
nothing escapes the gate. But no dependence on the derived centre was
removed at those sites, and "converted, same bits" counts them as done.
The bits are the same: `Frame::with_origin` keeps the motion's rotation
(`frame.rs:104`), and the differential below showed no verdict moving.

### NOTE 9. The row's corrections live only in the PR body

**BY-INSPECTION, sure.** The frozen head edits no row except germ's.
`work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md`
is unchanged from the merge base. Its spec section ("The unit
(2026-11-23)") exists only on `origin/main`, added after the branch
point.

The corrections the brief lists are true of the tree, checked on the
merge base:

- `ruled_region.rs` reads only a surface frame (`:378`);
- `mesh/src/curved.rs` reads none;
- `extent.rs`'s production body reads `Curve3::frame` once (`:1292`;
  `:1207–1209` sit in its tests module);
- `param_near` goes through `Conic::azimuth`.

Nothing records them in the tree, and nothing schedules the parts not
done (the cone section's image, the rest of the families, the
differential).

The spec asked that the cover close be judged against
`carrier_resolution`'s exact-rational oracle on the cone's section; the
row uses `Ellipse::anchored` and its own double-double oracle. That is
disclosed, but the spec's "must show" for the cone section is not met
as written. Row 1's probes A and B are, as stated, untouched. The germ
seed text is as described.

### NOTE 10. The shared ground carries only the three named kinds of edit

**DEMONSTRATED, sure.** I normalised base by
`.frame().{axis,u_ref,v_ref}()` → `.{…}()` and stripped the
`#[expect(…)]` blocks and the deny line from head, comparing
whitespace-insensitively. That leaves differences in four files:

- three are direction reads deliberately left on `frame()` (`chart.frame()`
  in `chart_singularity.rs` is not a conic; `curves.rs` `data()`;
  `sweep/src/test_support.rs`), so not edits;
- one is a rustfmt brace reflow in `sweep/src/blend/arms.rs`.

No other change.

## The claims

- **C1**: re-taken (MINOR 7, NOTE 8, MINOR 5). The blind spots are:
  - doors inside `conic.rs` that consume the derived centre
    (`unit_in_plane`, the frame `arc_of` stores);
  - group-level lint attributes;
  - closures handed the frame (`Circle::placed`, `map_scalar`);
  - `Curve3::data()` (stated);
  - crate roots without a deny line (`viewer`'s bin);
  - demos, benches and tools (stated).

  Fields are private to `conic.rs`, so no field or pattern read exists
  outside it.
- **C2**: per new door and placement.

  | door | centre-held | arc-held |
  |---|---|---|
  | `axis`, `u_ref`, `v_ref` | the stored frame's directions | the anchor frame's, which the derived frame shares bit for bit |
  | `extent_along` | `window.extent` at the stored centre's height, the formula `boxes.rs` had | the anchor's height plus `offset_extent`'s offsets: correct in ℝ, ill-conditioned at Interval along the normal (MAJOR 1) |
  | `cover_over` | `Cover::arc` on the stored frame, as before | `Cover::arc_of`: support from `extent_along`, ball and range from points and chords |
  | `ArcWindow::span` | the given ends | the given ends |

  On the arc-held path the derived frame is read only for
  `finite_frame` and stored unused. That has one consequence: an
  ellipse whose derived centre overflows while its points are finite is
  refused a cover. BY-INSPECTION, unsure it is reachable.
- **C3**: no Interval under-cover anywhere in my sweep (worst miss 0).
  At f64, the nearest bound under-covers (MAJOR 2). The bulge bound is
  a bound over the whole piece in ℝ: an ellipse is an affine image of
  a circle, so the piece lies in the parallelogram its chord spans out
  to the bulge `(1 − cos h)·(x(m) − c)`. No rounding is charged at f64,
  by convention; at Interval it comes from the arithmetic. The meet
  deviation claims less tightness, never more (MINOR 6).
- **C4**: the author's oracle shares `geom_core::exact` primitives and
  writes the same `−2·sin²(θ/2)` offset form as `conic.rs`'s `offsets`
  for phase (1, 0). Its sine series is valid only for |x| ≤ 0.3, so it
  cannot judge a long window.
  - It pins one fixture (MINOR 3).
  - A "miss" there is a point outside the cover. The bar permits that
    at f64 up to 3.6e-14, which is right for f64, but it also permits
    it at Interval, where an enclosure may not miss at all.
  - Interval widths at head for short windows, f64-exact data:
    - ball 1.2e-14 to 8.9e-14 at every size;
    - near 4e-14 (22 m) to 2.0e-8 (2.2e7 m);
    - far 1.1e-14 to 8.7e-9;
    - support 1e278 to 1e293 (MAJOR 1).

    Full table: `review-r2/sweep-head.txt`.
- **C5/C6**: see the differential section below.
- **C7**: MINOR 4 and MINOR 5. Every door that returns the derived
  centre directly is on the list. Of `conic.rs`'s public surface:
  - `Conic::frame`, `center`, `Ellipse::frame`, `Circle::frame` and
    `Curve3::frame` return it;
  - `major_frame`, `MajorFrame` and `section_frame` carry no centre;
  - `unit_in_plane` consumes it unlisted;
  - `Circle::placed` and `map_scalar` hand it to a closure.

  The selftest passes, and its six reds are all shape cases.
- **C8**: MINOR 3. The author's two mutants re-planted are red as
  claimed.
- **C9**: NOTE 9. The gate's 208/142/57 agree with the row's
  families as the PR body tables them.
- **C10**: NOTE 10. The `[lints]` tables in the five out-of-workspace
  manifests set `disallowed_methods = "allow"` (read). The PR body is
  true of the frozen head except:
  - the census figures (MINOR 7);
  - "the nearest-distance bound never overshoots in either" (MAJOR 2);
  - "each reading is bounded" at Interval, true only in the sense of
    finite (MAJOR 1).

## Differential and local runs (C5, C6)

**DEMONSTRATED.** `cargo nextest run -p geom -p geom-brep -p topo -E
'test(/cone|cover|arc_ball|depth_gap|section|extent|aabb/)'`, merge
base worktree against a head worktree, each on its own target
directory. The run includes my probes, which are on both trees.

| ε | base: run / failed | head: run / failed |
|---|---|---|
| 1e-9 (default) | 1058 / 3 | 1059 / 2 |
| 1e-12 | 1058 / 1 | 1059 / 2 |

Every red is one of three. None is the unit's.

- **My red probe (MAJOR 2)**, red on both trees at every ε.
- **`topo … square_search::the_square_wall_arm_agrees_with_the_traced_section`**:
  a random-seed fuzz row. It was red on base at 1e-9 and on head at
  both ε. Head's 1e-12 seed `0x3f9674328b8ceb68`, replayed on base at
  1e-12, is red there too (1 mismatch). This is the PR body's
  unnamed red, already germ's row.
- **`geom-brep pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual`**:
  red on base at 1e-9 only, a random-seed fuzz row. Seed
  `0xf79bc7c13a536d60` replayed on both trees is red on both. It is
  already recorded in
  `work/pcert/circle-lane-fuzz-draws-a-cone-section-that-is-no-ellipse.md`.

So no verdict moved on this set at 1e-9 or 1e-12. This is a verdict
differential only. I did not compare bits; the claim that a centre-held
conic keeps its bits still rests on the diff, which I read and agree
with (C2). The f64 vs Interval comparison per family is the sweep (C4),
which covers only the cover.

Full `cargo nextest run -p geom` on head: 651 run at ε 1e-6, at the
default and at 1e-12, with 650 passed each time. The one failure is my
red probe; the author's 647 plus my 4. Gate: `conic-centre-reads.sh
--selftest` OK, and the plain run OK. Not run by me: clippy with `-D
warnings` on the workspace (I ran it as the census instrument, with
warn arming), `demos/tour` clippy, `run-gates.sh` in full, and hosted
CI (not read).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, and Q8 (I read
`conic.rs` end to end). Each finding carries its confidence.

- **S1 (Q1), sure.** `Arc::chord_range` (`cover.rs`, new) is a second
  copy of `Arc::far`'s four-piece cut: the same `quarter`, `shift` and
  starts, and the same parallelogram argument re-derived in its doc
  ("over the four pieces `Arc::far` cuts"). The copy now also drives a
  nearest bound. `offset_extent` (`conic.rs`) is a near-parallel of
  `ArcWindow::extent`: the same crest/trough select, a different crest
  formula, and two spellings of "the extent of a harmonic over a
  window".
- **S2 (Q1/Q4), sure.** The `1 − cos h` cancellation appears twice in
  `cover.rs` (`Arc::far`, `Arc::chord_range`), while `conic.rs`'s
  `offsets` exists to avoid exactly this form. Sweep also: any `1 - …
  .cos()` in `crates/geom/src`.
- **S3 (Q2/Q5), sure.** The `Arc::on` doc says "`frame` is its frame at
  the centre it derives, read for directions alone". On the arc-held
  path `frame` is not read at all (support, range and ball all
  short-circuit), so the field exists only to carry a derived centre.
  That makes `Cover::arc_of` a new listed production read of the
  derived centre, added by the unit that closes such reads: the fix
  mints a fresh instance (the brief's trap), pinned at +1 in
  `crates/geom/src/cover.rs`.
- **S4 (Q3), sure.**
  `a_far_sections_cover_holds_its_stored_arc`'s Interval half asserts
  `is_bounded` (monotone in the wrong direction: a wider enclosure
  passes more easily), and its `≤ own` bar permits an Interval miss.
  Its single site cannot exercise the nearest bound. Its sweep varies
  only the size.
- **S5 (Q6), sure.** The disclosed deviations are not scheduled by any
  row or named unit: the missing meet, the reasons "of two kinds only",
  the differential not run, the 208 remaining reads. They appear in the
  PR body's "Not done" only.
- **S6 (Q1/Q7), likely.** `axis`, `u_ref` and `v_ref` are each written
  four times: the trait method, the `&C` forwarder, and two pairs of
  identical inherent methods on `Circle` and `Ellipse` that shadow the
  trait. The 142 `#[expect]` reasons are two strings copied 142 times.
  The `.counts` file is a hand-held census (the class this lane is
  warned about), though the gate does check it against the tree.
- **S7 (Q2), likely.** `offset_extent`'s comment "the divisor is floored
  at the least positive number, which it reaches only where p and q are
  both zero" is false of enclosures (MAJOR 1). A comment asserting a
  premise the scalar's lane does not satisfy.
- **S8 (Q5), likely.** The `cover.rs` module doc promises at f64 "the
  real-arithmetic bound computed as if exact". `chord_range` is not
  that bound computed as if exact, since its cancellation is not
  rounding of a conditioned formula. Related: `ParamWindow`'s doc says
  "the window computes nothing from them", while `chord_range` and
  `chord_ball` compute clamps from the ends.
- **S9 (Q8), unsure.** `conic.rs`'s module doc (lines 1 to 125) grows
  by unit-titled sections. The doors this unit adds are not reflected
  there; "every reading goes through the conic" sits beside a
  `unit_in_plane` that does not.
- **S10 (Q4), likely.** `cover.rs`'s `ball()` doc, "The whole conic's
  ball: its centre, out to the Gram bound", now opens with an
  early return to a different ball. The doc describes only the
  centre-held branch.

Ratings (1 to 5):

- **idiom/structure 3**: clean trait additions and a one-line
  `boxes.rs` reroute, but a parallel cut copied rather than shared
  (S1), and a dead derived frame carried in the arc-held piece (S3).
- **test quality 2**: one fixture, an Interval assertion that a
  1e292-wide enclosure passes, and five unsound mutants that survive the
  whole `geom` suite (MINOR 3).
- **doc/comment honesty 3**: the PR body discloses much, but states
  three things the tree contradicts (MAJOR 1, MAJOR 2, MINOR 7), and a
  code comment asserts a false premise (S7).
