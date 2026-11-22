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
