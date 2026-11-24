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
