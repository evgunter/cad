---
id: mint-has-no-route-to-the-fitted-general-circle-arm
kind: issue
title: no mint site reaches certify_fitted's Circle-carrier arm, so a general sphere circle's face stays uncached (DESIGN frontier (c))
status: closed
opened: 2026-10-01
priority: P1
cost: H
refs: [S331, D36]
branch: pcert/general-circle-fitted-route
closed: 2026-10-02
pr: 3733
---


Filed by the PCERT orchestrator, 2026-10-01, while answering Ev on
PR 3617.

An oblique fillet trihedron's corner octant is a sphere face whose
boundary arcs are circles neither polar nor meridian to the stored
chart axis. The closed-form lane covers only those two classes, so the
face is excused as uncovered (`mint_faces`' swallow) and reaches rest
storing no row. The certified route exists — `PcurveCache::certify_fitted`'s
Circle-carrier arm, and the mint holds the fitted door
(`AtRestPolicy::fitted_lane`) — but no mint site sends a Circle carrier
on a sphere into it. `docs/DESIGN.md` frontier entry (c) names exactly
this gap; the doc on `certify_fitted` names it as its first waiting
consumer.

Closing it empties most of the "uncovered" class `D36` names; what
remains outside every route (an oblique torus circle, a tilted cone
section, a NURBS carrier on an analytic chart) is what the `S331`
ruling's coverage-status answer is for. Specified after that ruling.
