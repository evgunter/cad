# The elbow's probe hacks (HELDRECORDS, `heldrecords/elbow`)

`probe-hacks.patch` is the env-gated instrumentation the elbow lane ran
against main's code before the junction ruling, to see what stood
behind each wall. It is a record, not a change: nothing applies it.

- `ELBOW_RIM=1`: skip the glue door's tangency table on a rim (the
  margin is the rim's identity), and read a transverse rim as unglued.
- `ELBOW_RIM2=1`: in `reduce::curved_face_arm`'s one-end-on-the-carrier
  arm, read an unsettled or in-band `wall_crossing` as no interior
  crossing — the crossing layer's wall, passed blind.

Apply on the merge base and run
`cargo nextest run -p editor-core --test all --run-ignored only zz_probe_outcomes --no-capture`
with the flags set. What each run showed is in
`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`.

## The junction's mutants (PR 7432's fix pass)

`mutate.py crates/topo/src/boolean/mod.rs` rewrites `junction_of` so
`ELBOW_MUT` picks one of its reads to break (the script's docstring
names each, with the reviewers' names); unset, the code is the head's.
Apply, build once, run the rim rows under each value, restore:

    ELBOW_MUT=<m> cargo nextest run -p sweep -p editor-core --test all \
      -E 'test(/a_body_on_a_rods_cap/) | test(/a_rod_unions_with_what_ends_on_its_rim/) | test(/a_pipe_unions_with_its_elbow::/) | test(/mate7a_torus_rest::the_g1/)'

What each turned red (2026-11-23, on the fix pass's head); every row
named is a door row unless marked:

| mutant | rows red |
|---|---|
| none | none |
| `T_GLUE` (R1 `T_GLUE`, R2 `transverse_glues`) | frustum at a corner; half ball, quarter ball (their 90° pairs glue); bowl; cusp; all four elbow rows |
| `SAME_GLUE` (R1) | bowl; the elbow bent back into its pipe |
| `NO_ROUTE` (R1) | frustum; half ball; quarter ball; cusp; bowl; all four elbow rows |
| `SEAM_UNROUTED` (R1, R2 `seam_never`) | half ball; quarter ball; the rod's, the pipe's and the turned elbows' junctions; the tube chain's |
| `CUSP_GLUES` (R2) | cusp; bowl; the elbow bent back into its pipe |
| `ANY_DEPARTURE` (R2 `seam_any_departure`) | bowl; the elbow bent back into its pipe |
| `NO_CONVEX` | the flanged rod's, alone |
