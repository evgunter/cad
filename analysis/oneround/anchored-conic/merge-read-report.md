# Merge read r1: PR 7203 `oneround/anchored-conic`, the hand work in `b303a32a6..a7fe5cf4c`

Head frozen at `a7fe5cf4cc134b139dcd4af38fbb052b1259d722`. Read:

- the three merges' resolutions (`git show --remerge-diff` of `e80763d8f`,
  `83d337786` and `a7fe5cf4c`);
- the three non-merge commits (`30c37f7cd`, `e3856409f`, `30276003`);
- main's new conic field reads since `ec8749552`, traced to how each was
  resolved.

I did not read main's own commits beyond that, and I did not read the
other reviewer's branch.

## Verdict: APPROVE-WITH-FIXES

The code merge is clean:

- No re-pointed site changes what it reads.
- No rebuild door loses a placement.
- No reader of the form appears outside `conic.rs`.
- No anchored circle comes back.
- Every claim M2 to M7 makes about the code holds.
- The local suite on the frozen head is green at ε 1e-9, with the one
  1e-12 red main's own.

The fix is to the **PR body**, which is what records this change. It was
not updated for the merge, and it states things that are now false (F1).
One doc measurement is also narrower than what the code does (F2).

## Executed, and against what

- **`ci` nextest profile, whole workspace, on `a7fe5cf4c`, ε 1e-9:
  17,651 of 17,651.** Judged against the implementer's 17,635 of 17,635
  on `30c37f7cd`, a head with 16 fewer rows (the second `origin/main`
  merge adds rows). The base's villarceau red from earlier heads is gone,
  since main has it fixed.
- **Same at ε 1e-12: 17,650 of 17,651.** Red:
  `sweep::all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`,
  `VolumeUncomputable`, margin `2.8154559647670245e-12`. That is the
  implementer's payload to the bit. It is held by
  `work/tesslie/main-red-a-cylinder-wall-through-a-prism-corner-union-is-volume-uncomputable-at-1e-12.md`,
  which is open on `origin/main` itself. I did not build main to
  reproduce it there.
- **Gates**, `scripts/run-gates.sh` on a clean worktree of the head with
  `GATE_BASE=e68799906` (`merge-base a7fe5cf4c origin/main` after a
  fetch): **exit 0, every gate green**. The two base reds of earlier heads
  (`engine-leaves`, `pin-cites-an-open-row`) are gone with main.
- **Hosted CI, run 45071025483**: when I last read it, "change filter"
  had succeeded (all its steps), "prime the build cache" was skipped, and
  every other job was **queued**: test (kernel), test (app), lint (fmt,
  clippy, gates), demos, corrupt input, viewer, python, step import,
  tools, interval backend, mesh budget. The interval oracle job was
  skipped. **It has not run.** I read no step logs beyond the filter.
- **My first-review dump probe**, ported to the head's API
  (`review-r1/merge/dump_probe_ported.rs`). It was ported once for the
  removed `anchor()`/`phase()` (read by datum name), and for `a7fe5cf4c`
  also through `Reach::fixture(at, 1.0)`, the same reach value the old
  constructor built. 48 far-centred cone sections, each minted, re-seamed
  a quarter turn, a half turn, and a quarter turn then reversed; 5,375
  lines.
  - At `b303a32a6` it is byte for byte my first-review dump of
    `8d5978300`.
  - At `a7fe5cf4c` it is not. **Every moved line is a quarter-turn
    variant's**: 30 of 48 and 30 of 48 carrier lines, 721 of 1,296 and
    724 of 1,296 point lines. The minted and half-turn variants are
    byte-identical.
  - In the moved carrier lines only `u_ref`'s three entries changed,
    never the anchor, axis, sizes or phase.
  - My exact-decimal oracle on the new dump: running bound 0 violations
    in 5,184 points, as before.
- **Mutants** on the head, against `geom`'s `conic::` and
  `curves::tests` rows (68). **All red**:

| mutant | red in |
|---|---|
| quarter turn rotates the pair the wrong way | `a_re_seam_moves_parameter_zero_and_never_the_anchor`, `a_re_seamed_ellipse_resolves_near_its_anchor_at_the_semi_axis`, `running_an_ellipse_back_is_its_own_door_and_not_a_frames_reversal` |
| half turn keeps the pair | `a_re_seam_moves_parameter_zero_and_never_the_anchor` |
| `same_content` ignores the form | `the_walk_names_an_arc_held_ellipses_anchor_and_phase` |
| `same_content` ignores the phase | the same |
| `data()` omits the phase | the same |
| `map_scalar` drops the phase | `a_re_seamed_arc_held_ellipse_reads_back_lifts_and_moves_as_itself` |
| azimuth ignores the phase's sine | the same |
| reversal keeps the sine | `a_re_seam_moves…`, `running_an_ellipse_back…` |
| `centred_at`'s arc-held arm sets the origin | `an_arc_held_ellipse_centred_elsewhere_is_translated_whole` |

