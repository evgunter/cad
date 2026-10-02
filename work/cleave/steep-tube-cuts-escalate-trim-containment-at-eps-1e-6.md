---
id: steep-tube-cuts-escalate-trim-containment-at-eps-1e-6
kind: issue
title: review_cleave_wrongarc's steep tube cuts escalate pcurve_trim_containment at ε 1e-6, red on main
status: open
opened: 2026-10-02
---


## What

`sweep`'s `review_cleave_wrongarc::steep_cuts_of_tubes_chord_inside_each_bore_face`
(`crates/sweep/tests/review_cleave_wrongarc.rs`, landed with #3718) is
red at `CAD_TOLERANCE_EPS=1e-6` on `origin/main` (`3ee0e4b6e`, measured
2026-10-02 in a clean worktree) and passes at 1e-9. The tally:

```
TALLY p2 tube: ok 46, skipped 0, wrong 0, refused 2, volume refused 0
  REFUSED tube a 0.4 d 0 turn 1.0682 tilt 1.2 at [0.0, 0.0, 0.5] phi 0 flip false: split refused:
    Pcurves(Certify { half_edge: HalfEdgeKey(24v1), error: Escalated { check: TrimContainment, sample: 0,
    cause: Indeterminate { margin: MarginDiag(Value(-3.0645639348403364e-6)), band: Band { zero: 1e-6,
    escalate: 1e-5 }, predicate: Some("pcurve_trim_containment") } } })
  (the same at flip true, margin -3.0645639349721754e-6)
```

So nothing is wrong, but the steepest cut (tilt 1.2) refuses: the
pcurve certifier's trim-containment margin at the split's first sample,
−3.06e-6, lands in the 1e-6 row's ambiguity band. The row requires
`refused.is_empty()` at every ε. The per-PR gate runs the extra ε rows
only for the crates it seeds, and the nightly runs them all, so this is
a nightly red.

## Owed

Measure what the −3.06e-6 is: a real overshoot of the trimmed window by
the split's pcurve, whose size tracks ε, or a certifier margin that
does not scale. Then fix the code, or state the row's ε premise.
