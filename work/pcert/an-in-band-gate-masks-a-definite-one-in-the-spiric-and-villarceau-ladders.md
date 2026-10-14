---
id: an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders
kind: issue
title: Two pcurve gate ladders escalate on an in-band gate before a later gate decides the class
status: closed
opened: 2026-10-13
priority: P3
closed: 2026-10-14
pr: 5089
---



## What

Two gate ladders in `crates/geom-brep/src/pcurve_cache.rs` decide "every
gate Zero" one gate at a time and return on the first in-band one through
`?`. So a gate read later that would decide the class definitely never
speaks, and the ladder escalates where it has an answer:

- the spiric chart's other-torus test (the loop over `gates` chained with
  `pcurve_spiric_chart_other`, `.map_err(winding_escalated)?`, which breaks
  on the first non-Zero gate);
- the Villarceau incidence gates (`pcurve_torus_villarceau_*`,
  `.map_err(incidence_escalated)?`, where any definite gate is `grazes`).

The identity ladder's rule is: any definite datum answers; an in-band one
is kept, and refuses only where no later datum decides. That rule now has
one home, `topo::boolean::carrier_eq::identity_data` (PR 4918, after
`rim_wedge::same_circle` refused whole ops on circles 1.4 m apart).
geom-brep cannot call into topo, so the rule needs a geom-brep home, or
these ladders spell it out.

Found by PR 4918's second-review sweep (2026-10-13). Not measured.

## Measured (PCERT, 2026-10-14)

Census: both ladders were instrumented to log every call's whole gate
pattern, each gate decided with no short-circuit. The run covered
geom-brep, topo, sweep, step-import, mesh and editor-core in release
(10524 passed), and no call had an in-band gate.

- The Villarceau gates ran 431 times: 421 all Zero, and 10 with the
  first gate already definite (DDDDD, DDDDZ, DZDDD), from grazers and
  fuzz moves. The calls came from `pcurve_villarceau`,
  `envelope_lemma_fuzz`, `at_rest_pcurve_faces`,
  `a_whole_villarceau_circle_bounds_a_torus_face` and
  `pcurves::villarceau_joint_tests`.
- The spiric chart ladder ran 559 times (537 f64, 22 Interval). The
  calls came from `pcurve_spiric`, `spiric_rim`, `review_s12_adv`,
  `shell7_*`, `verbs_shell`, `germ_torus_doors`, `torax_*` and the
  mirror-torus rows. Every non-Zero pattern came from `pcurve_spiric`
  and started on a definite gate.

Constructed on the spiric wall, the masking is real. The wall image is
the own torus's, with one gate `2ε` (in band) ahead of another
`3·K·ε` (definite), at the witness band (1e-9, 1e-8):

| Chart | Before | After |
|---|---|---|
| own torus, centre in band, `R` apart | `Escalated{ChartWinding}` on `_chart_center`, a kernel-defect ending | `CarrierOffChart` |
| mirror torus, `r` in band, axis tilted apart | `Escalated` on `_chart_minor` | `CarrierOffChart` |
| mirror moved apart, `R` in band | `Escalated` on `_chart_major` | `CarrierOffChart` |
| mirror, `R` in band, nothing apart (control) | `Escalated` on `_chart_major` | the same |

On the Villarceau gates the order is not reached through
`chart_pcurve` at the run's `K`.

- At the 64 incidence samples (`R = 2`, `r = 0.7`), a circle's
  first-order departure from the torus has a one-dimensional null
  space: a turn about the axis, which moves no gate.
- The least-squares move that puts any one gate at `m` departs
  `1.02·m` to `1.18·m` at its farthest sample.
- So a circle with a gate at `K·ε` or past it is refused, or
  escalated, by the incidence test before any gate is read. Only `K`
  under about `1/0.9` leaves the window open, and `CAD_AMBIGUITY_K`
  admits any `K > 1`.

## Closed (PCERT, 2026-10-14)

The rule now has a geom-brep home, `geom_brep::ladder::read_identity`.
It is generic over the per-datum reader, and it holds a kept reading's
run, splicing it superseded where a later datum answers.

- The spiric chart ladder and its other-torus ladder read it.
- The Villarceau gates read it too, so one rule covers every ladder. The
  measurement above is in the gate site's comment.
- `topo::boolean::carrier_eq::identity_data` keeps its own copy for now:
  `work/cleave/identity-data-reads-its-ladder-from-geom-brep.md` moves
  it onto the helper.
- The same shape at eight other gate loops in geom-brep is filed as
  `work/pcert/gate-loops-return-on-an-in-band-gate-before-a-definite-one.md`.

Rows: `pcurve_spiric::a_chart_gate_decided_apart_answers_past_an_in_band_one`
(the table's four poses), plus `ladder::tests` (four rows). Mutants,
each red:

- the first-E return, as on main, reds the spiric row and three ladder
  rows (`origin/main`'s `pcurve_cache.rs` against the new row reds it
  too);
- answering on the first definite datum but dropping a kept E that
  nothing later decides reds the control and two ladder rows;
- a kept run spliced whole rather than superseded reds the
  later-definite ladder row;
- the refusing cause taken as the first kept reading rather than the
  first poisoned one reds the unreadable ladder row.
