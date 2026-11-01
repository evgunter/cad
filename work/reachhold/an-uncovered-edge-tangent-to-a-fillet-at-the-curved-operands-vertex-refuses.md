---
id: an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses
kind: issue
title: An edge tangent to a curved face at a point touch no declaration can cover refuses in both operand orders
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Found while building `a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`,
measured on `origin/main` at `cd49025f` and on that unit's branch.

## Repro

A concave L, `(0,0) (6,0) (6,2) (3,2) (3,4) (0,4)`, unit thick at
z 0 to 1, every corner rounded r = 1 through the PATHS fillet door
(the pattern of `rounded` in `crates/sweep/tests/reach_continuation.rs`),
with its sharp twin stacked on it at z 1 to 2, every flush finding
declared. The east side is 2 long, so its two r = 1 fillets meet at
`(6, 1)` on one cylinder (centre `(5, 1)`) and leave no flat east wall.

- sharp L as A: `CurvedPierceUnsupported { operand: A, face: 4v1, edge: 2v1 }`;
- rounded L as A: `CurvedPierceUnsupported { operand: B, face: 4v1, edge: 2v1 }`.

## Why (measured)

The sharp L's east wall `x = 6` touches the fillet cylinder along the
ruling `x = 6, y = 1`, but the wall spans z 1 to 2 and the cylinder
z 0 to 1, so the faces meet at the one point `(6, 1, 1)`. No declaration
can back that: a `Tangent` needs a curve touch and the wall has no
continuation partner, so the crossing layer's one-sided cover has no
source (`crates/topo/README.md`, the cover clause). Both orders split
the sharp edge at the rounded L's vertex `(6, 1, 1)` (the joint between
the two fillets), so the touch is at an endpoint; the uncovered
mixed-sign arm of `reduce::curved_face_arm` then asks `wall_crossing`,
whose line × wall roots answer `Tangent`, and the pair keeps the
frontier.

A fix needs the touch read from structure rather than the band: the
touch point is a vertex of the curved operand, already carried as a
v-v record. Whether a vertex-coincident tangency may stand as its own
cover is a design question for the cover clause.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: asks which declarations can cover a touch; stage 4 retires declared Tangent and the undeclared-tangency refusals. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E changes where a cover can come from. The glue door declares a plane × cylinder pair `Tangent` itself wherever the witness lane verifies it, and it reads only the two faces' boxes (`crates/topo/src/boolean/glue.rs:72`–`:83`, `:98`). The wall × fillet pair here may now be covered without a declaration. Not measured: the repro has no fixture in the tree. If the witness lane verifies the ruling though the faces share one point, the union may build or move to another refusal. If not, the design question for the cover clause stands as written.
