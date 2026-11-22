# Merge read r2 of PR 7203 (`oneround/anchored-conic`), `b303a32a6..a7fe5cf4cc134b139dcd4af38fbb052b1259d722`

Frozen head `a7fe5cf4c`. Read: the hand work only, as
`git show --remerge-diff` of `e80763d8f` (32 files), `83d337786` (none) and
`a7fe5cf4c` (two tracker rows), `git diff fd7e5f47b e80763d8f` on the files
said to be main's side whole, and the three non-merge commits. Not main's
commits. Local, debug build, 4 cores.

I did not fetch or read the other reviewer's branch or any summary of it.

## Verdict: APPROVE

The hand work is what it is said to be: main's side re-pointed, the unit's
placement kept, nothing of the fix pass dropped. No MAJOR, no MINOR, four
NOTE. The hosted run had not reached a test or lint job when I read it, so
the verification below is local.

## Executed, and only read

EXECUTED on `a7fe5cf4c`
- `ci` nextest profile, whole workspace, clean worktree, at ε 1e-9 and
  1e-12.
- My first-review probe (one constructor call re-spelled, see M8): the
  12,228-line dump, compared with `b303a32a6`'s.
- A probe of the derived centre across `f64` seam turns: five frames, three
  sizes.
- Nine mutants, each in `geom` whole (628 rows).
- `scripts/run-gates.sh`, clean worktree.
- The hosted run 45071025483, read through the API at the job and step level.

ONLY READ: every hunk of the three remerge diffs; the non-merge commits; the
merge result against main's side for the twelve files of M6; the README
sentence; the rows. NOT RUN: ε 1e-6; clippy; rustdoc; the slow set.

## The claims

**M1. HOLDS** (likely). Every re-pointed site I read (`on_built.rs`,
`build.rs`, `parser.rs`, `on_surface.rs`, `winds.rs`, `junction.rs`,
`chart_iso.rs`, `circle_torus.rs`, `pcurve_cache.rs`, `join.rs`,
`offset_axial.rs`, `replace_face.rs`, the tests) reads the frame of the conic
main read: the field is private here, so an unconverted read does not
compile, and no converted one changes its object. Sites that now derive a
centre where main read a stored field, on an ELLIPSE arm:
`step-import/src/on_built.rs:385` (`Conic::of`), which sees only what an
import built, held at its centre; `on_surface.rs:758` (`plane`),
`offset_axial.rs` (the axis only) and `join.rs`'s four, each of which read
`frame()` on this branch before the merge as well. I found none new to an
arc-held ellipse's path. `Circle::with_frame`'s one caller is
`circle.placed(geom::Frame::reversed)`, which for a circle is the same
circle run back.

