---
id: steep-tube-split-refuses-trim-containment-at-eps-1e-6
kind: issue
title: review_cleave_wrongarc's steep tube cut refuses its split Pcurves TrimContainment at CAD_TOLERANCE_EPS=1e-6, so the row is red on main there
status: open
opened: 2026-10-02
priority: P1
cost: M
---


Found by REACH's `reach/door-backstop` lane while running its three-ε
battery. It reproduces on a clean `origin/main` at `3ee0e4b6`:

    CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep \
      -E 'test(steep_cuts_of_tubes_chord_inside_each_bore_face)'

`crates/sweep/tests/review_cleave_wrongarc.rs`
`steep_cuts_of_tubes_chord_inside_each_bore_face` fails with
`p2 tube: 0 wrong, 2 refused`. Two poses refuse:

    tube a 0.4 d 0 turn 1.0682 tilt 1.2 at [0, 0, 0.5] phi 0 flip false|true:
    split refused: Pcurves(Certify { error: Escalated { check: TrimContainment,
    sample: 0, cause: Indeterminate { margin: -3.06e-6, … } } })

The split's pcurve certificate reads trim containment in band at this ε
(margin −3.06e-6 against ε = 1e-6). The row passes at 1e-9. The
per-PR gate runs the 1e-6 row only for eps-sensitive crates the diff
touches, so this reds the nightly, not every PR.

## Also measured by PR 3805's lane (2026-10-02)

On `3ee0e4b6e`, in a clean worktree: `TALLY p2 tube: ok 46, wrong 0,
refused 2`. Both refusals are one pose in its two orientations: tube
`a 0.4 d 0 turn 1.0682 tilt 1.2 at [0, 0, 0.5] phi 0`, `flip` false
and true. Both read `Escalated { check: TrimContainment, sample: 0 }`
with margin −3.0645639348e-6 in the 1e-6 band's gap. No wrong answer.
A fix decides between two options:
- the trim reading should resolve at 1e-6 (a lever question for
  `pcurve_trim_containment`);
- the row accepts a typed refusal at the coarse band for this pose.
