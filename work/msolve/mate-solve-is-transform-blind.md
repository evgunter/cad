---
id: mate-solve-is-transform-blind
kind: issue
title: the mate solve is transform-blind: a Node::Transform between an instance and mated material silently yields a green document with a gap where contact was authored
status: open
opened: 2026-09-04
---


Measured by a FIX measurement lane at the FIX orchestrator's request,
while establishing whether it was safe to extend the mate-head member
vocabulary through identity-transparent nodes (PR 1731's ruling).
**It is a live defect, not a scope question**, and it is silent.
Filed here rather than on a program's slate so the owner claims it by
header edit.

**Claimant corrected 2026-09-04:** this was routed at S-MATE, which has
since CLOSED. `crates/editor-core/src/mate*` is now held by DOCM by inheritance of the FILES at
S-MATE's exit. Ev's steer (2026-09-04) is that the assembly-semantics
residue is a different question from DOCM's document-layer charter, so
it is re-homed to **MSOLVE**, opened as S-MATE's successor.

## The defect

`fold_pair` (`crates/editor-core/src/mate/solve.rs`) builds each mate's
coset from **authored alignment data** plus the pattern-derived static
offsets. It never reads the evaluated body.

`wire.rs:255` applies `env.poses.placement(doc, id)` at the
`InstantiatePart` node; `wire_transform` (`wire.rs:2284`) composes its
map onto that result **afterwards**. So a `Node::Transform` between an
instance and mate-named material moves the geometry and the solve never
learns of it.

`Transform` is invisible to naming by spec D2 — it contributes no
`RolePath` segment and `transform_rigid` is key-stable — so the mate's
name resolves cleanly through it, to a face that is not where the mate
says it is.

## Measured, through ordinary doors

Built with `DocEdit::InsertNode` only: two instances (`base`, a 1-tall
block; `top`, 3-tall), a `Node::Transform` over `top` translating +10 in
z with rotation angle exactly 0, and a `Rest` / `FrameCoincidence` /
`Opposed` mate seating `top`'s bottom cap on `base`'s top cap. Nothing
refused; every node evaluates `Ok`.

Control (no transform) against test (transform), same mate:

```
CONTROL solved relative(top) = Some(Frame { columns: [[-1,0,0],[0,1,0],[0,0,-1]],
                                            translation: [0.0, 0.0, 1.0] })
TEST    solved relative(top) = Some(Frame { columns: [[-1,0,0],[0,1,0],[0,0,-1]],
                                            translation: [0.0, 0.0, 1.0] })
```

**String-identical, asserted rather than eyeballed.** The mate is doing
real work — the `-1` diagonal is `Opposed`'s 180°, and the +1 lift seats
the cap — it just does it in the instance's own coordinates, and the
transform adds its +10 downstream.

## What a user gets

```
mate face A (base top)    z = 1
mate face B (top bottom)  z = 1 at the INSTANCE, 11 at the TRANSFORM
contact at the instance: true | contact in the product: false
```

`solve_document().fault(top)` is `None`. `product(..)` is `Ok`. **No
refusal, no diagnostic, no fault** — a green document with a 7-unit gap
where the author declared contact. That is the fail-loud posture
inverted: a decided path producing a definite wrong answer.

## The patterned form of the same shape is LOUD

```
PATTERN-OF-TRANSFORM: fault = Some(DanglingHead { mate: 4, side: B, head: 3 })
                      product = Err(RootFailed { node: 4 })
```

So `MateFault::DanglingHead` — which PR 1731 is ruling on whether to
retire — **is currently the only guard on the silent half's patterned
twin.** Extending the member vocabulary through identity-transparent
nodes without also composing the transform's map into the solve would
convert that refusal into a second silent wrong answer. That is why
this issue gates that work.

## The class, and the real coupling

Swept for *applies a rigid map to the body while contributing no
`RolePath` segment*, over `crates/editor-core/src/`:

| site | map | name contribution | status |
|---|---|---|---|
| `wire_transform` (2284) | rigid | **none** | **blind** |
| `wire_pattern` (2428) | rigid | `Instance(i)` | not blind — `derived_offset` re-derives the same map |
| `wire_placed_union` (2510) | rigid | `Instance(i)` | outside the vocabulary, refuses |

`Transform` is the unique production node with that combination. The
class statement:

> The solve compensates for exactly one geometry-moving node
> (`Pattern`, via `derived_offset`) and is blind to the other
> (`Transform`) — and the discriminator that currently hides the
> blindness is **name-table segmentation**, which is not the same
> property as **pose-relevance**. `member_of` decides admission on
> naming transparency; correctness needs pose transparency. They
> coincide today by accident.

So the constructive fix is a `derived_offset` sibling that walks the
input chain and composes **every** pose-bearing node's map, not just the
pattern's.

## What the measurement could not see

- **Rotation untested.** The transform is translation-only with angle
  exactly 0, chosen so the arithmetic is exact. A rotating transform
  would also mis-*orient* and would break the `Opposed` axis agreement;
  only displacement was measured.
- **One mate class, one primitive** (`Rest`/`FrameCoincidence`/`Opposed`).
  A `Prismatic` mate along +z might absorb the translation in its free
  direction, which would make the bug **class-dependent and
  intermittent**. Not measured; worth doing first.
- **Non-gauge side only.** `base` is the cluster gauge; the transform
  wrapped `top`. Untested: a transform over the gauge, over both sides,
  or a chain of two.
- The sweep greps `transform_rigid(`, so a mover using another means, or
  living outside `crates/editor-core/src/`, would be missed.
- The GUI/API layer above `editor-core` was not tested; some authoring
  door may refuse this shape before the kernel sees it. Reachability is
  established **at the kernel's public doors**.

## Home

`work/msolve/` — S-MATE's successor, opened 2026-09-04 for exactly this
residue.
