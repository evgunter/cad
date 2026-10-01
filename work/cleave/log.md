# CLEAVE log

## Opened at REACH's cut (2026-10-01)

Opened by the REACH orchestrator at its first sitting. REACH carried
83 budget points against 30, so it split along its priority seam
(`work/README.md`, Track size). Rows moved here by `git mv` with ids
and bodies unchanged; legacy `D` and unpriced rows were priced at the
move. No unit dispatched. — (REACH orchestrator)

## First sitting (2026-10-01)

Track claimed (`status: active`). — (CLEAVE orchestrator)

- `topo-mints-indeterminates-outside-the-funnel` parked on PR 3513
  (TOPO, open): that PR rewrites the `plane_eq.rs` and `sectors.rs`
  mints this row lists and files sibling rows under `work/topo/`.
  Weighing the row's `Contradicted`/`Escalated` question while the
  ground moves under it would be weighed twice. Re-open when 3513
  lands, and re-take the site list against the merged tree first.
- Wave 1 dispatches, each measuring on main before code:
  - **section rings** (`cleave/section-rings`): the three invalid-split
    rows (U-cutter pockets, bore annulus, steep ringed cap) as one
    lane, since the plan suspects one defect in how a section face with
    holes is encoded. Review: **dual** — the section-face encoding is
    shared by every split and hard to change later.
  - **interior witness** (`cleave/interior-witness`): the flush
    contained operand's `RayExhausted`, taking the item's first
    direction (a face-interior witness gives the answer). Review:
    **single, full** — a new containment witness can be wrong silently.
  - **edge midpoint** (`cleave/edge-midpoint`): one home for the
    point halfway along an edge, the curved/chord disagreement decided
    there. Review: **single, style**.
  - **split tangency declaration**: the declaration's shape is open
    (several viable answers), so the designer pair weighs it before any
    lane builds; `design: true` set on the row.
- Held for wave 2 (four cores, one build mutex): the graft
  reachability measurement and the `rehome_rings` reproduction.
- **Split tangency fork** weighed by the designer pair (design-fork row
  33). Both reports reject the framing and agree: derive an in-plane
  edge's side from convexity (`enters_material` on the two flanking
  faces) in rule (b), with no declaration. That revises the second
  half of Ev's 2026-09-24 ruling, so it went to Ev as PR 3642
  (`needs_ev`). The derived rule is needed under either answer. Its
  lane is held until the section-rings lane reports, because both work
  in `splitting/`, and its merge waits on Ev's answer. Neither designer
  executed its claim that the block ∪ slab repro then completes; the
  lane measures that first.
- Edge midpoint: PR 3645 is up and green, with style review dispatched.
  The `finish.rs` `Spiric`/`Nurbs` chord arm was measured as latent
  (`gate_operand` refuses those kinds first). The sweep went beyond the
  row into sweep, step-import and ssi.
- Wave 2: the `rehome_rings` reproduction and the graft reachability
  measurement are dispatched to one lane, one after the other, each
  with its own PR (`cleave/rehome-rings`, `cleave/graft-reach`).
- Interior witness, PR 3655 (green), now in full review. Measured: both
  probes are reachable (`ops.rs` on `(a∪c)∪b`, `finish.rs` on
  `(a∪c)∪(b∪x)`), and the `solid_contain.rs` schedule walk never fired.
  Decided: it lands even though the two newly fused `r4tri` orders
  publish without `b`'s names (the fold discards `b` whole). The
  geometry is right in every order, and no name denotes different
  geometry in two orders. The naming question is EMIT's, filed as
  `work/emit/a-member-the-fold-discards-whole-is-cited-nowhere-though-it-lies-flush.md`.
  The alternative was to hold a P0 wrong refusal until that row is
  settled.
- `rehome_rings` reproduced: a bore in the lune refuses `TornComponent`
  on a split and on `BoolPlanar` booleans. It is a wrong refusal, not a
  silent misplacement. Moved to P0. Fixed in PR 3660 (green), which is
  in full review.
- Graft reachability: no boolean reaches the plain certify, but the
  public `insert_void` door does. The measurement is in the row, which
  is back to `open` with `design: true`. Its question is the same as
  SHELL's `plain-transform-rigid-still-refuses-the-m7-8-class`, so one
  designer pair is weighing the class across both doors. (SHELL has no
  orchestrator; I will note this on its log when the weighing returns.)
- Graft/transform lane fork. The designers converged after one
  reconciliation round: the right is the scalar's, so the lane is
  sealed and `AtRestPolicy::nurbs_lane()` holds it; the transform reads
  it; the void graft carries certificates through `RemapKeys`; a scalar
  without the lane gets its own refusal variant. This is not a fork for
  Ev: it applies H5 ruling 3 and changes only agent-written text, so
  there is no `[ev]` PR and no fork-log row. Ev hears about it in chat.
  SHELL's `plain-transform-rigid-...` row was claimed (git mv) under
  the graft row, as was the forgery row. The `cleave/nurbs-lane` lane
  is dispatched for the transform, void, sealing and variant parts.
  Review: **dual**, because it reshapes a certification surface shared
  crate-wide. The mint-door collapse (`set_edge_curve` reading the
  policy, 46 call sites) follows as a second unit.
- Rehome rings merged (PR 3660) after a full review and one fix pass.
  The outcome depends on the pose: a circular cap refused
  `TornComponent`, and an elliptic run returned a silently wrong body
  (`Ok`, with the bore missing from its half). Both are fixed by the
  carrier walk. Filed `carrier-walk-none-is-answered-four-ways` (P1)
  and `work/exch/infer-outer-reads-an-arcs-sag-off-a-sample-polygon`
  (P3). TANG's `arc-aware-point-in-loop` may now be closable; that is
  TANG's call.
