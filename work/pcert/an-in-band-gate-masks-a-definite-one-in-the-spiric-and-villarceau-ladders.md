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

On the Villarceau gates, what reaches them through `chart_pcurve`
depends on the gate and the torus. This was measured per gate, and is
corrected by PR 5089's second review, which found the first draft's
single-pose claim too wide.

- **`R = 2`, `r = 0.7`, 64 incidence samples.** A circle's first-order
  departure from the torus has a one-dimensional null space: a turn
  about the axis, which moves no gate. The least departure (minimax
  over the samples) that puts any one gate at `m` is `0.995·m` to
  `1.04·m`. So a gate in band or past it is read about as far off by
  the incidence test, which decides first. (The first draft quoted
  `1.02·m` to `1.18·m`, the least-squares move's maximum, which is not
  a lower bound.)
- **Near a horn torus the plane gate is cheaper.** On `R = 2`,
  `R − r = 1e-6`, band `(1e-9, 1e-8)`, the Villarceau circle moved
  along its own normal by `3e-9` to `1e-8` escalates on `_plane`, and
  by `3e-8` it is read on the torus with `_plane` definite and refused
  as grazing it.
- **The masking order needs more.** It needs a gate read before
  `_plane` (`_offset`, `_equator`) in band with `_plane` past it. On
  that near-horn pose, with `_plane` at `3e-8` or `5e-8`, `2e-9` or
  `5e-9` of offset or of equator departs about its own margin (`2.6e-9`
  for `2e-9` of offset, `2.03e-9` for `2e-9` of equator). The incidence
  test escalates on it before the gates read anything, at both
  `R − r = 1e-6` and `1e-4`.

So the masking order is not met at the door on any pose measured, but
the ladder's undecided arm is: the near-horn `_plane` rows reach it.

## Closed (PCERT, 2026-10-14)

The rule now has a geom-brep home, `geom_brep::ladder::read_identity`.
It is generic over the per-datum reader, and it holds a kept reading's
run, splicing it superseded where a later datum answers.

- The spiric chart ladder and its other-torus ladder read it.
- The Villarceau gates read it too, so one rule covers every ladder. The
  measurement above is in the gate site's comment. Their undecided arm
  is pinned at the door by
  `pcurve_villarceau::a_plane_gate_in_band_near_the_horn_escalates_on_it`:
  a mutant that reads the Villarceau ladder's undecided arm as Zero reds
  it. A Villarceau ladder put back to the first-E loop reads the same
  at the door on every pose measured, and is held by `ladder::tests`
  alone.
- `topo::boolean::carrier_eq::identity_data` keeps its own copy for now:
  `work/cleave/identity-data-reads-its-ladder-from-geom-brep.md` moves
  it onto the helper.
- The same shape at eight other gate loops in geom-brep is filed as
  `work/pcert/gate-loops-return-on-an-in-band-gate-before-a-definite-one.md`.

Rows: `pcurve_spiric::a_chart_gate_decided_apart_answers_past_an_in_band_one`
(the table's four poses), plus `ladder::tests` (four rows). Mutants,
each red:

- the first-E return, as on main, reds the spiric row and three ladder
  rows. `origin/main`'s `pcurve_cache.rs` against the new row reds all
  three of its poses, since the row reads every pose before asserting;
- answering on the first definite datum but dropping a kept E that
  nothing later decides reds the control and two ladder rows;
- a kept run spliced whole rather than superseded reds the
  later-definite ladder row;
- the refusing cause taken as the first kept reading rather than the
  first poisoned one reds the unreadable ladder row.
