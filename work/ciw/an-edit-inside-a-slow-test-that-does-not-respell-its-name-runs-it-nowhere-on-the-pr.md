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

