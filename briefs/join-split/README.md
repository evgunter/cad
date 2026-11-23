# Probes from the design weighing of the shared-side split

Throwaway instruments written while the question was weighed, against `tesslane/ruled-density` at `a020a3edca`. They are evidence and a starting point, not code to keep.

- `chord-midpoint-probe.diff`: environment switches (`NO_LANE_RULE`, `CHORD_FLOOR`, `FOLD_INSERT`, `JOIN_PROBE=cure`) that turn the three present shared-side rules off and split an overused side at its straight midpoint in all but one patch, where the patches join.
- `surface-point-probe.diff`: the same with `JOIN_PROBE=surf`, which splits every patch that has the side inside at the surface foot of the side's 3-D midpoint (a stand-in for the midpoint in the face's chart), and a probe row printing signed volume per cell.
- `volumes-*.txt`: that row's output on the bored balls of `split_rulings` under each: today, the chord midpoint, the surface point.
