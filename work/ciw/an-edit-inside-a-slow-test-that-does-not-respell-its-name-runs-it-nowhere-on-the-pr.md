---
id: an-edit-inside-a-slow-test-that-does-not-respell-its-name-runs-it-nowhere-on-the-pr
kind: issue
title: an edit inside a slow test, or to its helpers, that does not re-spell its name runs it nowhere on the PR
status: open
opened: 2026-10-28
priority: P3
cost: E
---


Filed by TESS's thin-rings lane (PR 5911), outside its fence.

## What

`docs/prompts/implementer-discipline.md` §2 says the per-PR gate runs
"the slow tests your diff adds or edits". `.github/workflows/ci.yml`'s
slow-set step runs a slow test only when an added Rust line in the diff
spells its leaf name (`scripts/check-fast-set-times.py` `added_tokens`,
then `scripts/slow_set.py --new-expr`). An edit to a slow test's body,
or to a helper it calls, that leaves its `fn` line untouched therefore
runs the test in no PR step. The ε rows that run "every test file it
adds or edits" (`EPS_EXTRA`) also run under the `ci` profile, so they
skip it as well. The script's module doc names the general blind spot
("a test whose name no added line spells"), but §2 and its readers
assume an edit is caught.

## Witness

PR 5911 head `e779226321`
(`crates/sweep/tests/an_annular_tube_through_a_plate.rs`) changes the body
of the slow test `thin_nested_rings_build` and the `every_op` helper it
calls. The run's change filter selected `sweep`, and the slow-set step
logged "0 tests run". The two ε rows ran the file's four fast tests only.
The changed test was green only by a local run.

## Wanted

Either make the gate select the slow tests of every test file the diff
edits (the file-level rule `EPS_EXTRA` already uses), or reword §2 to
say a slow test runs on the PR only when an added line names it. A §2
edit goes to Ev, because `docs/prompts/` binds every lane.

## Evidence, 2026-10-31 (PR 6128)

A second witness. PR 6128 (`vseam/a-labelled-creation-is-one-recording`),
at head `3f7b8d48ed`, rewrote `labelled_as_one_undo` in
`crates/viewer/tests/node_labels.rs`. That helper is the body of the
slow test `every_creation_labelled_is_one_undo_whatever_door_commits_it`,
which the same PR had just moved into the slow set. No added line spelled
the test's name, so the PR's slow-set step selected nothing, and the
PR's central row went green on no gate at all. The review caught it. The
fix pass re-spells the name in the helper's doc comment.

A third (GREEN lane, PR 6343, 2026-11-02). The PR re-pinned eleven
sweep slow rows by editing their expectation tables
(`cylinder_sphere_frame`'s `pose_rows!` door arrays, `germ_tilted_rod`'s
`OTHER_SPINS_STOPS`, and the asserted `Stop` in `torus_doors_join`). Its
first gate ran `Starting 0 tests` in the slow-set step and still
concluded `gate ok`. The fix pass names each row in its pin comment.

## Evidence: PR 6818 (TINT, 2026-11-11)

Two more slow rows show the same blind spot, from both sides.

- **How the red got in.** PR 6371 retyped `BooleanError::ResultInvalid`'s
  field from `errors` to `findings`. Nothing in that diff spells
  `germ_tilted_rod::the_steeper_tilts_union_stops_by_identity`, so the
  test ran on no PR step. It stood red on main from 2026-11-08 until
  PR 6818.
- **How the fix goes unchecked.** PR 6818 re-pins that test inside its
  body and adds a line to
  `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`'s
  `ROSTER`. Its head `403e5baf1f` reported, in hosted run 42965866425 (job
  `test (kernel)`):
  - `slow set over: package(geom-core)|package(sweep)`, followed by `0 tests run`;
  - change filter: "1 of its rows are in ci's slow set and skip this step"
    for `bounds_census.rs`, and "3 of its rows" for `germ_tilted_rod.rs`.

  So neither fix ran hosted at any ε. The PR's evidence for both is a
  local run.

- **PR 6934** (`green/eps-1e-6`, 2026-11-14): two of its thirteen slow rows were edited without their names on an added line (`a_vertex_piercing_a_lens_wall::a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate`, a re-pinned stop inside the body; `m5_s11_concave_sense::loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below`, a constant it reads). Each now has a doc line naming the test, which is what selects it. Even selected, a slow row runs on the PR at the default ε only, so an ε 1e-6 pin on one is checked by no PR step.


## Evidence (GREEN, 2026-11-14): a PR that re-pins six slow tests got a green gate that ran none of them

PR 6931 (`green/default-eps`) pins or re-blesses six slow-set tests by editing their bodies and helpers. Its first gate run (head `447474cb7`) was green with the slow-set step lasting one second: the diff's added lines spelled none of the six names. The lane made each pin's comment name its test, and the second run (43555174396) is the one that ran them. Every lane that greens a nightly red is in this position, since greening a slow test is an edit inside it. On GREEN this turns "the gate is green" into no evidence at all for a pin PR, so the lanes now read the step log for which tests ran.

**Comments written for the selector (PR 6931's fix pass).** The lane's
answer was to make a comment beside each pin name the test that reads
it. Those mentions exist so that an added line spells the name and the
slow-set step selects the test; they are to go when the selector reads
edits inside a test. They are:

- `crates/sweep/tests/a_plane_near_a_needle_cones_apex.rs`, the module
  header's last sentence;
- `crates/sweep/tests/an_aimed_ray_from_far_reads_a_split_side_right.rs`,
  `Read`'s doc (two test names);
- `crates/topo/tests/shell_role_reader.rs`, `MOVED_THIN_READS`' doc;
- `crates/topo/src/boolean/offer_rows.rs`, `QUOTED_AS_A_LENGTH`'s doc;
- `crates/viewer/tests/door_at_rest.rs`, `DIE_UNPROVEN`'s doc.

The review of that PR also read which rows the second run reached: each
of the six once, at the default ε, in the slow-set step, and nowhere
else (`work/ciw/an-extra-eps-step-that-selects-no-test-reads-green.md`).
