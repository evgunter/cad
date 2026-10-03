---
id: split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence
kind: issue
title: Split's ON verdicts at margins within ε can mint a pinch from a value coincidence, which D1 tier 3′ (i) forbids
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [3856]
design: true
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


## Ev's ruling (2026-10-03)

In chat: tier 3′ (i) "specifically DOES refer to coincidence < eps, and
says we can't infer intent from that." A value coincidence is not intent
however small its margin, so a pinch minted from a ≤ ε ON verdict, at
δ = 0 and at 0 < δ ≤ ε alike, violates (i). (A first designer round read
"near-coincidence" as the band only and recommended keeping the pinch;
that reading is overturned. Record: `analysis/design-fork/split-band-on`.)

## Designed (2026-10-03, round 2; both designers converged)

- **The line** is structural, read before any surgery: at an ON vertex in
  `split_reduce`'s per-ON-vertex loop, after rules (a)/(b), the count of
  maximal one-side runs of its orbit (`insert::above_runs`). 0 runs (a
  touch, a convex edge per PR 3642, a face in the plane) or 1 run (an
  ordinary cut through a vertex) mint nothing that touches: derived, as
  today. ≥ 2 runs is the pinch class (`NOTCHED`, the D7 mirror lane, the
  both-sided frontier): every success there is a half holding two copies of
  the vertex on one point.
- **Undeclared ≥ 2 runs refuses typed**, the same answer at δ = 0 and at
  0 < δ ≤ ε (nothing structural tells them apart). The text names the
  vertex (or edge) and the side whose pieces would touch. Its recourse is to
  declare it on the split, or move the plane or the geometry. Per D4 (i)
  and SELECT-DESIGN §3d, a contact site has no "tighten the tolerance" arm.
- **A user who means the pinch declares it on the Split node**, the same shape
  as the boolean's declarations (F5: recipe data on the node). The node names
  target vertices/edges asserted to lie on the tool plane, and the kernel
  door takes `SplitDeclarations` ("none" for a plain call). A declaration is
  verified, never trusted: Zero holds; in band it is bridged, as a C4
  `Rest` is; definitely off refuses `SplitDeclarationContradicted`. The
  result carries no new contact record: the copies share the cut vertex's
  point (rung 1), and the intent lives on the node.
- **Consequences.** These pinch rows re-baseline to the declared door with
  the same bodies, each gaining a sibling that pins the undeclared refusal at
  δ = 0 and δ = 1e-10:
  - `notched_block_end_to_end`;
  - `review_m3_pr3_bob`'s table and mirrored fixture (`bob_mirror_pinch_refuses_typed`
    succeeds today, so its name is wrong);
  - `review_m3_pr6`'s mirror rows;
  - the BOOL1 notch rows;
  - PR 3856's rows;
  - `seat8_split_lowering`.

  D7's mirror lane stays, as the mechanism for a declared below-side pinch.
  PR 3642 stands.
- **Ratified text** (the `[ev]` PR's diff): tier 3′ (ii)'s "an op's copies of
  one vertex" is qualified to copies at a structural or declared coincidence;
  D1's "Coincidence discipline in the reduction" gains the derived/declared
  line; the frontier's "single-sided pinches succeed" becomes "declared
  single-sided pinches succeed".
- **Reach past split: a separate row and a separate fork.** The boolean's
  `ContactAcc` vertex-level records come from Zero verdicts with no declaration
  (`corner_kiss_promoted`), and tier 3′ (ii) calls them "declared":
  `work/contact/boolean-vertex-contact-records-are-inferred-from-values.md`.
- **Still owed beside the design:**
  1. The δ = 5e-10 arm. A Zero-decided vertex yields a chart residual of
     about 8δ that the attachment gate refuses in kernel-defect voice. Under
     this design an undeclared pinch refuses earlier, but a declared one still
     meets it. Measure first.
  2. `SliverVertex`'s text says "within tolerance", but it fires beyond ε.
  3. Before building, add one scratch row confirming that a 2-run vertex
     whose sides connect elsewhere refuses rather than succeeds.
- Brief correction: the fixture is `crates/topo/tests/m3_pr3_split.rs`.
