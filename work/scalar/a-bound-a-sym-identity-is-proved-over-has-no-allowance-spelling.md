---
id: a-bound-a-sym-identity-is-proved-over-has-no-allowance-spelling
kind: issue
title: Two bounds are T because a Sym identity is proved over them, which DL5.2's allowance cannot carry
status: open
opened: 2026-11-20
---

Found building `a-bound-to-decide-against-never-becomes-a-t`. A point where DL5.2 (`crates/geom-core/README.md`, "a bound to decide against ... never becomes a `T`") does not say what to do.

## What

Two bounds are `T` because the symbolic tier proves an identity over the margin they enter, which an `f64` allowance cannot carry (an allowance enters `Sym`'s decision as an unknown, by design: `Decide::allowing` at `Sym`):

- `edge_nurbs::limb2_enclosure_as` (`crates/geom-brep/src/edge_nurbs.rs`) builds `T::from_computed(floor).enclosure_hull(T::from_computed(upper)).free_identity(identity)`: limb 2's margin is the bracket of two `f64` bounds, and at `Sym` its node is the residual identity over a free parameter (ERROR-DESIGN E12; `scripts/gates/free-identity-allowlist.sh`). Deciding the bracket at `f64` loses the identity, so a limb `Sym` proves zero today would read undecided.
- `editor-core/src/coincide/replay.rs` `swept_along` levers a component at `Lever::unsourced(T::from_f64(pow2))`, the power of two at or above a decided pair's reach (`from-f64-sites.tsv`, `constant`): a positive constant factor, so the tier proves the sine's identity through the product. `Margin::levered_at` would carry the arm as an allowance, and at `Sym` an allowance makes the margin an unknown, so the discharge would be lost.

## The question

Whether DL5.2's bound row admits a bound whose margin's identity is proved at `Sym`, and by which door. The allowance as built keeps `Sym`'s re-valuation instrument (`sym/revalue.rs`) honest: a margin node that dropped a positive scale would re-value to a different number than its own build. A positive constant scale could keep the reading's node (zero exactly where the reading is), at the cost of a re-valuation claim about it. Not decided by the unit that found it.
