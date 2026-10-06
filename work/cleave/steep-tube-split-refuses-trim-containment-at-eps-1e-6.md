---
id: steep-tube-split-refuses-trim-containment-at-eps-1e-6
kind: issue
title: review_cleave_wrongarc's steep tube cut refuses its split Pcurves TrimContainment at CAD_TOLERANCE_EPS=1e-6, so the row is red on main there
status: closed
opened: 2026-10-02
priority: P1
cost: M
pr: 3981
branch: cleave/steep-tube-eps
closed: 2026-10-03
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

## Owed

Measure what the −3.06e-6 is. It is the split pcurve's trim-containment
margin at the first sample (`HalfEdgeKey(24v1)` at `flip false`,
`18v1` at `flip true`). Decide whether it is a real overshoot of the
trimmed window whose size tracks ε, or a certifier margin that does not
scale. Then fix the code, or state the row's ε premise.

## Measured (CLEAVE `cleave/steep-tube-eps`, 2026-10-03)

The −3.0645639e-6 does not track ε. The face window's
`v_min = −1.0000030645639348` is bit-identical at ε = 1e-9, 1e-8, 1e-7,
1e-6 and 1e-5. It is not an overshoot either: it is an inside
clearance. The window is the hull of the face's stored rows, and the
cut row's `Harmonic` box (`harmonic_span_box`) dips its conservative
`amp·Δt²/8` past the rim (amp 1.0288606, Δt²/8 = 2.9786e-6, product
3.0645639e-6). The rim half-edge's exact box sits that far inside it.
Check 5 decided the escape two-sidedly, so a negative clearance in
`(−K·ε, −ε)` escalated, although both readings it lies between (Zero,
Negative) pass. Fixed at the certifier: `trim_containment` decides
only an escape's positive part, as the iso rows' `pcurve_iso_domain`
gates do. The row now passes at the default, 1e-9, 1e-6 and 1e-12.
