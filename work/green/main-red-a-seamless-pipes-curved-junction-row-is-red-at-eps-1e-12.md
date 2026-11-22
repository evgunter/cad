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