**M2. HOLDS, with one figure looser than stated** (sure of what I measured).
- `quarter_turned_where` takes `f.quarter_turned_renormalized()` at `f64`
  and `f.quarter_turned()` at `Interval`; nothing in `conic.rs` says the turn
  re-squares the frame (the two hits for the word are "not re-squared" and
  the `Interval` arm's counterfactual).
- The derived centre across a quarter turn, `|Δcentre| / (u·a)`, at
  `a` = 1e4, 1e5, 1e6: the rows' own fixture 0, 0, 0; four other frames
  1.23, 1.85, 1.48; 0.10, 0.00, 1.06; 0.41, 0.33, 0.26; 0.20, 0.33, 0.26.
  Across a half turn: 0 on all fifteen. So "still moves, by a rounding of
  the semi-axis, and not over a half turn" is true. The module doc's "0.5 to
  2.6 `u·a` measured on frames in general position" is those frames'
  figure: mine run 0 to 1.9, three of twelve under 0.5 (n1).
- The point of the turned ellipse at the anchor's parameter stands 0.74
  `u·a` from the anchor at all fifteen, which is the two-sided pin's
  regime (the row reads 0.66; inside [0.05, 8]).
- Re-planted: the quarter turn's phase dropped, RED (3 rows); the half
  turn's, RED (`a_re_seam_moves_parameter_zero_and_never_the_anchor`).

**M3. HOLDS** (sure). `e.same_placement(k, frame, &same)` is on the one
path; the form ignored is RED and the phase ignored is RED, both in
`the_walk_names_an_arc_held_ellipses_anchor_and_phase`.

**M4. HOLDS as a merge; what the oracle is has changed on main** (likely).
The resolution is main's frame reading with the unit's anchor arm laid on
it, nothing else. Main's `polar(&frame)` is the stored rotation's polar
factor, not the stored entries; for an arc-held ellipse `frame()` keeps the
stored rotation (only the origin is derived), and the anchor arm takes the
stored point and phase by name and subtracts `A·cos θ₀ + B·sin θ₀` with
that same `A`, `B`. So the oracle's arc-held curve is consistent with its
centre-held one. No row's truth moved that the suite can see (17,651 of 17,651 at ε 1e-9).
The oracle is no longer "the stored entries as reals" for either placement:
n2.

**M5. HOLDS** (sure). `plus_zero_frame` is gone; `Circle::placed` and
`Ellipse::mapped` carry `f.map_scalar(fold_signed_zero)`, which keeps an
ellipse held as it was. The phase is not folded, as stated.

**M6. HOLDS** (sure for the twelve files diffed). Against main's side the
merge result differs only by re-points in `circle_torus.rs` (5 lines),
`chart_iso.rs` (1), `pcurve_cache.rs`, `join.rs`, `offset_axial.rs`,
`replace_face.rs`, `closed_form_limb2.rs`, `props_cone_incidence.rs`,
`curve_promotion_report.rs`; `census/point.rs` is main's to the byte;
`in_band_sweep.rs` is main's file with the unit's row. Kept from the fix
pass: `pcurve_cache.rs`'s `centred_at`, `replace_face.rs`'s
`e.translated(delta)`, `build.rs`'s signature row. The doors on the G9
list in `offset_axial.rs` and `replace_face.rs` are `Curve3::Circle` arms
only. `cover-kind-match.counts`: `join.rs` 14, and the gate passes.

**M7. HOLDS** (sure). Each of the eight is in the tree: the row
`an_arc_held_ellipse_centred_elsewhere_is_translated_whole` (the mutant that
sets the origin is RED in `geom` now, where it survived at the delta read);
`Family { Circle, EllipseAtCentre, EllipseOnArc }` with no unreachable arm;
"held at its centre"; `Circle::placed`'s doc; `running_point`'s doc;
the three paths; the equal-semi-axes ellipse in
`the_running_residuals_are_the_plain_chain`; `Ellipse::mapped`'s doc.

**M8. HOLDS where I ran it.**

| run | this head `a7fe5cf4c` | judged against |
|---|---|---|
| `ci`, ε 1e-9 | 17,651 of 17,651 | the implementer's 17,635 of 17,635 on `30c37f7cd`, two merges of main back: all green on both, 16 more rows here |
| `ci`, ε 1e-12 | 17,650 of 17,651 | the implementer's 17,634 of 17,635: the same one red |
| `ci`, ε 1e-6 | not run | |

The one red at 1e-12 is `sweep::all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`,
`VolumeUncomputable`, `props_chart_junction_closed`, margin
2.8154559647670245e-12: the payload the implementer reports from main at
`d3d6534c3`, to the digit. I did not run main myself, so "main's red" is the
implementer's reproduction and the tesslie row's, matched by me only on the
payload (likely). `pcurve_villarceau` and the ringless cone face, red at the
delta read, pass: main cleared them.

**The hosted run 45071025483** (head `a7fe5cf4c`), read at the end of this
review: `change filter` completed, success, every step; `prime the build
cache` and the interval oracle skipped; the other eleven jobs (`test
(kernel)`, `test (app)`, `lint (fmt, clippy, gates)`, the mesh budget, the
demos, corrupt input, the wasm viewer, the python suite, step import, tools,
the interval backend) QUEUED, none started. It has not finished, and it has
said nothing yet about this head beyond the filter.

The dump: 12,228 lines, BYTE FOR BYTE `b303a32a6`'s. The probe's one edit is
its reach, `Reach::of_frame(at, &unit_cylinder)` for `Reach::of_frame(at,
(1.0, 1.0))`, both `Measured { at, lever: 1.0 }`. The dump takes no quarter
turn, so nothing in it could have moved for that reason; the quarter turn is
M2's probe.

**M9. HOLDS** (sure). The unit row's new section says what the code does,
and says in so many words that main's "an arc-held one has none" (a re-seam)
is not built: `Ellipse::seamed_near` turns the seam under the anchor. The
readers row keeps its id and its list with the corrected paths. The general
anchor's row cites the approved sentence. The README's "its parameter zero
there" is true of an ellipse as minted only, which the row says.

**Fresh instances of what the fix pass closed.** None in the hand work. One
that main brought: n3.

## Gates

`scripts/run-gates.sh`, `GATE_BASE` unset, resolved by the script to
`e68799906` (the merge base with `origin/main`): rc 0, every gate green.
Judged against the delta read, where two were red on head and base: both
were the base's, and main has since cleared them.

## Findings

No MAJOR, no MINOR.

- **n1. NOTE.** `conic.rs`, module doc: "0.5 to 2.6 `u·a` measured on frames
  in general position". On four frames of mine it is 0 to 1.9, a quarter of
  the readings under 0.5. A measured range stated as if it were the range.
  DEMONSTRATED, sure; nothing computes with it.
- **n2. NOTE.** The README sentence this unit's rows now cite says "a conic
  is the curve its stored numbers state". Main's oracle
  (`conic_oracle/exact.rs:143`–`:160`) reads the stored frame's polar
  factor, and the `f64` evaluator sums the stored entries; the two are the
  curves `work/onehome/a-stored-frame-reads-as-two-curves-…` holds apart.
  The merge applies main's reading to the arc-held arm as it must. Not this
  unit's to settle. BY-INSPECTION, likely.
- **n3. NOTE.** Main brought a production door that rebuilds a circle
  through a centre constructor and is not on the readers row's list:
  `step-import/src/build.rs:1976` (`column_circle`, the same circle with its
  axis reversed, re-squared by `Curve3::circle`). A circle, so it loses
  nothing today; it is the class the list is for. BY-INSPECTION, sure.
- **n4. NOTE.** `step-import/src/on_built.rs:372`–`:393` (`Conic::of`)
  restates an ellipse as `centre + a·cos + b·sin` from `frame()`. New from
  main, import-only today; one more reader for the readers row. Likely.

## Style

Exercised on the hand work: Q1 (the local `graze_point` beside main's
`graze` is a declared twin, kept for the anchored fixture), Q3 (the nine
mutants), Q4 (M2's sentences; n1), Q5 (the docs M7 names), Q6 (n1 is a
measurement with no guard and no need of one), Q7, Q8 (`conic.rs`'s changed
regions only this time, not the whole file). Nothing in the merge reads as a
second decision.