- **A probe of the derived centre across a turn**
  (`review-r1/merge/centre_across_quarter_turn_probe.rs`): an arc-held
  ellipse, b = 0.37a, at a = 1e4, 1e5 and 1e6, on the rows' fixture frame
  and on 40 random frames each, seamed near 1.4 (a quarter turn) and
  near 3.0 (a half turn). Results under M2.

Only read: the remerge diffs of `conic.rs`, `curves.rs`, `exact.rs`,
`signed_zero.rs`, `offset_axial.rs`, `replace_face.rs`, `join.rs` and
the rest of the 22; `e3856409f` whole; the rows the range edits; the PR
body.

Not run: ε 1e-6; the slow set; clippy and rustdoc locally (the hosted
lint job had not run).

## The claims

**M1, holds (by inspection, likely).**

- Since `ec8749552`, main added 63 reads of a conic's frame as a field,
  in 28 files. I traced each production one to its resolution on the
  head. Every one that can see an ellipse reads `frame()`:
  `curve_pair/on_surface.rs` (`self.carrier.frame()?`), the step-import
  sites and `pcurve_cache.rs`. The rest are circle-only, where `frame()`
  is the stored field, or are not conic frames at all (editor-core's
  `self.frame`, `spiric_arc.rs`'s ovals, the product frame).
- On main every ellipse is held at its centre. The only ellipses held on
  the arc are `plane_cone_section`'s, and the only paths that reach them
  are import-free. So a re-pointed site reads on such an ellipse the
  derived centre, which is the only centre it has; on main the same site
  had only the stored centre. Same meaning, rounding apart.
- The one new site that looked like a rebuild,
  `replace_face.rs:2532`, reads a range off `frame()` and the semi-axes
  and builds nothing.
- `Circle::with_frame`'s caller now uses `placed(Frame::reversed)`,
  which `Circle::placed`'s doc now names as the run-back for a circle.

**M2, holds in code and docs; one figure is narrow (F2).**

- `quarter_turned_where` takes `quarter_turned_renormalized()` at
  `Inexact` (`conic.rs:957`). The module doc (`:66-71`) and the
  method doc (`:933`, "not re-squared") say so, and no sentence in
  `conic.rs` or the rows says the quarter turn re-squares.
  `one-stored-form…:101` says `half_turn_curve` re-squares, which is
  about `pole_form`, not the quarter turn.
- The **PR body** still does (F1).
- The two-sided pin and `a_re_seam_moves_parameter_zero_and_never_the_anchor`
  pass in the `ci` run. I did not read the pin's 0.66 `u·a` value out.
- My probe, the derived centre's move across a quarter turn:
  - **0 on the rows' fixture** at every a, as claimed (exact-thirds
    entries).
  - **0 across a half turn** on every frame, as claimed.
  - On 40 random frames a quarter turn moves it 0 to 3.7 `u·a` on most,
    with a tail of 4.9, 5.0, 5.5, 6.9, 7.6, 12.6 and 13.9 `u·a`. The
    largest come where the seam handed to `anchored` is near the axis
    (`|n̂ × t̂|` 0.11 to 0.22). Frames with `|n̂ × t̂|` above 0.6 still
    reach 4.9 `u·a`.
  - The doc's "0.5 to 2.6 `u·a` measured on frames in general position"
    is the implementer's four frames, and it understates that spread.
    See F2.
- Re-planted quarter-turn and half-turn mutants: red (table above).

**M3, holds (DEMONSTRATED, sure).** `same_content_by` passes `&same` to
`same_placement`, and the placement still takes part on every ellipse
path: the form-ignored and phase-ignored mutants are each red in
`the_walk_names_an_arc_held_ellipses_anchor_and_phase`. A circle
compares `frame()`, its stored field.

**M4, holds (by inspection, likely).**

- `truth` now reads the directions from the polar factor of the frame
  (`polar(&carrier_frame)`, `v` from it, main's), and keeps this unit's
  arm: the stored anchor and phase by datum name, with
  `A·cos θ₀ + B·sin θ₀` subtracted exactly.
- For an arc-held ellipse the oracle is therefore the curve through the
  stored anchor along the polar factor's directions. Main's reading of
  "the curve its stored numbers state" is that polar factor (the ONEHOME
  row `a-stored-frame-reads-as-two-curves…` names the two readings).
  That is main's decision, not this merge's.
- Whether the change of `v` moved any row's truth: every row that reads
  the oracle passes at ε 1e-9 on the head, the near-gap families' 40 of
  40 included. I did not diff truths.

**M5, holds (by inspection, sure).** `plus_zero_curve` folds through
`Circle::placed` / `Ellipse::mapped(|f| f.map_scalar(fold_signed_zero))`.
`mapped` keeps the `Held` and so the phase, unfolded. `plus_zero_frame`
is gone.

**M6, holds where I spot-checked (by inspection, likely).** Against
`origin/main` at `fd7e5f47b`:

- `circle_torus.rs` and `census/point.rs` differ by frame re-points
  only.
- `offset_axial.rs`, `replace_face.rs` and `join.rs` differ by
  re-points, by this unit's own pre-existing changes (`translate_curve`
  through `e.translated`; `join.rs`'s section loops through
  `conic.frame()`), and by pattern bindings renamed for the field's
  removal.
