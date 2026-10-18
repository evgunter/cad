---
id: onpair-has-no-rung-3-lane-for-a-spline-carrier
kind: issue
title: OnPair refuses a spline carrier (Unimplemented): no between-sample bound like Intersection's rung-3 lane
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## Finding

Filed by TANG's `OnPair` lane (`tang/onpair-description`), from its review (m5).

`Intersection` with a `Curve3::Nurbs` carrier between two analytic
surfaces certifies through the rung-3 lane (`analytic_rung3`), which
bounds the carrier between the schedule's samples. `OnPair` has no such
lane, so certification check 1 (`geom_brep::certify::run_checks`)
refuses a spline carrier on a pair `CertifyError::Unimplemented` rather
than certify it on nine samples
(`certify::tests::on_pair_meters_incidence_on_both_surfaces` pins the
refusal). No door mints one today (the must-carry doors' carriers are
lines and conics); a fitted smooth seam would need the lane.
