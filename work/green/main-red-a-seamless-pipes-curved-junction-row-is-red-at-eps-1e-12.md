---
id: main-red-a-seamless-pipes-curved-junction-row-is-red-at-eps-1e-12
kind: issue
title: main red (slow set, eps 1e-12): a_seamless_pipe_cut_by_an_oblique_plane_measures::a_curved_junction_certifies_across_the_departure_arm
status: open
opened: 2026-11-22
priority: P0
cost: E
---


Reported by BOXFINISH's volume-premises lane (2026-11-22), which met it
in its head-against-merge-base slow-set differential for PR 7344 and
found no row holding it; filed by the BOXFINISH orchestrator, who has
NOT re-run it.

`topo::all a_seamless_pipe_cut_by_an_oblique_plane_measures::a_curved_junction_certifies_across_the_departure_arm`
is red at `CAD_TOLERANCE_EPS=1e-12` on main at `ae076e05` (the lane's
merge base), in the CI profile, and equally on the lane's head, whose
diff (`topo::props`'s volume re-derivation at an exact-witness scalar)
does not reach it. It is green at the default ε and at 1e-6 in the same
runs.

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p topo --ignore-default-filter \
      -E 'test(/a_curved_junction_certifies_across_the_departure_arm/)'

Not known: the payload; which merge turned it red; whether the nightly
of 2026-11-21 shows it. `git grep` finds the test named in rows of
CIW, CLEAR, EMBED and PCERT, none of them a red at 1e-12; the lane's PR
body lists eleven other inherited reds with their rows.

This row records the red. Whoever owns the junction's certificate
(the test's subject) takes it from here.
(BOXFINISH orchestrator)

## Measured (GREEN, `green/seamless-pipe`, 2026-11-22)

**The payload.** At ε = 1e-12 the row panics in `pipe::<Interval>`,
at the roof's `set_edge_curve`, in the frame along `(1, 2, 3)` with
the floor 100 m up the axis:
`Certification { error: AnalyticRung3(Stalled { operand: Cylinder, cause: Indeterminate { margin: Enclosure { lo: 0.0, hi: 2.193467629756736e-12 }, band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("ssi_hull_sup") } }) }`.
The `f64` body builds and certifies `Holds` there. The same build
also reds `a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval`
at 1e-12, which no row held. Measured per frame and pose on main
`fc75e13cdd`: only two of the twenty `Interval` builds refuse, the
`(1, 2, 3)` frame (2.193e-12) and the far frame `(6e2, −4.2e2, 2.4e2)`
(1.415e-12), both at the 100 m pose. Every pose builds at 1e-6.

**The nightlies.** The 11-21 nightly (44870651337, `f5ef8f2b`) has the
row green in its 1e-12 shard 3. The 11-22 nightly (44989179403,
`787aed5f`) failed its build job and ran no suite, so it shows nothing.

**The merge.** Bisected first-parent with a probe that spawns the row
at 1e-12 inside the gate's default-ε job: green at `afa8184dcf`,
`194acb6efb` and `e4d679d37e`, red with the same payload at
`9bc9d6517b`, `adb7ec1291`, `55e8566eeb` and `fc75e13cdd`. **The merge
is `9bc9d6517b`, PR 7131 (`onehome/carrier-frames`).**

**What it is: a sound refusal in PR 7131's accepted cost class.** A
`Stalled` refusal is undecided, not a wrong answer. Under PR 7131's
mutant patch at 1e-12, F1X alone builds both poses, A alone narrows
them (1.619e-12 and 1.079e-12) without building them, and F1F moves
neither. So the cause is **bracket**
(`geom_core::test_support::at_the_frame_cost_band`), the same as
`verbs_tubewall_r1_probes`' stall on `ssi_hull_sup`. Ev accepted that
cost on PR 7131 (2026-11-20). The PR measured it with the `ci` profile,
which leaves out the slow set, so neither row was on its list.

**Greened by re-blessing.** `pipe_at_interval` pins the two refusals at
ε ≤ 1e-12 by type, predicate and margin band, `(2.1e-12, 2.3e-12]` and
`(1.35e-12, 1.5e-12]`. A worse margin, another refusal, or a build
there is red. Both rows keep every other check at every pose, including
the `f64` verdict and its across-pair certificates at the pinned poses.
