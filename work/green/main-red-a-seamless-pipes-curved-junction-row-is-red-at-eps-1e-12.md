---
id: main-red-a-seamless-pipes-curved-junction-row-is-red-at-eps-1e-12
kind: issue
title: main red (slow set, eps 1e-12): a_seamless_pipe_cut_by_an_oblique_plane_measures::a_curved_junction_certifies_across_the_departure_arm
status: closed
opened: 2026-11-22
priority: P0
cost: E
pr: 7402
branch: green/seamless-pipe
refs: [pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted]
closed: 2026-11-23
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

## Its sibling row is red with it, one payload (GREEN U9, 2026-11-22)

(GREEN, `green/held-reds`) This row also holds
`topo::all a_seamless_pipe_cut_by_an_oblique_plane_measures::a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval`,
which PR 7344's body cited to QUAD's closed
`quadrature-interval-floor-grows-with-the-body-past-the-band`. On
current main (`5ea1ac0a70`, hosted, throwaway run 45051569400 with both
names out of the slow set) both rows are green at the default ε and at
1e-6 and red at 1e-12, and both panic at the same unwrap
(`a_seamless_pipe_cut_by_an_oblique_plane_measures.rs:239`):

    Certification { error: AnalyticRung3(Stalled { operand: Cylinder,
      cause: Indeterminate { margin: MarginDiag(Enclosure { lo: 0.0,
      hi: 2.193467629756736e-12, .. }), band: Band { zero: 1e-12,
      escalate: 1e-11 }, predicate: Some("ssi_hull_sup") } }) }

The nightly of main `1942290ead` (run 44989234943) shows the same pair
red at 1e-12 with the same payload. One cause, so one fix greens both.

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
`787aed5f`) failed its build job and ran no suite. The dispatch
44989234943 (on `green/nightly-build` at `1942290ead`, main `4d3402ecb1`'s
kernel) did read 1e-12 with the slow set, and both pipe rows are red
there at `hi: 2.193467629756736e-12`. U12 is attributing that run's
other new 1e-12 reds.

**The merge.** Bisected first-parent with a probe that spawns the row
at 1e-12 inside the gate's default-ε job: green at `afa8184dcf`,
`194acb6efb` and `e4d679d37e`, red with the same payload at
`9bc9d6517b`, `adb7ec1291`, `55e8566eeb` and `fc75e13cdd`. **The merge
is `9bc9d6517b`, PR 7131 (`onehome/carrier-frames`).**

**What it is: a sound refusal, past the cost PR 7131 counted.** A
`Stalled` refusal is undecided, not a wrong answer. Under PR 7131's
mutant patch at 1e-12, F1X alone builds both poses and F1F moves
neither. Mutant A also narrows them (1.619e-12 and 1.079e-12), out of
their windows, so it reds the pin as well. The cause is **bracket**
(`geom_core::test_support::at_the_frame_cost_band`), and A also
narrows it. What Ev accepted on PR 7131 was a counted cost, *"about
11–12 rows … at margins of 1.02–1.32e-12"*. That count was taken on
the `ci` profile, with no slow row at 1e-12. These two rows are not in
it, and 2.193e-12 is past its range. So they are pinned against
ONEHOME's P1 row `pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted`.

**Greened by a pin.** The test file's `FRAME_COST` table, keyed by the
frame's axis and the floor height, pins the two refusals at
ε ≤ 1e-12. The windows live there. A build, another refusal or a
margin outside the window is red. Both rows keep every other check at
every pose, including the `f64` verdict and its across-pair
certificates at the pinned poses. At 1e-12 the closed-form row skips
the pinned poses, with a printed line, so the kilometre-out pose's
convergence at `Interval` goes unread there until the ONEHOME row
closes.

## Closed (GREEN orchestrator, 2026-11-23)

PR 7402 pins both seamless-pipe rows at ε 1e-12 as PR 7131's frame
cost, keyed on values through `FRAME_COST`, against ONEHOME's
`pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted`. Ev ruled on
PR 7505 (2026-11-23 06:20Z) that the full count stands. The PR's gate
passed on `8e58bee11f`, and the pins held at 1e-12, 1e-6 and 1e-9 in
throwaway run 45070512697.
