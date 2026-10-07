IN PROGRESS

# Review of PR 4207 (frozen head 04bdeebe4, base 3e9d1a96)

Interim notes (inspection done, executed batteries running):
- Row `a_pinchs_cones_share_one_point_key` passes on head (executed).
- `Body::share_point` (body.rs:702) reads no coordinate and checks none: a
  rebind onto a key whose stored point differs is not refused.
- `zip.rs` now carries two hand-rolled union-finds (258, 411); a third in
  merge_faces.rs:1414.
- Row doc (join_pierce_runs_sweep.rs:1827) restates the cause the PR body
  corrected ("the other's from the cube's pierce copies").
