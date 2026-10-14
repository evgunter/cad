---
id: boolean-gate-mints-route-through-the-funnel
kind: unit
title: boolean, census and merge gates build their decided-sign escalations by hand, off the log (mints C10-C12, C19, C23)
status: closed
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [a-gate-rejection-of-a-decided-enclosure-bisects-to-budget]
branch: cleave/boolean-gate-mints
closed: 2026-10-11
pr: 4613
---

Unit 4 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## Sites

- **C10, `boolean/contain.rs:626` (`bool_contact_arc_end_vertex`, I11).**
  Every end decided against every vertex, and the point is still in the
  band of the end. The two passes disagree by construction. Decide once,
  or type the finding (design item 7).
- **C11, `boolean/sectors.rs:418` (`bisector_zero_refusal`).** An honest
  `±zero` enclosure, built by hand and off the log, reachable at K ≤ 2.
  Route it through the decision's gate.
- **C12, `boolean/plane_eq.rs:102` (`orientation_zero`, I17).** A decided
  Zero with its decided margin. Route it through `decide_nonzero`.
  - The undeclared caller is `plane_eq.rs:254`.
  - The declared caller, `carrier_eq.rs:726`, is D10-held. Leave it, or
    let it take the same door.
- **C19, `census.rs:2583` (`material_wedge_side`, I8, on RESTREAD's
  ground).** `classify_dihedral` reads `Transverse` after the edge screen
  decided agreement. Decide once, so that the second question takes the
  screen's verdict as typed input.
  - Today `own_close` gives this arm the defect ending because its
    margin is `INVALID`.
  - Keep that ending by type, not by `is_invalid()`.
- **C23, `merge_faces.rs:2901` (`LoopWinding` decided zero, on FUSE and
  TOPO's ground).** The decided margin is honest, but it is built by hand
  and off the log. Route it through `decide_nonzero`. Its `SizedDecision`
  ending (`LOOP_WINDING`) stays as it is.

Announce the RESTREAD, FUSE and TOPO seams in the PR.

## A cost to note

Routing a decided verdict through a gate door adds rejections that the
box driver bisects until its budget runs out. That is SCALAR's open
`a-gate-rejection-of-a-decided-enclosure-bisects-to-budget` (P3, E). It
is not a blocker, because every existing gate rejection has the same
cost.

## Built (branch `cleave/boolean-gate-mints`)

Measured first on `origin/main` `40ef883718`, at ε = 1e-9, 1e-6 and
1e-12. All five sites reproduced at every row, and no site's refusal was
on the frame's escalation log:

- **C10:** a half-disc cap whose arc end stands `2Kε` off its stored
  corner, probed at the carrier's end. The pre-pass returned
  `Escalated { ArcEnd, Decided }` over `MarginDiag::INVALID`, rendered
  "undecided: margin is invalid".
- **C11:** `bool_sector_bisector_side`'s `±zero` enclosure (unit level;
  the arm needs K ≤ 2).
- **C12:** two parallel planes over an extent of radius `ε/2`:
  `bool_plane_orient` decided Zero, handed back by hand.
- **C19:** two blocks whose declared faces cross at `α = 0.05` along a
  line both crossing edges hug (`φ·sin α ≤ ε`). The edge screen passed,
  the dihedral read `Transverse`, and the census raised `CensusEscalated`
  over `INVALID` (`material_wedge_side`), with the defect ending.
- **C23:** a zero-winding outline, built by hand.

Per site:

- **C10** is two definite readings that disagree, so it is a typed
  finding, not an escalation. The vertex pass placed the point clear of
  the edge's corners, and the arc pass placed it at the arc's end. The
  four `bool_contact_arc_end_vertex` readings are decided once through
  `decide_magnitude_reported`, which fits because they are norms. They
  ride in `ContainError::ArcEndClearOfCorners(Box<ArcEndGaps>)`.
  - The boundary pass reads a conic's ends through `Curve3::eval`, the
    evaluation the certifier pins (`carrier_endpoint_{start,end}`), so
    the two passes stop reading one end two ways.
  - Which arm is reached, and from where:
    - **The band's own parting** (every end at a corner, the passes
      parted by `K ≤ 3`): reachable at the run's band, at every caller.
    - **An end off both corners** (`ArcEndGaps::off_corner`): through
      the public `contfp` door at a caller's finer band; and at the run's
      band only where the certifier's `distance` and the pass's `norm3`
      part by rounding across a band narrower than that (`K` near one).
      The census reports it as `ValidationError::ArcEndOffCorner` with
      the defect ending: the kernel's two readings of one end disagree.
      At `carrier_touch::ball_off_face` the arm is unreachable outright,
      since a foot is placed only clear of every boundary carrier by at
      least `escalate`.
  - `Escalation::Decided` had no other raiser, so it is retired, along
    with its two `CONTAINMENT_RAISED` rows. `(Boundary, Decided)` was
    already raised nowhere.
