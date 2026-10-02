---
id: split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence
kind: issue
title: Split's ON verdicts use Band::linear(tol), so a plane within tol of a concave edge may produce a pinch half — near-coincidence becoming contact
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

DESIGN's tier 3' (i) says near-coincidence never silently becomes contact. A plane within tolerance of a concave edge, but not on it, reads the edge's vertices ON and can mint a pinch half whose touching is an artefact of the band. Unverified: whether an in-band ON verdict escalates typed (Q1) or decides. A row: a plane offset by tol/2 from a notch's tip line.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Measured (2026-10-02, review of PR 3856)

Reproduced on main (9768086e8) and on `tquery/split-pinch-shared-point`.
The fixture is the notched block (`m3_pr3_split.rs` `NOTCHED`, height
1), cut by `plane_y(1 + δ)` (origin `(0, 1 + δ, 0)`, normal `+y`) at
`Tol::witness()` (ε = 1e-9). The notch tip line is at `y = 1` exactly.

- **δ = 1e-13, 1e-12, 1e-11, 1e-10:** split reads the tip ON and
  returns the same pinch half as δ = 0. Above has 3 shells and two tip
  copies at `(4, 1, 0)`.
  - On main the pseudomanifold door with no records refuses that half:
    `UndeclaredContact` ×2 `VertexVertex` plus `EdgeEdgeOverlap`. It
    refuses the δ = 0 half the same way, so it never told the two
    apart.
  - After PR 3856 the copies share the tip's point, and the door
    passes the half at every δ in that range. A plane that misses the
    tip by up to 1e-10 now yields a touching body that nothing
    reports.
- **δ = 5e-10:** split refuses, but in a kernel-defect voice: "geometry
  attachment gate: the unified conventional residual at sample 0
  escalated: margin 4.0e-9 lies inside the ambiguity band (1e-9, 1e-8).
  There is no way through: this is a kernel defect; report it". That is
  an in-band input, not a defect, so the refusal's words are wrong.
- **δ = 1e-9, 2e-9, 1e-8:** split refuses typed: "a vertex lies within
  tolerance of the split plane … move the split plane or the
  geometry". This is Q1's escalation, as it should read.

So after PR 3856 the door cannot catch this: on one shared point, a
band-made pinch and a designed one are the same body. The fix has to
be at split's ON verdict. Today a margin below ε decides ON, and only
a margin inside the band (ε, 10ε) escalates typed. One fix is for a
vertex the plane misses, even by less than ε, to escalate typed (Q1)
rather than decide ON; which ON test is right is CLEAVE's to design.
**P1:** a ratified "never" (D1 tier 3′ (i),
near-coincidence never silently becomes contact) is violated on a
reachable input.

