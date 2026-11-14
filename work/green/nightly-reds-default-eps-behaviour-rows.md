---
id: nightly-reds-default-eps-behaviour-rows
kind: unit
title: Nightly reds: the behaviour rows red at the default ε, and the viewer's door-at-rest pair
status: review
opened: 2026-11-14
priority: P1
cost: M
branch: green/default-eps
pr: 6931
---

From nightly 43371638371 (main `e36675c1`, 2026-11-13), each leg's junit read whole. GREEN's rule holds (`work/green/program.md`): a red is greened here, not fixed. A move a merge explains is re-blessed, naming the merge; anything else is pinned at what main produces, or `#[ignore]`d naming the row it waits on, and the fix row goes on the program that owns the code.

**Difficulty letter:** M. **Review tier:** single review, STYLE — A pin can be read against what main produces. Both logged before any draw (2026-11-14).

## What is red

- `sweep::all a_plane_near_a_needle_cones_apex::a_needle_cone_cut_off_its_apex_lands_its_closed_forms` (all rows; α = 1°, tilt 1.5: `CensusUnsupported … Unsettled(Meter "curve_pair_pose_basin")`)
- `sweep::all an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out` (default only; `Unmet { stop: Floor }`)
- `topo::all shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void` (default only; -1.1025e-16 against -1.0142e-16)
- `topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it` (all rows; `arc_root_just_outside_its_span`, `roots_a_hair_apart`)
- `viewer::all door_at_rest::the_threaded_door_seam_answers_the_newest_landing` and `::a_landings_door_reads_pending_until_its_own_answer_settles` (all rows, and the `viewer app feature` job, which stops at them with 784 tests not run)

This slate's rows: `main-red-the-aimed-ray-seam-split-census-misses-its-settle-floor`, `main-red-two-offered-tolerances-fail-just-below-their-margin`, `main-red-two-offer-rows-quote-a-positive-margin-against-a-negative-case` (the last two name one test). Elsewhere (unchecked): `work/splitdoor/tier-3-prime-cannot-settle-a-ruling-against-an-e…`, `work/germ/a-sector-direction-read-off-a-needles-short-run-…`, `work/coincide/no-row-cancels-the-door-between-replay-and-disch…`.

## What the lane found (`green/default-eps`, 2026-11-14)

All six reproduce on main `b363f5fa` at the default ε. Corrections to the
list above, each checked against the tree:

- **The needle cone** is red at the default ε and at 1e-6 and 1e-12 as
  listed. Re-blessed: PR 6648 (`onecopy/carrier-standing-2`, merge
  `af002229b`) typed the census's meeting refusal, and the pin was
  spelled on the string it replaced. The splitdoor row named above is
  the residue's row and now says so; the residue did not move.
- **The seam-split census** is red at the default ε only. Pinned: four
  poses stop their measure at the floor since PR 6478
  (`spiricarm/rotational-lane-cylinder`, merge `898df16c9`, bisected).
  Fix row
  `work/spiricarm/a-seam-split-slivers-measure-stops-at-its-floor-2600-volumes-wide-since-the-rotational-cylinder-lane.md`.
- **The needle cavity** is red at the default ε only (the test stands
  down at 1e-12). Pinned: four runs of the moved 10.5ε channel read their
  shell volume 8.0% to 8.6% short since PR 6505
  (`onedoor/one-surface-record`, merge `9e1484043`, bisected). The germ
  row named above is not its row: that one is the turned 12ε corner
  refusal the test already pins (`Want::Corner`). Evidence added to
  `work/onedoor/the-planar-face-reading-is-two-formulas-at-two-scalars.md`.
- **The offer rows** are red at every row (the children run at the
  design ε). Pinned: the root lane quotes chord lengths since PR 6687
  (`onecopy/needle-basin`, merge `f875961fe`). Fix row
  `work/onecopy/the-conic-root-lanes-chord-rungs-quote-a-length-where-two-offer-cases-declare-a-negative-margin.md`.
- **The viewer's door-at-rest pair** is red at the default ε only as run
  here (green at 1e-6 and 1e-12 once the count is right; the nightly's
  "all rows" was not re-run before the change). Re-blessed: the composed
  die's second blend joint proves since PR 6710
  (`onecopy/edge-ends-turns`, merge `75a84e844`), which
  `editor-core`'s `the_door_proves_half_the_composed_dies_joints` was
  re-blessed for in PR 6771; the viewer's copy of the count (3: one
  unproven joint and two vertex identities) was left behind, and is now
  2. The coincide row named above (no row cancels the door between
  replay and discharge) is about something else and is untouched.
