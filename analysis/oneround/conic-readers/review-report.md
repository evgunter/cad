# Review of PR 7510 (`oneround/conic-readers`), reviewer r1

Frozen head `bd07a867e790d35fc4bc7d67e317ca5068e0e28f`, merge base
`2b9cd8b24c73769b84f1c20e3cbcd57310d0fbf2`. A full review: the ten
claims, then the style lane. I did not read the other reviewer's
branch, scratch files or any PR comment; of the PR I read its body
only.

## Verdict

**NOT-MERGEABLE-AS-IS.** One fix round away, and nothing structural.

- **No under-cover found.** At `f64` and at `Interval` the arc-held
  cover and extent hold every sampled point of the stored curve, over
  eccentricity, size, seam, reversal and window (13,000 fuzz cases,
  below).
- **A conic held at its centre keeps its bits**: shown, not argued
  (17,343 dumped readings, none moved).
- **What blocks**: at `Interval` the new arc-held extent returns a
  bound some 1e245 to 1e292 m wide along the ellipse's own stored
  normal, where the merge base returned 5e-14 (M1). The unit's row
  reads exactly that reading and passes, because it asserts only
  `is_bounded()`.
- **What also has to change before this is evidence**: the unit's one
  row pins a short symmetric window at the anchor, so six mutants of
  the new code survive the unit's own suite, four of them unsound
  (M2).

## Executed, and only read

**Executed** (all local; one `CARGO_TARGET_DIR` per tree):

| what | result |
|---|---|
| `cargo nextest run -p geom`, head, default ε, clean tree | 647 run, 647 passed |
| same at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`, with my probes in the tree | 651 and 652 run; the only red is my own probe for M1 |
| `-p geom-brep -p topo` filtered `cone cover arc_ball depth_gap section extent aabb`, head, default ε, random seeds | 983 run, 982 passed; the red is `the_square_wall_arm_agrees_with_the_traced_section` |
| that test on the merge base, at my seed `0xb91c779de973c230` and the author's `0x28ab5920d3c632ea` | red at both: it is `main`'s, not the unit's |
| the same filter, base and head, seed pinned, at default, 1e-12 and 1e-6 | identical pass/fail sets at each ε (the end of this report) |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings`, head, viewer included | clean |
| `scripts/run-gates.sh`, head with my surviving gate plants applied | exit 0 |
| my own deprecation census, base and head (`cargo check --workspace --lib --bins --all-features`, and `--all-targets`) | C1 |
| a bit dump of 31,887 readings on base and head, `f64` and `Interval` | C5 |
| three fuzz probes (6,000 + 3,000 + 4,000 cases) and four measuring probes | C3, C4 |
| 17 mutants on `geom` | C8 |
| 11 gate plants and 7 controls through clippy and the gate | C7 |

**Only read**: `crates/geom/src/curves/conic.rs` to line 1310 (every
non-test line), the diff of all 96 files, `cover.rs`'s public readings
and its `Arc` piece, the gate script and its counts, the two rows, the
PR body. Hosted CI run 45106429303 was still queued (`change filter`
job) when I looked; I read nothing from it.

**Not executed**: `demos/tour` clippy; a mutant of the inherent
`Circle::v_ref` / `Ellipse::v_ref` run against `geom-brep` and `topo`
(they survive `geom`'s suite, where nothing calls them; C8).

A disclosure on method: my first test job ran as a detached script of
four commands in sequence, and I briefly armed a waiter behind it,
which the brief forbids; I killed the waiter before it did anything.
While the first job's later steps ran I added my probe files to the
tree, so its 1e-6 and 1e-12 `geom` rows include them (hence 651 and
652 where the clean count is 647).

## Findings

### MAJOR

**M1. At `Interval`, an arc-held ellipse's extent along its own stored
normal is vacuous.** DEMONSTRATED, `sure`.

`offset_extent` (`crates/geom/src/curves/conic.rs:585`) reads the
crest that would cancel as
`(p·sin θ₀ − q·cos θ₀)² / (A + |k|).max(f64::MIN_POSITIVE)`. Along the
ellipse's own normal `p` and `q` are enclosures of zero with a width of
some `1e-16·a`, so `A + |k|` reaches zero, the floor makes the divisor
`2.2e-308`, and the ratio's upper end is `(1e-16·a)² / 2.2e-308`.

Measured, the author's own fixture (`n = (1,2,2)/3`, `b = 0.8a`,
half-window `3/a`), width of `cover.support(n)` at `Interval`:

