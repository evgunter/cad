---
id: a-lane-choice-is-logged-as-a-choice-so-a-turned-one-refuses-a-leaf-the-general-lane-serves
kind: issue
title: A closed-form lane choice is logged as a choice, so a turned one refuses a leaf the general lane serves
status: open
opened: 2026-11-24
priority: P2
cost: M
---


Found by the three designers of
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
(2026-11-24), by reading; no decided leaf in M13's corpus shows it.

`geom_brep::intersect`'s `closed_form_shown` asks whether a section may
take its closed-form lane (for example `pn_axis_normal`, the
circle-against-ellipse frame of plane×cone) through `decide`, in the
caller's role, which defaults to `Role::Choice`. The answer picks an
instrument, not a shape: where it is not `Zero` the general lane builds
the same curve. So:

- a tilt that is `Zero` at the nominal and definitely nonzero over a
  leaf turns a logged choice, and under any certificate that compares
  choices that leaf is a flip, though the part's form is the nominal's;
- `closed_form_shown` swallows an in-band `Err` (`ladder::in_band`) and
  takes the general lane, but `decide` has already put the escalation
  on the node's log, and `drive::classify_replay` reads that log: the
  leaf may bisect or refuse as a sliver on a decision whose undecided
  answer was served (not checked by running; worth a row that drives
  an in-band tilt over a box).

DESIGN `[verdicts.consumers]` says a lane choice records nothing. First
step: list the sites that read through `closed_form_shown` and the
role each logs in; then either an unlogged door with its ledger row, or
the check role, per site. Ground: `crates/geom-brep/src/intersect.rs`
(GERM, SECT and TANG claim it; the finding is the box drive's).
(BOXFINISH orchestrator)

## Measured instances (M13's addendum and M13b, 2026-11-24)

The class is wider than `closed_form_shown`, and it is measured: **318
of the 660 named flips on the suite's drives turn a logged choice and
move only coordinates the leaf's enclosures already carry** (same
cells, names, incidence, face senses, carrier kinds, standings; no
finished-body check turns).

- `segment_gap` (300 leaves; the `notch` drives): "whether two
  straight segments lie apart along one of them" (`profile`'s gate), a
  step of the simplicity proof's route. The moving vertex is on the
  same side at the witness and over the leaf.
- `carrier_line_circle` (18; the band-parameter plate): a larger hole,
  the same cells.
- `pcurve_chart_placement` turning between the two lanes at one point
  (2 builds, the tour's bracket) is inferred to be the same: two
  readings of an azimuth across a chart's cut that the turn undoes.
  Its margins are not measured.

Not in the class, though first reported as such:
`extrusion_normal_component` (122 leaves). The leaf's depth is
negative and the evaluator places the body the other way through its
frame, so the part is on the other side of its sketch plane.

**The reading each site is owed:** what does the decision's other sign
build? If another part, it is a choice and its flip is honest. If the
same part by another route, the decision is re-posed, or made off the
log under a ledger row, or given a role the leaf comparison does not
read (the last would change E6 and go to Ev). Until a site is read its
leaves stay named flips: refused mass, never a wrong certificate. The
three designers of the parted-path fork agree on this and say it does
not gate that fork. (BOXFINISH orchestrator)