- `in_band_sweep.rs` differs by the unit's near-gap row. The fixture is
  now `Family { Circle, EllipseAtCentre, EllipseOnArc }` with no
  unreachable arm (`e3856409f`).
- The rebuild doors in `offset_axial.rs` (`:2218`/`:2251` region) and
  `replace_face.rs` are still circle-only: their ellipse arms return
  `None` or are bounds.
- Not checked: `pcurve_cache.rs`, `ring_path.rs`, `surface_group.rs`,
  `chart_iso.rs` and the step-import tests beyond their re-point lines;
  `cover-kind-match.counts` beyond the gate's green.

**M7, holds (DEMONSTRATED for the `centred_at` row, by inspection for
the rest, sure).**

- `an_arc_held_ellipse_centred_elsewhere_is_translated_whole` is red on
  the origin-setting mutant.
- The near-gap enum is as claimed.
- `carrier_resolution.rs` says "held at its centre".
- `Circle::placed`'s doc says what `Frame::reversed` is there and why an
  ellipse has its own door.
- `running_point`'s doc names every charge.
- The three paths on the readers row are corrected, and every path that
  row cites now exists.
- `the_running_residuals_are_the_plain_chain` has `Ellipse::anchored(…,
  1.1, 1.1)` again (`implicit.rs:3355-3362`).
- `Ellipse::mapped`'s doc says "an isometry of space that keeps its
  parameter".

**M8, holds (DEMONSTRATED).** The counts and the dump are above.

**M9, the rows say what the code does (by inspection, likely).**

- The unit row's new section "What `oneround/anchored-conic` (PR 7203)
  lands of it" states all of the following, and each is what
  `conic.rs` does:
  - the second placement is for ellipses only;
  - one construction holds the arc;
  - parameter zero is at the anchor only until a re-seam;
  - main's "The stored form" section says an arc-held conic has no
    re-seam, and that is not built here (`:205-212`).
- The `one-stored-form…` row (re-titled as the readers unit, id kept)
  and `a-section-conic-is-anchored-at-a-general-point-of-its-arc` read
  consistently with the code.
- One sentence in the owner's section is now stale on the head: "This
  unit (PR 7203) holds the two placements as a private tag beside an
  unread field" (`the-section-table…:185-186`). The unit's own section
  corrects it (`:197-198`), so the row is right taken whole.

**Fresh instances of what the fix pass closed (by inspection, likely):
none.**

- No new rebuild door loses a placement (M1, M6).
- Outside `conic.rs` the form is read only by datum name
  (`memo.rs:1129`, the exact oracle, `carrier_resolution`'s tests).
- No code, test or doc mints or describes a circle held on its arc.

## Findings

**F1, MINOR (by inspection, sure). The PR body was not updated for the
merge, and it is the change's record.** As of the frozen head it says:

- "at `f64` a quarter turn re-squares the frame and the centre read
  through it moves about 1.2 `u·a` (review r1)". This is false since
  `e80763d8f` (M2), and the brief records the implementer as claiming
  that no sentence in the PR body still says it.
- "Hosted CI cannot run. This head conflicts with `main`…" Hosted CI
  run 45071025483 exists for this head.
- "Stacked on `onehome/carrier-frames`…" PR 7131 is merged.
- The verification table is "on `08473553a`" with base counts from
  `ec8749552`, and lists villarceau and two gates as reds. None of those
  are red on this head.
- The opening section quotes PR 7194's sentence as the ratified text.
  PR 7258's sentence has replaced it on main.

**F2, NOTE (DEMONSTRATED, likely). The quarter turn's centre move is
quoted from four frames.** `conic.rs:70-71` says "0.5 to 2.6 `u·a`
measured on frames in general position". On 40 random frames per size
I measure 0 to 3.7 `u·a` typically, 4.9 `u·a` on well-conditioned
frames, and up to 13.9 `u·a` where the seam handed in is near the axis.
The sentence's kind of claim ("a rounding of the semi-axis") holds. Its
range does not, and nothing reads the range.

## Style (on the hand work)

Questions exercised: Q1, Q3, Q4, Q5 and Q6.

- **S1 (Q4, sure).** F1 is Q4's shape. The merge changed a premise the
  PR body cites (the re-squared quarter turn) and the sentence stayed.
  The code side was swept; the prose side the PR presents was not.
- **S2 (Q6, likely).** F2 is a measured figure at a claim site with no
  guard and no register, and it is narrower than the behaviour.
- **S3 (Q3, sure).** The new `centred_at` row can fail: it went red on
  the mutant aimed at it.

Ratings on the hand work:

- idiom/structure 4;
- test quality 4;
- doc/comment honesty 3 (the code's docs are right, and the PR body is
  stale on five counts).