| semi-axis | head | the same locus held at its centre (the merge base's reading) |
|---|---|---|
| 2.2e1 | 1.0e280 | 5.0e-14 |
| 2.2e3 | 1.0e284 | 3.8e-12 |
| 2.2e5 | 1.0e288 | 4.3e-10 |
| 2.2e7 | 1.0e292 | 4.2e-8 |

It is not a far-section effect: in the bit dump every arc-held ellipse
of that frame goes vacuous along its normal, at every size (0.9 m
included), every seam and every window, 112 readings of 112. The
`f64` door `conic_arc_extent_along_bracketed`, which lifts to
`Interval`, does the same: for `a = 2.2e4` over `[0, 0.3]` the merge
base returns `[10.749999999966922, 10.750000000033078]` and the head
`[−1.0e286, +1.0e286]`.

What it is not: unsound (the true value is inside), and not reached
along a world axis that the normal equals exactly (there `p` and `q`
are exact zeros; measured, width 0) nor along a direction a rounding
off the normal (measured, width 8.9e-16). It needs a direction the enclosure
cannot tell from the stored normal. That is the direction a section's
own plane has: `plane_cone_section` hands the plane's `n` to
`ellipse_anchored` (which stores `n` over its length), and
`topo` reads `cover.range_along(normal)` at
`census/in_plane.rs:237`, `splitting/containment.rs:1776`,
`chart_region.rs:1432`, `boolean/trace.rs:547`. Whether one of those
meets a section ellipse at `Interval` I did not establish (`likely`).

The unit's row reads this very reading
(`cover/tests.rs`, `dirs = [n, toward, …]`, `isupports`) and passes:
it asserts `x.is_bounded()` and `h − hi(sup) <= own`, both of which a
1e280-wide enclosure satisfies. The PR body's "each reading is bounded
and holds the same 65 stored points; its widths were not tabulated" is
true and is this defect. Mutant M15 (the crest read as `A − k`) makes
my probe for it green and reds nothing else, which locates it.

Probe: `cover::tests::review_probes::probe_interval_support_along_the_own_axis_is_of_the_arcs_size`
(red at the head), and `probe_interval_widths_by_reading`.

**M2. The unit's suite cannot see an unsound arc-held cover or extent
away from a short symmetric window at the anchor.** DEMONSTRATED
(surviving mutants), `sure`.

`a_far_sections_cover_holds_its_stored_arc` is the only row on the
arc-held path. Its windows are `[−h, h]` with `h ≤ 1.4e-2`, its
double-double sine is good to `|x| ≤ 0.3`, and it reads the support's
upper end only. Against it, and the rest of `geom`'s 647 tests:

| mutant | unit's suite | my probes |
|---|---|---|
| a piece's bulge halved (`chord_range`) | **green** | red |
| one of the four pieces dropped | **green** | red |
| the nearest bound without the bulge | **green** | red |
| the farthest at the piece's ends only, no bulge corners | **green** | red |
| the trough's sign in `offset_extent` (`amp − k` for `amp + k`): the extent's lower end short | **green** | red |
| `v_ref` negated on the arc-held path only | **green** | red |
| the `&C` forwarders of `extent_along` and `cover_over` dropped (a borrowed arc-held ellipse falls to the centre default) | **green** | **green** |
| the crest read in its cancelling form `A − k` | **green** | **green** |

The first five are under-covers. The sixth survives because the row's
window is symmetric, so the extent is even in `q`. The code is right
on all of them at the head (my probes pass there); the point is that
nothing the unit ships would say so tomorrow. The arc-held extent's
lower end is reached through `conic_arc_extent_along` and every arc
box, and no row of the unit reads it.

### MINOR

**m1. The arc-held cover claims less than the merge base did on a
long window, and one stated invariant is now false.** DEMONSTRATED,
`sure`. The stated deviation ("`Cover::arc_of` does not meet the whole
conic's cover") understates it: the four-piece chord bound is looser
than the centre form by a piece's bulge, `(1 − cos h)·a`, on both
sides.

- Full turn, `a = 0.9`, `b = 0.4`, from a point 0.57 m off the arc:
  the range goes `[0.458, 1.860]` to `[0.347, 2.006]`; the ball's
  radius 0.900 to 0.985.
- `a = 2.2e4`, window `[0, 0.3]`, the same site: nearest 0.458 to
  **0**.
- Over the dump's 1,008 arc-held `f64` distance ranges: nearest
  tighter in 515, **nearest lost to exactly 0 in 182**, farthest
  looser in 430.

`cover.rs:657–662` still says "no reader of an arc's cover reads past
its whole conic", and
`an_arcs_cover_reads_no_farther_than_its_whole_conics_piece` pins it
with `Ellipse::new` only; for an arc-held ellipse it is false (2.006
past 1.860 above). Every plane-cone ellipse is arc-held, at every
size, so this reaches ordinary sections, not far ones. I found no
verdict it moves (the seeded differential, end), and it is the safe
direction. The PR body's "BITS/VERDICTS MOVED: none observed" should
say that every arc-held reading moved and in which direction.

**m2. The `Interval` half of the row permits a miss.** BY-INSPECTION,
`sure`. `out <= own`, `lo(inear) − d <= own`, `h − hi(sup) <= own`
allow a point 3.6e-14 m outside an enclosure. An enclosure may not
miss at all; what the oracle owes is the rounding of its own last
subtraction. Measured over 3,000 cases the `Interval` ball, nearest
and farthest miss by exactly 0 and the support by at most 4.7e-17 of
the scale, so the bar can be the oracle's rounding.

**m3. The census does not reproduce, and "112 directions converted"
is short by eight.** DEMONSTRATED, `sure`. Mine, the same five doors
marked `#[deprecated]` on `2b9cd8b24`, spans de-duplicated by
`(file, line, column)`, outside `conic.rs`:

| | the PR | mine |
|---|---|---|
| production, `--lib`, no features | 323 reads, 276 lines, 60 files | 332, 285, 60 |
| production, `--lib --bins --all-features` | not taken | 339, 291, 63 |
| `Curve3::frame` callers | 12 | 9 |
| with tests, `--all-targets --all-features` | 852 in 189 files | 814 in 188 |
| remaining at the head, `--all-features` | 208 in 142 functions, 57 files | **208 in 142 functions, 57 files** |

The end state agrees exactly, and no file has a read without a pin or
a pin without a read. The starting count does not: the seven
all-features-only reads are in `sweep/src/test_support.rs`,
`topo/src/test_support_fixtures.rs` and
`curve_face/conic_oracle/exact.rs`, all three on the list, so the
census that produced "323" did not see sites the list holds. Of the
208 that remain, **eight are direction-only**
(`geom/src/curves.rs:1293` and `:1300`, two each;
`topo/src/boolean/vtxfac.rs:1319`; `topo/src/region.rs:1096`;
`topo/src/ring_path.rs:1603`; `sweep/src/test_support.rs:2053`), and
two listed functions read nothing else:
`Curve3::representability_margins` and
`boolean::vtxfac::read_vertex_on_face` carry "owed a reading through
the conic" for `frame().axis()` / `.u_ref()`, whose reading this PR
added. Four of the eight are a `.frame()` and its `.axis()` on two
lines. `Conic::center` has no production caller at all.

**m4. The gate lets a new production read through by three routes it
does not state.** DEMONSTRATED, `sure`. Planted in
`crates/geom-brep/src/extent.rs` (no pin); clippy with `-D warnings`,
`conic-centre-reads.sh` and the whole of `run-gates.sh` all pass
(`analysis/oneround/conic-readers/review-r1/gate-plants-that-pass.patch`):

- `#[allow(clippy::style)]` on a function that returns
  `c.frame().origin()`. The lint is in `clippy::style`; a group allow
  on an item overrides the root's deny, and the gate's `awk` looks for
  the lint's own name. `clippy::all` and `warnings` are the same
  route.
- `c.placed(|at| { o = at.origin(); *at })` on a `Circle`, and
  `e.mapped(|at| …)` on an `Ellipse`: the closure is handed the stored
  frame. For an ellipse that is the centre **or the anchor**, with
  nothing saying which, which is a worse read than the derived centre.
  `map_scalar` is the same door. None is on the list.
- `e.point_at(0.0) + e.second_derivative_at(0.0)`: the centre from two
  readings. This one is of the kind the header states ("restated
  without these doors").

Controls, all refused by clippy as they should be: a plain read, a
`&dyn Conic`, a fully qualified call, the path `geom::Conic::frame`
handed to `map`, a generic `C: Conic`, `Curve3::frame`, a closure, and
a read inside a local `macro_rules!`.

**m5. The gate's list is not the row's list.** DEMONSTRATED (census),
`sure`. The row names `crates/topo/src/boolean/mod.rs` (two sites, the
`Rim` round trip) among the doors that rebuild a conic from a centre.
They are at `boolean/mod.rs:8299` and `:8419`,
`Curve3::circle(rim.center, rim.axis, rim.radius, rim.u_ref)`: they
read a struct another function filled, so no door fires, the file has
no pin, and the PR body's rebuild-door row leaves it out. "Each listed
and pinned" holds for reads through the five doors, not for the sites
the row says remain.

**m6. `Cover::arc_of` adds a production read of the derived centre
that nothing uses.** DEMONSTRATED (mutant M13), `sure`.
`cover.rs:688` reads `Conic::frame(&ellipse)` under a fresh
`#[expect]` and stores it in `Arc::frame`, whose doc says it is "read
for directions alone". On the `on` path `support`, `range` and `ball`
all return before any field but `window` is read: replacing that frame
by one translated 1e9 m reds nothing, my probes included. So the
function that exists to stop reading the derived centre reads it, is
the 208th read on the list, and refuses the cover (`None`) where the
derived centre is not finite though the anchor is. See Style S1.

**m7. The PR body is wrong in four places.** `sure` unless marked.
(a) the census figures, m3. (b) "`geom-brep/src/extent.rs` has one
read, through `Curve3::frame` … `extent.rs`: listed": it has no
production read (all four are under `#[cfg(test)]`, from line 1126)
and no pin. (c) the per-file function counts in the family table do
not match the pins: `implicit.rs` 9 against 8 pinned, `pose.rs` 7
against 8 (`unsure` whether a family split explains them). (d) the
local clippy run excluded `viewer`; the brief I was handed says "the
workspace". It is clean with viewer in.

### NOTE

**n1. Wrong premises in the brief.** `sure`. (a) "the one that did
not pass is not named": the PR body names it. (b) "clippy on the
workspace": `--exclude viewer`, m7(d). (c) My branch existed locally
at `origin/main`'s tip, not at the frozen head; it had no commit and
no remote, so I re-pointed it at the frozen head. (d) The row's
"The unit (2026-11-23)" section is on `origin/main` only, not at the
frozen head.

**n2. The oracle is not the one the row's spec names.** The spec says
"the exact-rational oracle of `carrier_resolution::tests::…`", built on
`plane_cone_section`; the row uses its own double-double evaluation of
an `Ellipse::anchored`. Disclosed, and I think it is the better
judge of "the curve the stored numbers state". It is independent of
`conic.rs`: it shares `geom_core::exact::{two_sum, two_product}` and
reads the stored frame through the new `axis()` / `u_ref()` (a swap
there reds the row, mutant M10). `likely`.

**n3. A stored seam is up to 2.3e-14 off square to its axis, and the
extent takes it as square.** DEMONSTRATED, `likely` that it is the
base's and not this unit's. `Ellipse::anchored` with a `toward` near
the axis stores `u_ref·axis = −2.3e-14`; `along` reads the part of `n`
across the plane, so a stored point stands `a·(u·n)(n·d)` off the
extent: 7.5e-8 m at `a = 2.6e6`. That is larger than the misses this
unit removes (2.5e-10 at 2.2e7). The author's fixture has
`toward·n = 0` exactly and cannot see it. My probes net it out.

**n4. `crates/viewer/src/bin/viewer.rs` and 25 `examples/*.rs` are
unarmed roots.** BY-INSPECTION, `unsure` that it matters. The gate
asks the deny line of `crates/*/src/lib.rs` only. My plant in the
viewer binary did not compile for an unrelated reason (`geom` is a
dev-dependency there), so this is not demonstrated.

**n5. The six planted reds test the script's text, not the arming.**
`likely`. None shows that the lint fires on a read; that rests on
clippy and on the 142 expectations being fulfilled, which I confirmed
(my seven controls). None covers m4's routes.

**n6. The germ seed is true.** Red on a clean `2b9cd8b24` at
`0x28ab5920d3c632ea`, and at a second seed of mine.

## The claims

**C1.** Re-taken, m3: the end state is exact, the start is not, eight
direction reads are left. What a deprecation census cannot see, each
checked: a field by pattern (none: `Circle` and `Ellipse` fields are
private); a macro (0 of 339 in an expansion); a crate the build did
not check (the three all-features files; the out-of-workspace roots,
which I did not census); a read through a closure that is handed the
stored frame (`placed`, `mapped`, `map_scalar`: no
call of today's reads a conic's stored `origin()` within four lines,
by grep, 13 hits read and dismissed); a read of `Curve3::data()`'s stored point
(five mentions of the datum outside geom's curves, none taking a
centre from it, by grep); a struct that carries a centre someone else read (m5). Of
the converted reads, every one is `frame().axis|u_ref|v_ref()` to the
same method on the conic, checked mechanically on 73 files and by eye
on 6; the bits are the same by the dump.

**C2.** One sentence each.

- `axis()`, `u_ref()`, `v_ref()`. Centre-held: the stored frame's
  directions. Arc-held: the same stored directions, the frame standing
  at the anchor; `v_ref` is crossed on each read. True of both; no
  centre is derived.
- `extent_along(window, origin, n)`. Centre-held: the stored centre's
  height plus the harmonic's ends and crests (`ArcWindow::extent`).
  Arc-held: the anchor's height plus each end and crest as an offset
  from it. True of both at `f64`; at `Interval` true and useless along
  the normal (M1).
- `cover_over(window)`. Centre-held: `Cover::arc` on the stored frame,
  the window's piece met with the whole conic's. Arc-held: one piece,
  support from `extent_along`, ball about the middle's point, range
  from four chords and their bulges. Sound; looser on a long window
  (m1).
- `ArcWindow::span()`: the two ends as given, placement-free.

A derived centre **is** read on the arc-held path, once, directly, at
`Cover::arc_of`; its value reaches no reading (m6). `chord_range`,
`chord_ball` and `offset_extent` read points, `P″` and directions
only. The trait's **defaults** for `extent_along` and `cover_over` are
the centre form through `self.frame()`; they are true of an arc-held
conic only because `Ellipse` overrides both and `&C` forwards both,
and dropping the forwarders reds nothing (M2, row 7).

**C3.** No counter-example. `f64`: 6,000 cover cases (ball, eight
supports, four point sites and three line sites each, 401 samples) and
4,000 extent cases (both ends, seven directions); semi-axes 1e-2 to
3e7, ratios 1e-3 to 1e3, six seams, reversed or not, windows 1e-7 rad
to 13 rad in either orientation, anchors to 1e3 m. Worst miss, net of
n3: ball 6.9e-16 of the scale, support 8.4e-16, nearest 5.9e-16,
farthest 1.9e-15, extent 5.5e-16. `Interval`, 3,000 cases: ball,
nearest and farthest 0; support 4.7e-17; nothing unbounded. The bulge
bound holds over the whole piece: a point `δ` from the middle is the
chord's point at `sin δ` plus `(cos δ − cos h)·(x(m) − c)`, and
`cos δ − cos h ∈ [0, 1 − cos h]` for `|δ| ≤ h ≤ π`; a piece has
`h ≤ π/4`. No rounding is charged at `f64`, which is `cover.rs`'s
stated contract (`[simple.conditioned]`). Who relied on the meet with
the whole conic: m1.

**C4.** The oracle is independent (n2). The row pins one fixture: one
eccentricity, one frame with an exactly square seam, one anchor,
symmetric windows of at most 1.4e-2 rad (M2). "A miss" is a stored
point outside the cover's statement, positive outside; at `f64` the
bar permits it to 8u of the anchor's distance, which is the contract's
"as if exact"; at `Interval` the same bar is wrong (m2). The author's
two "before" figures reproduce (C8). The `Interval` widths, the same
fixture, now / the same locus centre-held:

| semi-axis | ball radius | nearest | farthest | support `u_ref` | support `n` |
|---|---|---|---|---|---|
| 2.2e1 | 1.1e-14 / 6.0e-14 | 4.0e-14 / 5.2e-13 | 1.1e-14 / 6.8e-14 | 2.5e-14 / 8.9e-14 | **1.0e280** / 5.0e-14 |
| 2.2e3 | 2.9e-14 / 5.5e-12 | 2.0e-12 / 5.0e-11 | 1.9e-14 / 5.0e-12 | 2.5e-14 / 9.5e-12 | **1.0e284** / 3.8e-12 |
| 2.2e5 | 2.7e-14 / 6.2e-10 | 2.0e-10 / 5.1e-9 | 2.0e-14 / 4.9e-10 | 2.5e-14 / 8.7e-10 | **1.0e288** / 4.3e-10 |
| 2.2e7 | 3.0e-14 / 4.7e-8 | 2.0e-8 / 5.0e-7 | 8.0e-9 / 5.4e-8 | 2.5e-14 / 8.6e-8 | **1.0e292** / 4.2e-8 |

Everything but the last column is as claimed and better than before.
The nearest's width still grows with the semi-axis (the bulge's norm),
and the farthest's jumps at 2.2e7.

**C5.** Taken. 31,887 readings on each tree: circles and ellipses
held at the centre and on the arc, three frames, sizes 0.4 to 2.2e7,
four seams, seven windows; ball, box, `aabb`, distance range from
three sites, support and extent along six directions, the bracketed
extent, `conic_arc_aabb`, `distance_within`, `clearance`, `along`,
`unit_of`, `semi_axis_vectors`, `param_in`, `half_turn_curve`; `f64`
and `Interval`.

- Held at the centre, circle or ellipse: **0 of 17,343 moved.**
- Held on the arc: 11,559 of 14,544 moved; `along`, `unit_of`,
  `semi_axis_vectors`, `param_in` did not. Against the stored curve
  each is sound (C3). Direction: at `f64` the support and the extent
  agree with the base to 1e-12 relative in all but three readings
  (those under 1e-6); M1's 336 readings are vacuous; the ball and the
  distance range are tighter on a short window and looser on a long
  one (m1).

The cover and extent code reads no ε, so one dump serves every ε. The
verdict differential is the seeded suite runs, at the end.

**C6.** As in the table at the top. The one red is
`the_square_wall_arm_agrees_with_the_traced_section`, `main`'s.

**C7.** m4, m5, n4, n5. The count file catches a new listed function
and a listed function gone; it does not catch a second read in a
listed function (stated), nor a read moved between two functions of
one file. Every expectation sits on a `fn` (142 of 142; none on an
`impl` or a `mod`), and each of the two reasons matches the doors its
function reads (0 mismatches of 142). Doors of `conic.rs` that hand
out or admit the centre: `Conic::frame`, `Conic::center`,
`Circle::frame`, `Ellipse::frame` (listed); `Circle::placed`,
`Circle::map_scalar`, `Ellipse::mapped`, `Ellipse::map_scalar` (the
stored frame, to a closure; unlisted); `Conic::unit_in_plane` (a point
less the centre); `P + P″`; `Curve3::data()` (stated).

**C8.** Both of the author's mutants are red as claimed, on
`a_far_sections_cover_holds_its_stored_arc`: the cover back on the
derived centre at 2.2e3 (farthest, 4.04e-14 past 3.65e-14); the extent
alone at 2.2e4 (the ball, 1.95e-13). Mine: M2's table, plus the
support at its ends only (red on the unit's row), the ball's chord
halved (red on the unit's row), the trait's `v_ref` negated for every
ellipse (red on five existing box rows), `u_ref` and `v_ref` swapped
(red on twelve). The inherent `Ellipse::v_ref` and `Circle::v_ref`
negated are green in `geom`, where nothing calls them; I did not run
them against `geom-brep` and `topo`, where the 112 converted sites
are. Patches: `analysis/oneround/conic-readers/review-r1/`.

**C9.** `ruled_region.rs`, `mesh/src/curved.rs`: no read, true.
`extent.rs`: no production read at all (m7b). `settled_foot`: no
frame, true. Row 1's probes A and B: untouched, true, and stated.
Row 2: not done, stated. The germ seed: true (n6). The gate's list
against the row: m5.

**C10.** Every changed `.rs` outside `crates/geom` carries only the
three stated kinds of edit (85 `.rs` files changed; 73 checked by
normalising the base and comparing, 6 by eye, 6 are geom's own). The
five `[lints]` tables are needed: the root `clippy.toml` reaches those
roots and the lint warns by default. The PR body: m7.

## Style

Questions exercised: Q1 (prose sweep and a look for undeclared
copies), Q2, Q3, Q4, Q5, Q6, Q7, Q8 (`conic.rs` end to end to its test
module; `cover.rs` in parts only).

**S1. The fix mints the defect it closes** (the lane's own warning).
`cover.rs:354–361`, `:687–698`. `Arc` gained `on: Option<Ellipse<T>>`
beside `frame`, `semi_u`, `semi_v`, and every reading opens with
`if let Some(ellipse) = &self.on { return … }`. On that path the three
old fields are dead, yet `arc_of` derives the centre frame to fill
them and takes a new `#[expect]` to do it. Two shapes are living in
one struct; an arc piece that is either "about a frame" or "through an
ellipse" would have no field to fill with a derived centre. `sure`.

**S2. Two near-copies of one bound.** `cover.rs:1485` (`chord_range`)
and `:1623` (`Arc::far`) both cut four pieces and shift the chord by a
bulge; the new one says so ("the four pieces [`Arc::far`] cuts … as
there"). They differ in where the bulge comes from (`−P″` against
`point − origin`). Likewise `offset_extent` (`conic.rs:585`) and
`ArcWindow::extent` (`boxes.rs:132`) share the end/crest/trough
selection line for line. The centre form could be the anchor form at
phase "centre", or the reverse; as it stands a fix to one will not
reach the other. Where else: `Arc::ball` against `chord_ball`. `likely`.

**S3. The same three one-line methods are written five times.**
`conic.rs:223–229` (trait), `:402–410` (`&C`), `:665–681` (`Circle`
inherent), `:708–718` (`Circle` impl), `:981–997` and `:1160–1170`
(`Ellipse`). The inherent and trait copies can drift and no test ties
them: the inherent `v_ref` negated is green in `geom`. `Ellipse::frame`
delegates to the trait; these do not. `likely`.

**S4. A default that is the thing the unit forbids.**
`conic.rs:306–316`. The trait's default `extent_along` and
`cover_over` read `self.frame().origin()` under docs that say "stated
from the point the conic stores". That is true of `Circle` and false
of the default as written; it is kept true by an override and two
forwarders that no test holds (M2). A required method, or a default
that cannot be reached by an arc-held conic, would not need the
sentence. `likely`.

**S5. The module doc still teaches the read the gate forbids.**
`conic.rs:8–11`, `:45–48`, `:73–76`: "read at the conic's centre
([`Conic::frame`])", "a reader that wants a centre as a datum reads
that", "a position is read as the centre ([`Conic::center`], the
origin of [`Conic::frame`])". The module's header does not mention
`axis`/`u_ref`/`v_ref`, `extent_along` or `cover_over`. Q4: the doc
rotted while the code moved on; fix the sentences. `sure`.

**S6. A comment that is not true.** `cover.rs:357–360`: `frame` "read
for directions alone" (it is not read; m6). And `cover.rs:1657`,
"The whole conic's ball", now heads a function that returns an arc's
ball on one branch. `sure`.

**S7. A test that gets easier as the guarantee degrades** (Q3's second
shape, exactly). `cover/tests.rs`, the `Interval` half: `is_bounded()`
plus one-sided containment passes a 1e280-wide enclosure and would
pass any wider one short of infinity. It needs a width ceiling. `sure`.

**S8. `conic.rs` now depends on `cover` and `boxes`, and `cover` on
`Ellipse`.** `conic.rs:136–138`, `cover.rs:38`. The curve's trait
returns a `Cover` and takes an `ArcWindow`; the cover's `Arc` piece
holds an `Ellipse` and `Ellipse::cover_over` matches on `held`.
`cover.rs`'s header says geom's two readings "are the only kind
matches". A cycle between the conic and its region, where before the
cover took a frame and two sizes. Not how I would have cut it, and I
was told not to rule on the interface's shape, so only: it is there.
`unsure`.

**S9. Two reason strings pasted 142 times.** Disclosed ("of two kinds
only"). They are consistent with the doors read (C7), which is more
than I expected, but they say nothing a reader of the site can act on,
and two of them are wrong in the sense of m3 (the reading they are
"owed" exists). A disclosed narrowing with no row of its own that I
could find. `likely`.

**S10. Disclosed and unscheduled** (Q6). "`Cover::arc_of` does not
meet the whole conic's cover" sits under "Not done" in the PR body
with no `work/` row; so does the differential. The unit's row stays
open and may be meant to carry both; it does not say so. `likely`.

**S11. The gate keeps its door list twice.** `conic-centre-reads.sh`'s
`DOORS` array and `clippy.toml`. The script checks the second against
the first, so a door added to one only is caught in one direction
(dropped from `clippy.toml`: red; added to `clippy.toml` only:
silent, and harmless). A hand-written list guarding a hand-written
list. `unsure` that it matters.

**S12. `demos/tour` reads `circle.frame().origin()` "as a user
would".** If a user is meant to read a conic's centre that way, the
public surface and the kernel's rule disagree, and the manifests'
`allow` is where that disagreement is parked. `unsure`.

### Ratings

- **Idiom and structure: 3.** The conversions are clean and exactly
  mechanical; the new `Arc` is two shapes in one struct with a dead
  derived-centre read (S1).
- **Test quality: 2.** One fixture, one window shape; six mutants of
  the new code survive, four unsound, and the row passes a 1e280-wide
  enclosure it reads itself (M1, M2).
- **Doc and comment honesty: 3.** The PR body is unusually candid
  about what was not done and both mutant claims reproduce; but its
  census does not, `Arc::on`'s doc is false, and the module doc still
  teaches the centre read (m3, S5, S6).

## C5/C6: the seeded differential

`cargo nextest run -p geom-brep -p topo` filtered to
`cone cover arc_ball depth_gap section extent aabb`, 983 tests, every
fuzz seed pinned to `CAD_FUZZ_SEED=0x5eedc0de00000007` on both trees:

| ε | merge base `2b9cd8b24` | frozen head `bd07a867e` | the reds |
|---|---|---|---|
| default | 983 passed | 983 passed | none |
| 1e-12 | 982 passed, 1 failed | 982 passed, 1 failed | `topo boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` on both |
| 1e-6 | 981 passed, 2 failed | 981 passed, 2 failed | `geom-brep pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual` and `geom-brep::all pair_section::one_loop_keeps_its_order_near_the_node` on both |

The pass and fail sets are identical, test for test, at each ε. So
**no verdict these 983 rows pin moved**, and the three reds are the
merge base's (I did not look into them further; the 1e-6 pair may be
worth a row if they are not known). What this does not show: the rows
were not written to meet an arc-held ellipse on a long window, so m1's
loosening has no row that could have moved.

Judged against: the same test binary's verdict on the merge base, same
seed, same ε. It is a differential of verdicts, not of bits; the bits
are C5's dump.
