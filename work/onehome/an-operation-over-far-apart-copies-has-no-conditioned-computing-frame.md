---
id: an-operation-over-far-apart-copies-has-no-conditioned-computing-frame
kind: issue
title: An operation over copies more than the session box apart has no conditioned computing frame, and refuses copies each inside the box
status: open
opened: 2026-11-19
priority: P2
cost: M
design: true
---



An operation over copies computes in its first operand's frame: the first operand in its space's coordinates, else the first (`Framed::computing_among`, `crates/editor-core/src/eval/framed.rs`). That covers the gathers (the `⊔` in `wire_assemble`, `distribute::assembled`, `product::gather_sources`), the Boolean fold (`fold_members`) and the subtract (`wire_subtract`). Each operand is read into that frame and asked the session box there: `Framed::graft_into` → `topo::boolean::graft_placed` for a gather, and `Held::moved` for the fold and the cut. Two copies that each stand inside the box can stand more than the box apart, and then the second is past the box in the first's frame. The operation refuses a document every one of whose bodies stands inside the box.

## Measured

A unit block, two copies at `±x` along the world `x` axis, at `f64`. On PR 7099 under main's rule (the first operand's frame):

| `±x` | `⊔` | `∪` | `⊖` |
|---|---|---|---|
| 400 | builds | builds | builds |
| 499 | builds | builds | builds |
| 501 | refuses, margin −2 | refuses, margin −2 | refuses, margin −2 |
| 600 | refuses, margin −200 | refuses, margin −200 | refuses, margin −200 |
| 900 | refuses, margin −800 | refuses, margin −800 | refuses, margin −800 |

Every refusal is `NotFinished[OutsideSessionBox]` at the operation. The far corner reads at `2x + 1` in the first copy's frame, so the threshold is a separation of 999 m. This is main's behaviour.

Pinned as over-refusals in `crates/editor-core/tests/product_finishes_once.rs`:
- `copies_either_side_of_the_origin_refuse_to_{assemble,unite,subtract}`;
- `a_product_over_copies_past_the_box_apart_refuses_as_its_aggregate` (the product reading the copies refuses as `ProductInvalid`, with the box's finding).

Under PR 7099's interim rule at `439110eb7` (the nearest frame the operands share, else the space's coordinates, for the gathers only), the `⊔` at ±900 built (reach 901 m). A `∪` refused, margin −800 (measured with a scratch probe). A `⊖` refused too: the subtract kept `computing_among` (by the code).

## Why each frame tried fails

- **The space's coordinates.** `[placement.frame]` (`docs/DESIGN.md`): an operation computes in a frame "chosen so its arithmetic is well conditioned near the geometry it builds, and never in the world's". A top-level document's space is the world, so this is the frame the text rules out. That is the interim rule above.
- **The first operand's rotation, its origin at the operands' centre (R-1).** Built at PR 7099's `1efd4c4df` (`Framed::computing_frame`): the session box ask fired, and the frame was recentred at the midpoint of the hull of the operands' world vertices. The box is a cube on the world axes, of half-width `B`. A body inside it can lie `√3·B` from a point, and in a rotated frame the coordinates of a box-valid document reach up to `√3·B`.
  - Measured at `1efd4c4df` (`copies_past_the_box_about_the_recentred_origin_refuse_at_the_graft`): two copies turned an eighth about `z`, at `(900, 900, 0)` and `(−900, −900, 0)`, each inside the box. They stand about 1273 m from the centre along the kept rotation's first axis, and the `⊔` refused at its graft.
  - The general rotation reaches about 1559 m.
- **The trigger reads values (R-2).** That rule decided whether to recentre by asking the box, and placed the centre by a midpoint. `framed.rs`'s computing-frame choice reads the frames' structure and no value, so the `f64` witness and every lane over a box decide in one frame.
  - A value test near its threshold can pick different frames on different lanes. The drive's lanes then disagree: a flip refusal, not a false certificate, but a refusal.
  - The vertex-only hull also misses a curved face's bulge past its vertices.

## Candidate directions

- **Ask the box where each image stands, not in the computing frame** (PR 7099's earlier option (A)). The box's question is about the world, so ask it there: thread the image's world motion into the session box predicate. That predicate is tier 3's and 3′'s first (`validate.rs`, the three `in_session_box` asks), reached through a finish entry on all six scalars' `AtRestPolicy::finish`. The computing frame stays the first operand's, so the arithmetic still runs at up to twice the box.
- **A translated frame with the space's axes.** Recentring alone keeps each coordinate within `B` of a per-axis midpoint, so containment holds. But it keeps the world's axes, which may be what "never in the world's" forbids, and choosing the centre still reads values (R-2).
- **A box not tied to the world axes:** for example, a ball, or a box read in the computing frame's axes. That changes D4's session box and its `[tolerance.units]` cube.

This goes to designers, then to Ev. It does not block PR 7099 (stage 1 of the operand-coordinates work), which ships main's frame rule with this over-refusal pinned.
