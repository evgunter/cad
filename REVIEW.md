IN PROGRESS

# Review of PR #4061 (frozen head 2c3f7072)

Interim (container may restart):
- Inspection: walk_order, precedes, b_runs and A's run value are identical to main; held_cut's fan
  branch is equivalent because a fan always has lo.0 != hi.0 (run_fan is empty for from == to).
- Claim 5: mutants tieflip / noorigin (walks_before) and runflip / runfwd / two_fwd (walk_run)
  all go red on the committed unit rows.
- Batteries main vs head: running.
