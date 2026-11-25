---
id: a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine
kind: issue
title: The injectivity engine opens no window on a spindle sheet's face or an approximated face, so every finished body holding one refuses Unsupported at the at-rest gate
status: open
opened: 2026-11-21
priority: P0
cost: H
---


## Finding

From EMBED's per-solid census trace (lane `embed/trace-cone-torus`,
2026-11-21), on main `c2a0c41f40` with the at-rest gate
(`embed/gate-measure`).

`topo::clearance`'s window builder (the `match &charted` over the
lifted carrier, `crates/topo/src/clearance.rs`) returns
`unsupported("a spindle sheet's face")` for `Surface::Spindle` and
`unsupported(APPROXIMATED)` for `Surface::Approx`, twice each (the
stored and the lifted match). `injective::by_kind` says the same of a
spindle sheet ("has no injectivity lane yet"). No cell of such a face
is ever asked.

Both kinds are first-class carriers a finished body holds:
- `docs/DESIGN.md` `[surfaces]`: "Plane / cylinder / cone / sphere /
  ring torus / spindle sheet are first-class variants", and `Approx`
  is "the certified approximating class (OFFSET-DESIGN O2)".
- `crates/topo/README.md` (the full revolve): an arc whose circle's
  centre is off the axis by no more than the minor radius picks "one
  sheet of a spindle or horn torus, whose points on the axis are apex
  vertices in the pole form".

No clause says a finished body may not hold either, so each refusal is
a valid body refused at its door: P0.

## Count and reproducers

- **Spindle, 20 tests**, all f64. Cheapest:
  `sweep::all the_football_revolves::a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip`
  (0.09 s): `Refused(Unsupported { carrier: "a spindle sheet's face", face: FaceKey(3v1), side: First })`.
- **Approx, 7 tests**, all f64, all `sweep::all`. Cheapest:
  `a_moved_body_keeps_its_verdict::an_offset_cap_keeps_its_verdict_under_a_move`
  (0.14 s): `Refused(Unsupported { carrier: "an approximated face", face: FaceKey(1v1), side: First })`.

Checked: both reproducers refuse at the window with exactly that text,
which no other site raises.

## What is needed

Two units, not one:
- **The spindle sheet:** a window on its chart (the torus's two turns,
  with the apex vertices on the axis as collapsed sides, the reading
  PR 7115 gives a cone's apex and a sphere's pole), and `by_kind`'s
  injectivity on one sheet.
- **The approximated face:** a window on its fit's knot domain, the
  NURBS face's iso-bounded window (`iso_bounded_domain`), and a reading
  of the face through its fit and its distance certificate.

## The approximated half, measured (lane `embedwalk-approx`, 2026-11-25)

An approximated face is read as its fit, a NURBS face, and takes the
NURBS window (`window_of`). On `embed/gate-measure` (`6eb881a63`), the
seven `sweep::all` reproducers refusing "an approximated face": six
pass with the window; the seventh,
`encl_curved_loft_shell::a_moved_plane_meets_a_fitted_cap_along_their_certified_section`,
now refuses at the NURBS window's trimmed-face test, filed as
`a-trimmed-free-form-face-has-no-window-in-the-injectivity-engine`.

## The approximated half: no certificate is composed (2026-11-25)

The "reading of the face through its fit and its distance certificate"
asked for above reads the fit alone; the certificate is not composed
into any reading. The fit is the face's geometry (OFFSET-DESIGN O2:
"geometry questions … read the fit"), the face's edges are sections of
the fit and its chart images lie on the fit's chart, so the boundary
the gate asks embedded is the fit's; the certified distance is the
face's claim about the surface it stands in for, which tier 3
re-derives (O5). Charging it could not certify the described surface
anyway: a surface within δ of an embedded fit may fold at scale δ, so
no touching pair's local certificate carries over, and the described
body's own edges are not stored. The window reads the fit
(`clearance::carrier_lifted`, through `Surface::spline_chart`).

On `embed/gate-measure`, `operand_gate_pose::an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`
goes from a 0.5 s refusal to a 37 s pass with the window: a slow-set
candidate once the gate lands.
