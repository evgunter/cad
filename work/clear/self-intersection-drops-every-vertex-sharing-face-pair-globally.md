---
id: self-intersection-drops-every-vertex-sharing-face-pair-globally
kind: issue
title: clearance's self-intersection drops every face pair that shares a vertex, and a face against itself, globally - vacuous on a cone's lateral faces or a triangle-section loft
status: closed
opened: 2026-10-06
priority: P0
cost: H
branch: clear/selfx
pr: 5955
closed: 2026-10-29
---


## Finding

`editor_core::clearance`'s self-intersection candidate filter
(`crates/editor-core/src/clearance.rs`, `clearance_with`, the "wedge
rule" block) keeps a pair only when
`x.face != y.face && x.vertices.is_disjoint(&y.vertices)`. So it never
examines:

- a face against itself, and
- two faces that share any vertex, anywhere on them.

The comment beside it says so: "**The exclusion is GLOBAL where the
justification is LOCAL** … A cone's apex, where every lateral face
shares one vertex, removes the whole lateral check". It also says the
gap is "stated … in the unit's deviations". No row in `work/` scheduled
it, so this file is that schedule.

Consequences: `self_intersection` passes a body unexamined when all its
faces share vertices. Examples are a triangle-section loft, where every
wall shares a seam with both neighbours and a cap vertex with the
other, and a cone's lateral faces. Adjacent loft walls that cross away
from their shared seam can never be seen, which is the shape of
`work/carvetail/loft-between-opposite-turning-joints-reverses-a-seam-between-stations.md`
and of a roll near a half turn.

The local justification (faces that share a vertex meet there) needs a
local discharge, not a global drop. One proposed shape, from CARVE's
2026-10-06 designers: near the shared boundary, a cell pair whose normal
cone is narrower than a right angle cannot cross; away from it, the
ordinary distance test decides. The design is open (`design: true`).

## Why it matters now

CARVE's two designers on the loft's section placement
(`work/carve/self-overlapping-spines-build-and-validate.md`) both put
the loft's embedding certificate on this engine at a certifying scalar
(D1, Ev's B at #1737), after SHELL-3 moves it into `topo`. Whether they
need adjacent pairs is part of their reconciliation. Either way, the
engine's answer for any body is vacuous on the pairs this drops.

Filed by the CARVE orchestrator, 2026-10-06, from two designer reports
(read-only lanes). Not measured; read off the filter and its comment.

## The design (two designers, converged after three rounds; CLEAR orchestrator, 2026-10-27)

- **The question is embedding.** "Is the boundary embedded?" replaces
  "are the non-adjacent pairs apart?"
- **Every face pair is examined, a face against itself included.** A
  face on an analytic carrier is one-to-one by kind; a torus needs
  R > r decided at a funnel site. The discharge is a receipt line, not
  a silent skip.
- **Touching vs clear is decided per cell pair.**
  - A cell is clear of a shared edge or vertex when `cell_region`'s
    existing readings say so, in metres.
  - A clear pair is decided by distance, as today.
  - A touching pair is never decided by distance. It discharges only by
    a local certificate. Otherwise it splits, and at the depth floor it
    refuses typed, naming the two faces and the shared cell.
- **The certificate has three arms.**
  - *Across faces, at a shared edge or vertex: departure.* The edge
    projects to a Lipschitz graph, and each face's departure cone lies
    strictly on its own side of it. At a vertex, the projected sectors
    are disjoint.
  - *Within one face, against itself or across its chart seam: a
    strongly monotone projection.* J(L∘S)+Jᵀ is positive definite, with
    L taken from the pair's centre Jacobian. Collapsed charts use the
    cone's development, or a graph over the tangent plane at a pole. A
    NURBS corner collapsed to a point refuses.
  - *Declared wedge-0/2π edges: second order.* The second inward
    derivative of the faces' height difference has a definite sign.
- **The proposal in this row's body is unsound,** in both its
  right-angle and half-space forms ("normal cone narrower than a right
  angle cannot cross"). Counterexample: a flat ribbon swept 370° along
  a planar arc is one NURBS wall whose normals are all one vector, and
  it overlaps itself.
- **`min_separation` and `clearance ≥ c` drop their pairing rule.**
  - They are metric doors with no topology. Scopes that meet are at 0,
    with the witness named on the shared cell.
  - `NoAdmittedPair` and `Window::vertices` retire.
  - Counterexample to keeping the rule: two faces sharing a vertex, like
    the arms of a U, that come within 0.1 m at the tips. The rule skips
    the whole pair, so the tips are never measured.
- **A pair with a recorded contact refuses typed** until the handoff to
  CONTACT-DESIGN is designed.
- **What is Ev's:** only the rewording of `docs/ERROR-DESIGN.md`'s
  *Self-intersection scope* clause, on the `[ev]` PR.

The unit pins these rows:
- the 370° ribbon reads Violated;
- the carvetail fold reads Violated, with a witness at v ≈ ½;
- a cone, a pyramid and an embedded triangle loft read Holds;
- a declared wedge-0 kiss holds;
- a `clearance ≥ c` between scopes that share an edge reads Violated
  at 0, on the shared cell.
- a large curved body certifies within the default budget, with the
  cell-pair counts reported. Candidates: a long helical sweep, a full
  torus, a big cylinder shell with a thin wall. A case that blows the
  budget is a finding, not a re-tuned dial (Ev asked, #5866).

It lands on the moved engine, after `SHELL-3`. The analytic arms can
land before `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch`;
free-form walls need that row.

## Ev's ruling (#5866, 2026-10-28)

Approved. Ev asked what the correct semantics are: "if the point is to
detect self-intersections of the surface then it's presumably going to
have to, like, show that the regions that intersect are separated by a
chart region that maps to a region that definitely doesn't intersect,
or something like that". On the statement that *Holds* certifies
f : M → ℝ³ injective, with touching cells certified injective on their
union and non-touching cells at positive distance, Ev wrote "ok great!",
and asked that the clause lead with "it proves injectivity". The clause
in `docs/ERROR-DESIGN.md` now does. The unit builds against that clause.

## Closed (2026-10-29, #5955)

**What landed.** `topo::injective` proves f : M → ℝ³ injective, per E7 as Ev ruled on #5866.
- Every face pair is examined. An analytic face is discharged by kind, a torus only once R − r > 0 is decided.
- Touching cells are decided only by a local certificate:
  - *monotone*, within one face;
  - *departure*, across a shared edge or vertex, on order-free face presentations.
- Non-touching cells are decided by distance, with a verified coincidence witness.
- The metric doors read 0 where scopes meet.

**Left out, each on its own row or refusing typed:**
- The second-order arm. A cusp refuses typed. Filed as `the-second-order-arm-certifies-a-declared-wedge-0-edge`.
- The CONTACT-DESIGN handoff. A recorded contact refuses typed. Filed as `a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off`.
- Curved shared edges. These refuse at depth 0. Filed as `departure-presents-no-curved-shared-edge`.
- The by-kind premise. Filed as `an-analytic-face-may-cover-its-carrier-more-than-once`.
- Exact zeros in an assertion. Filed as `an-assertion-over-scopes-that-meet-refuses-a-window-superset-for-an-exact-zero`.

**Review.**
- DUAL concurrent: R1 APPROVE-WITH-FIXES, R2 NOT-MERGEABLE-AS-IS. Blind-coded tally 1: R2's order-dependent presentation, a soundness MAJOR with a red probe.
- Two fix passes. A focused review between them read APPROVE-WITH-FIXES.
- The row is `docs/dual-review-log/rows/PR-5955.md`.
