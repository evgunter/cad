# SELFX — self-intersection proves the boundary injective

Unit: `work/clear/self-intersection-drops-every-vertex-sharing-face-pair-globally.md`
(P0). Read its `## The design` and `## Ev's ruling` sections, and
`docs/ERROR-DESIGN.md`'s *Self-intersection scope* clause (Ev, #5866).
Those three together are this spec. Program: CLEAR. Branch:
`clear/selfx`. Review: DUAL, concurrent pair (H), on a frozen head.

The engine is `topo::clearance` (SHELL-3, #5909). NURBS walls have
windows since NURBSWIN (#5927), so free-form faces are in scope.

## What is true when this lands

1. **The embedding door asks one body whether f : M → ℝ³ is
   injective**, per the clause.
   - Every face pair is examined, a free-form face against itself
     included.
   - A face on an analytic carrier is injective by its kind. A torus
     needs R > r decided at a named funnel site. The discharge is a
     receipt line, not a silent skip.
   - Touching versus clear is decided per cell pair, in M, through
     `cell_region`'s readings in metres.
   - A clear pair is decided by distance.
   - A touching pair discharges only by a local certificate. Otherwise
     it splits, and at the depth floor refuses typed, naming both faces,
     the shared cell and which certificate arm failed.
   - A non-touching pair at distance 0 is `Violated`, with a verified
     witness.
2. **Three certificate arms, each a funnelled interval decision at a
   named site:**
   - *departure*, across a shared edge or vertex;
   - *monotone*, within one face: J(L∘S)+Jᵀ positive definite, with L
     from the pair's centre Jacobian. A collapsed chart (a cone apex,
     a sphere pole) uses its development or a tangent-plane graph. A
     NURBS corner collapsed to a point refuses typed.
   - *second order*, at a declared wedge-0/2π edge.

   Each new site gets a row in `docs/predicate-dimension-audit.md`.
   Implement the arms in that order. If the second-order arm proves
   larger than its row says, land the first two, have the cusp seam
   refuse typed, and file the arm as its own row: Ev's ruling allows
   that order, but say so in the PR.
3. **The metric doors lose their pairing rule.** `min_separation` and
   the bound walk measure every named pair: scopes that meet read 0,
   with the witness named on the shared cell.
   - `NoAdmittedPair` and `Window::vertices` retire.
   - Rows that pinned the old drop are re-pinned to the new semantics,
     and the PR says which moved and why.
4. **A pair with a recorded contact refuses typed** until the
   CONTACT-DESIGN handoff is designed. File that handoff as a row.
5. **The rows the unit pins**, each of which must go red under the
   failure it names:
   - a 370° flat ribbon: `Violated`. It must go red if a normal-cone
     rule replaces the monotone arm.
   - the carvetail fold: `Violated`, with a witness at v ≈ ½.
   - a cone, a pyramid and an embedded triangle loft: `Holds`. They
     must go red if the global drop returns.
   - a declared wedge-0 kiss: `Holds` (or a typed refusal, under
     item 2's fallback).
   - `clearance ≥ c` between scopes that share an edge: `Violated` at
     0, on the shared cell.
   - a large curved body certifies within the default budget, with
     cell-pair counts reported (a long helical sweep, a full torus, a
     big thin-walled cylinder shell). A case that blows the budget is
     a finding, not a re-tuned dial.

## Ground

- `crates/topo/src/clearance.rs`, `cell_region.rs`, editor-core's
  `clearance.rs` and `measure.rs`, `docs/predicate-dimension-audit.md`.
- SYMCLEAR (#5928) is in review and touches editor-core's half and
  `topo::ClearanceLane`. Merge main as it lands.
- Announce any seam with `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full first.
- Never arm a waiter or a background chain. Anything that outlives
  600 s runs `setsid`-detached and is polled in the foreground.
- CI is the verification of record. The diff seeds `topo`, so allow
  25 min.

## Report

≤150 lines:
- each certificate arm and its soundness argument, in one paragraph
  each;
- which rows pin what;
- the large-body budget numbers;
- the goldens moved and why;
- the PR and its CI.
