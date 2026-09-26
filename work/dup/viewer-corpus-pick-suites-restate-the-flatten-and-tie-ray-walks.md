---
id: viewer-corpus-pick-suites-restate-the-flatten-and-tie-ray-walks
kind: issue
title: flatten, tie_rays_for and the answers walk are restated across four corpus pick suites
status: closed
opened: 2026-09-20
priority: P4
cost: D
closed: 2026-09-26
---


## Finding

- **Where**: the four corpus pick suites in `crates/viewer/tests/` —
  `index_memo.rs`, `pick3_acceptance.rs`, `review_pick2_r1.rs`,
  `review_pick_r2.rs`.
  - `fn flatten(index: &PickIndex) -> Vec<FlatPart>` in
    `pick3_acceptance`, `review_pick2_r1` and `review_pick_r2`.
  - `fn tie_rays_for(index: &PickIndex)` in `index_memo` (answering
    `Vec<Ray>`), `pick3_acceptance` and `review_pick2_r1` (both
    answering `Vec<(Ray, f64)>`), plus `index_memo`'s `rays_for`,
    which is the same axis-and-diagonal enumeration without the tie.
  - the per-ray answer walk, `service_answers` in `index_memo` and
    `answers` in `pick3_acceptance`.
- **What makes the fold a judgement, not a substitution**: two of the
  suites are promoted review suites for the pick door, and the walks
  they restate are the SECOND implementation their rows compare the
  door against. `tie_rays_for`'s two return types are also a real
  difference (`Vec<Ray>` against `Vec<(Ray, f64)>`), so the reconcile
  comes before the move. The surviving per-row test applies: name the
  helper each row would otherwise read, and say whether a bug in it
  would be invisible.
- **Importance**: medium. Unlike the door classes closed beside it,
  this one carries an oracle — these ARE the hand-parallel walks — so a
  wrong fold here makes a suite compare the door against itself.
- **Instrument, and its blind spot**: `git grep -n 'fn flatten(\|fn
  tie_rays_for\|fn rays_for\|fn answers(\|fn service_answers('` over
  every tracked file, no path argument, then a read of each body. It is
  name-shaped and therefore structurally blind to a fifth copy under a
  new name; a whole-function scan over the four files is the second
  instrument this owes before a count is published.
- **Raised by**: the S-DUP lane closing the four viewer-suite door rows,
  2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.


## Closed

Folded by the S-DUP lane `dup/b6-b`, 2026-09-26, cut from
`032999ff2`. PR: batch 6.

- **Census re-taken at the merge base.** The row's name instrument
  (every tracked file, no path argument) returned the row's members:
  `flatten` ×3, `tie_rays_for` ×3, `rays_for` ×1, `answers` /
  `service_answers` ×2. The second instrument, structural needles over
  every tracked `.rs` — `Bvh::build(&boxes)` outside `crates/*/src`,
  `boundaries.iter().step_by`, `Ambiguous { hits }) => hits`, the
  six-axis literal, `value * 1.03125` — found what a name cannot:
  - a FOURTH flatten, `index_memo`'s `FlatReference::of`, and the
    exhaustive walk spelled twice (`index_memo`'s `FlatReference::pick`
    against `pick3_acceptance`'s `every` + `winners`, which its own
    header calls "this same exhaustive walk");
  - the landing driver four times (`pick3_acceptance`'s
    `over_every_landing`, three hand-written loops in the two review
    suites), with `bump_op` ×3 and `ring_bump` ×3 beside
    `index_memo`'s `first_length_slot`, which `review_pick_r2` declared
    itself a restatement of;
  - the wide aim's vertex × axis × reach loop ×3, the six-axis array
    ×7 in these files;
  - the winner's barycentric bound read ×3 — and `pick3_acceptance`'s
    copy was NaN-blind (`fold(0.0, f64::max)` drops a NaN), where the
    other two red on one;
  - `det_and_conditioning`, byte-identical in `review_pick_r2` and
    `editor-core`'s `review_pick_r2_probes`.
  Not members: `rays_for` (one copy; its axis rays now go through the
  shared `AXES`/`aimed`), `index_memo`'s `hits` and
  `assert_flat_reference` renderings (they render a refusal as text
  rather than panic — folded into one local `rendered` instead), and
  the two review suites' own candidate walks (each measures two
  acceptances over one candidate set; that walk IS the suite — and
  `review_pick2_r1` spelled it twice, now one local `nearest_both`).
- **The homes.** `crates/viewer/tests/common/corpus_pick.rs`:
  `FlatReference` (with `FlatPart`, `FlatHit`, `every`, `pick`),
  `too_wide`, `extent`, `tie_rays_for`, `wide_aim` (+ `Aim`),
  `ring_bump`, `set_slot`, `over_every_landing`. `set_slot` is the one
  spelling of a slot write as a `SessionOp`; `index_memo`'s `Edit` now
  carries the `Expr` and delegates to it. The rays and the
  door-answer list go one level lower, to `editor_core::test_support`
  (`AXES`, `aimed`, `listed`, `det_and_conditioning`), behind
  `editor-core`'s `test-support` feature, which `viewer` enables
  through a dev-dependency.
- **The reconcile before the move**: `tie_rays_for` answers
  `Vec<(Ray, f64)>`; `index_memo` drops the reach at its one call.
- **The per-row oracle test the row asked for**, suite by suite:
  - `index_memo`'s three differential rows and `pick3_acceptance`
    read `FlatReference` as the second implementation. The door
    (`PickIndex::pick`) reads none of it, so neither compares the door
    with itself; a defect in it makes the two disagree and reds them
    (plant V1).
  - `review_pick_r2` reads the reference's trees, the wide aim, the
    landings and `too_wide`; its `PINNED` equality sees any move in
    any of them (V1, V4, V5b, V6, V8).
  - `review_pick2_r1` compares two acceptances over ONE candidate set
    per ray, so a defect in the shared walk moves both sides equally
    and is invisible to it by construction — true of its private copies
    before the fold as well. A `panic!` control (V2) proves it reaches
    the home.
  - The aims decide which rays are fired, not what any ray answers;
    only `pick3_acceptance`'s ray-count floors see them (V3).
- **Behaviour**: every tally the four suites print under
  `--nocapture` was diffed line-sorted against the merge base's and is
  identical (321 lines), including `review_pick_r2`'s `PINNED`
  `(442782, 141983, 12786, 6882)`. The one deliberate change is the
  NaN arm `pick3_acceptance`'s bound read now has.
