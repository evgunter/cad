# Census gap 2 — the flush seat certifies (spec)

Orchestrator work order for **#1063** (#943's second half).
Substrate: the gap-2 exploration (2026-08-27; file:line evidence in
the lane report). Ratified ground: `docs/CENSUS-REST-CLOSURE-DESIGN.md`
option A (U-R2, "Q1 YES, sure") — **the conclusion is not
re-litigated; its JUSTIFICATION was corrected 2026-08-27 and this
spec implements the corrected one.**

Standalone kernel unit, carried by the PCURVE orchestrator. It is
NOT PCURVE work (census machinery, not edge descriptions) and shares
nothing with P-1a but the orchestrator and the block series.

## What is broken, measured

Seat a post under a shelf **flush with the shelf's end** — the
obvious way to draw it — declare `Node::Mate { class: Rest }`, run
the A5 gate. Gap 1 (CENSUS-REST #969) already made the census back
the subordinate boundary events with the mate's own `PatchContact`,
so the F1 hard error is gone. What remains: the seat measures
**Uncertified** with exactly one `CensusUnsupported{Face}` carrying
`ChartDivergence{detail: "no shared SurfaceKey and no GeomSource on
both faces"}`, in BOTH argument orders. Flush, inset and
line-contact seats are observationally identical today. The demo
seat is authored with an INSET, its comment saying flush ≡ inset
until this lands.

## The corrected justification this unit must honour

The world carrier is a **choice of representative frame**, not a
frame-free object: `Surface::Plane` carries `u_ref`, and
`world_carrier` returns `s_a`. What makes the choice legitimate is
**frame-invariance of the answer**, and "exact" is too strong
because `decide`'s `Ok(Zero)` is `|m| ≤ zero`, not bit-zero.

## The three binding conditions

1. **Door 2 consumes the verdict the census currently discards.**
   `census.rs:2154` throws away the `ContactVerdict`; Door 2 must
   read it, so the area certification knows what Door 1 proved
   (`Definite` vs `Bridged`) rather than re-deriving or assuming it.
2. **The residue is metered at the PAIR'S OWN CHART EXTENT**, not
   Door 1's pinned 1 m lever arm. A contact the size of a peg and a
   contact the size of a table must not share a lever.
3. **The frame-invariance lemma is WRITTEN, and argument-order
   symmetry is PINNED.** `overlap(A, B)` and `overlap(B, A)` must
   agree, and the doc/comment must state why the representative
   choice does not change the verdict. This is the unit's central
   proof obligation — the thing that makes the carrier honest rather
   than convenient.

## Scope

- **The interior-witness rung is NOT optional.** A shared trim edge
  makes the region walk refuse `TouchingBoundary`, so the world
  carrier alone does not close the flush seat. Whatever it
  certifies, it must still decline the cases it cannot witness —
  three-outcome honest.
- **One new metered predicate is expected**: `chart_region_carrier_tilt`
  (the substrate's name; keep it or justify a better one). It is a
  K row with k-lint implications — meter it, state its lever, and
  give it its three outcomes. **The existing branch adds ZERO
  `decide()` rows, which is precisely its defect, not an economy.**
- Cross-instance CURVED declared pairs stay refused (u_ref/seam
  divergence is real there) — unchanged, and the spec says so.

## `m9/census-xid` @ 890d3fb6: RE-DERIVE, harvesting two pieces

Do not build on it. It carries a real defect: `interior_witness`
builds `q` on **A's** plane and feeds it to `contfp` against **B**,
whose contract requires `q` already on the plane of `face` — the
precondition is never discharged and Door 1's trilean is never
consulted. It touches no `editor-core` test and no design doc, so
all three of U-R2's own deliverables are absent, and it silently
re-blesses both bogus-record probes by relocating their fixtures.

**Harvest**: `ChartRead::{Minted, WorldCarrier}` and the
`overlap_on` / `overlap_of_uv` split are good and worth keeping.

## Acceptance

1. The FLUSH seat certifies at the A5 gate — and the demo's inset
   retires to a genuine flush, which is the visible acceptance.
2. Argument-order symmetry pinned; the frame-invariance lemma
   written where the next reader will meet it (at the predicate,
   not only in a PR body).
3. The new metered row is ε-row three-outcome honest and its lever
   is the pair's own extent; k-lint green or its firing escalated,
   never silenced by moving geometry.
4. Line-contact and inset seats keep their honest outcomes —
   certifying flush must not certify everything.
5. Both bogus-record probes still refuse, on their own fixtures, not
   relocated ones.
6. Hosted CI green; the PR states which ε/compile-mode points it drew.

## Process

Implementer arm: **block PCURVE-1 slot 2 = OPUS** (block drawn
2026-08-27, byte 251, mod 4 = 3 ⇒ fable at slot 4). One block series
covers this orchestrator's dispatches across both tracks — the
balance the randomization buys is over the dispatcher, and a second
series would only create collisions with itself. Difficulty
pre-logged **M**, task-class **NUMERIC** (a new metered predicate
with a new lever is a numeric decision).

Review: **protocol v6 cross-model dual**, R1/R2 assignment
randomized by a `/dev/urandom` byte drawn AT REVIEW DISPATCH, byte
and assignment recorded in the row; ordinal claimed on main at
review dispatch; both briefs carry v6 item 5's lane-isolation READ
rule.

Standard brief lines: OUTPUT DISCIPLINE; the verbatim foreground
sentence AND its `setsid` exception; lane-private publish paths; no
`Co-Authored-By` in lane commits; comments state the invariant;
k-lint discipline; merge-main + BUILD THE UNION.
