# GUI-2 review R1 — mutation probe record

Frozen head `609c22d4` (PR #1106). Each mutant was applied to
`crates/viewer/src/pick.rs`, the four rows below were run, and the
worktree was restored. Purpose: claim 4 of the review charter — show
the two headless agreement rows CAN go red, and map what they are
blind to.

Rows run per mutant:

- `select_pick::the_id_passs_transform_samples_the_pixel_the_ray_was_cast_through` (shipped)
- `select_pick::the_ray_paths_answer_is_the_id_maps_inverse` (shipped)
- `select_pick::every_id_round_trips_to_the_patch_it_names` (shipped)
- `review_gui2_r1::cursor_projection_is_exactly_a_shift_and_scale_in_ndc` (this review's)

| mutant | shipped transform row | shipped inverse row | shipped round-trip | R1 shift/scale row |
| --- | --- | --- | --- | --- |
| 1. `cursor_projection` drops the `− cx·w` translation | **RED** | green | green | **RED** |
| 2. `cursor_projection` halves the x scale (`sx·0.5`) | green (BLIND) | green | green | **RED** |
| 3. `IdMap::key_of` off by one (`entries[id]`) | green | green (see note) | **RED** | green |
| 4. `by_name` records `index` instead of `index + 1` | green | **RED** | green | green |

Notes:

- Mutant 2 is the measured blind spot: the shipped transform row derives
  its cursor from the same projection it checks, so the residual it
  bounds is sub-pixel BEFORE scaling and any scale error (either
  direction) stays inside the `< 1.0` box. This matches issue #1097 §4's
  own caveat ("a sign/scale error … only hardware shows") — but the
  scale half is in fact checkable headlessly, and the promoted R1 row
  `cursor_projection_is_exactly_a_shift_and_scale_in_ndc` now pins it.
- Mutant 3 note: the shipped inverse row stayed green because on the
  single-body plate a wrong PATCH still has the right (node, body), and
  `name_of` is an independent (unmutated) lookup. The round-trip row is
  what owns that mutant; jointly the suite catches it.

Script: applied with `python3` string replacements inside
`with-build-slot.sh`-wrapped `cargo test` runs; full log in the review
lane's scratchpad (one-shot comparison artefact, not committed — this
table is the record).