- **C11** goes through `k_stats::gate_rejection` with
  `SizedPass::NonZero`, so it is on the log and tagged Zero.
  - `vtxfac` quotes the bisector's own reading, on the log under the
    predicate that decided it: `sectors::pierce_side_reported` (over
    `geom_brep::enters_material_reported`) keeps the Zero an `On` was
    decided on with its predicate (`OnReading`), and `Entry::on` carries
    it to the refusal.
  - `recl` quotes `±zero`: a `PairRecord` code keeps no margin, and a
    coplanar pair's `On` is read off no bound at all. The site is pinned
    and is listed as residue on `hand-minted-indeterminate-ratchet-gate`.
- **C12** goes through `decide_nonzero` on the undeclared ladder. The
  declared `carrier_eq` caller keeps `orientation_zero` (D10-held).
- **C19** is decided once. The dihedral is the last carrier screen: a
  definitely transverse pair does not hold the crossing and answers for
  nothing, as `CrossingSideVerdict::Undecided`'s contract already said.
  The arm and its mint are gone.
- **C23** goes through `gate_rejection` with `LOOP_WINDING.passes`.

Endings, before → after, at every ε row:

- **C11, C12 and C23:** unchanged. The gate's tag reaches no ending. C12
  is pinned at all three `PlaneDoor`s against the untagged margin.
- **C10, build (`contfp`), and the Boolean's `esc`:** the lever "move
  the point clear of the arc's end", now under a definite statement
  rather than "undecided: margin is invalid". The Boolean carries it as
  `PointInFaceRefused`.
- **C10 at rest (the census):** the band's own parting goes from
  `CensusEscalated`'s defect ending (by `is_invalid()`) to
  `CensusUnsupported` with the arc end's lever, pinned on public-operator
  geometry at `K = 1.25`. At the run's `K` the same point is the vertex
  reading's escalation, pinned too. An end off both corners goes from
  `CensusEscalated` (defect) to `ArcEndOffCorner` (defect, by type). The
  review's witness (a far arc, `K = 1.001` and `1.2`) reads the band's
  parting at every ε row, where the frame's end read it off its corner.
- **C10 at `reduce::wall_crossing`:** from a Containment escalation to
  `SpanVerdict::Unsettled` (the caller keeps its door, as for the face
  door's other non-escalation refusals), pinned.
- **C10 at `carrier_touch::ball_off_face`:** unreachable, before and
  after, by the clearance argument above.
- **C10 at `rim_wedge::contains`:** before, the `bool_contact_arc_end_vertex`
  `INVALID` escalation (`Escalation::Decided`). Now no verdict: the piece
  is `Cover::Unread`, and `departures` refuses the claim as outside its
  envelope (`Departure::EdgeKindUnread`, the claim's `unsupported`), never
  `NoEdge`'s decided contradiction. Pinned at `contains` (both arms) and
  through `departures` (the band's parting). The pre-existing
  boundary-read-as-interior conflation is filed:
  `seam-cover-reads-a-boundary-point-as-the-face-interior`.
- **C19:** `CensusEscalated` (defect) plus `EdgeEdgeCross { side:
  Some(Undecided) }` became `EdgeEdgeCross { side: None }`, the plain
  hard finding. The declaration is reported by the confirm pass
  (`ContactContradicted`) before and after.

Census passes over the five files, production code only (pass 1 / pass 2):

| file | before | after |
|---|---|---|
| `boolean/contain.rs` | 1 / 0 | 0 / 0 |
| `boolean/sectors.rs` | 1 / 1 | 0 / 0 |
| `boolean/plane_eq.rs` | 3 / 2 | 3 / 2 (`orientation_zero` for the D10-held caller; `unreadable_norm`, TOPO's row) |
| `census.rs` | 1 / 0 | 0 / 0 |
| `merge_faces.rs` | 5 / 3 | 4 / 2 (`merge_pair_extent` ×2, D10-held class d) |

## Closed (PR 4613, 2026-10-11)

Unit 4's mints route through the funnel's doors. C10's arc-end finding
is typed (`ArcEndClearOfCorners` in band, a defect finding
`ArcEndOffCorner` off both corners, with ends read through
`Curve3::eval` as certification pins them). C11's vtxfac margin is
carried, and recl's ±zero site is pinned and listed on the ratchet row.
C19's dihedral is the last screen, and C23 keeps its `Decided` whole.
`rim_wedge`'s arc-end touch now refuses as the seam cover's typed lane
limit rather than a decided contradiction; the pre-existing
boundary-as-interior reading is filed as
`seam-cover-reads-a-boundary-point-as-the-face-interior`.
