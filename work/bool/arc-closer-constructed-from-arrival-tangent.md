---
id: arc-closer-constructed-from-arrival-tangent
kind: issue
title: "paths: a construct-from-arrival tangent closer — the arc through the departure point and Start whose END tangent is Start.dir"
status: open
opened: 2026-09-02
github: 1578
refs: [1573, 433, BOOL-10, BOOL-12]
---

## From GitHub issue 1578

Opened 2026-09-02; 0 comments.

**Filed from BOOL-12 (PR [#1573](https://github.com/evgunter/cad/pull/1573)) as the schedule for a disclosed deviation.**

BOOL-12's spec asked for the tangent seam member as a CONSTRUCTION: the circular arc through the departure point and `Start` whose end tangent is `Start.dir`, with a sharp departure. The unit built the declare-and-CHECK form on the existing `tangent_arc_to` construction instead (`tangent_arc_to(Start.arrives_tangent())`), for a measured reason: under the construction form the stadium — the member's canonical fixture and an acceptance criterion — cannot close (the constructed cap's start tangent is the incoming straight, so the derived departure junction is an undeclared tangency).

**What remains un-authorable without this form:** an author who knows where the seam is and how the closing arc must ARRIVE, but not the departure angle, must solve for the angle by hand. The construct-from-arrival form would remove that friction: `.angle(θ)` is then unnecessary for the arrival-declared case.

**Shape:** a new arc MODE on the point state (§2c family growth, arc-construction vocabulary, not seam vocabulary), so it lands on the ground BOOL-10 is redesigning (`arc_continue`'s retirement and the declared-subdivision arc form). Take it there or as a small S-BOOL unit after BOOL-10; the both-ends-tangent case stays the seam fillet's.

Refs issue 433, BOOL-12 (`docs/BOOL-12-SPEC.md` deliverable 3), BOOL-10.

## Home

`work/bool/` — the PATHS lattice work the Q1 ruling chain produced is S-BOOL's charter, and the issue names BOOL-10's `arc_continue` retirement as the ground it lands on.
