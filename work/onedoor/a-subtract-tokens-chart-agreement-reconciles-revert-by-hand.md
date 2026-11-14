---
id: a-subtract-tokens-chart-agreement-reconciles-revert-by-hand
kind: issue
title: the Boolean's one-record token reconciles revert's plane convention by hand when it reads how two charts face under subtract
status: dispatched
opened: 2026-11-11
priority: P1
cost: M
design: true
---


Filed by the onedoor fix-3 lane on PR 6505 (rv3 review S5).

## What

`crates/topo/src/boolean/ops.rs`, `OneRecordTokens::of_result`, reads
whether a glued pair's two charts face one way through the one
spelling `one_record::charts_agree`, but it feeds B's sense in as
`fb.sense != turned`, where `turned` is computed by hand: under ∖ a
reverted plane turns its chart where every other kind turns its faces'
senses. That convention belongs to revert (`transform::OrientationClass`)
and is restated here from outside it; a change to how revert treats a
kind would leave this site silently wrong.

## What closing it takes

Read the senses where revert has already applied its convention: the
moved and kept faces' senses in the result, on the records they wear
there, so the token needs no knowledge of revert. That needs a live
result face on each side of every pair (a fully covered decided face
has none), or a revert-owned door that says how a reverted record's
chart faces against its source's.

## Spec (ONEDOOR, 2026-11-14)

Weighed by two designers, who converged. Revert has no convention of its own to hand out. `Body::reverted` borrows the reflection's `OrientationClass` through a local `is_plane` closure (`revert.rs:196-199`, applied at `:352-353`), and the boolean token rebuilds the same split with `== OrientationClass::Plane` (`boolean/ops.rs:5614-5620`). So it is written twice, out of someone else's enum. Reading the senses off the result was rejected: a fully covered decided face has no live result face, and the void fallback (`ops.rs:7786`) builds from pristine operands.

- **One door, which revert writes through.** In `revert.rs`:
  - a private `Reversal { Chart, Sense }`, built by an exhaustive `match` on `Surface` variants (not on `OrientationClass`), so that a new surface kind does not compile until revert decides how it reverses;
  - one `pub(crate)` door, `reverted_sense(surface, sense) -> bool`.

  `Body::reverted` dispatches every split it makes today through `Reversal` (plane faces, mirrored chart images and pcurve rows, normal negation), and its face-sense loop calls `reverted_sense`. The `is_plane` closure retires.
- **The token asks the door.** `OneRecordTokens::of_result` drops `turned`. B's sense under ∖ is `reverted_sense(b_surface, fb.sense)`. Materials agree is the glue relation flipped under ∖. Its doc at `:5570-5575` follows.
- **The borrowed spelling cannot return.** `OrientationClass`'s doc narrows to the reflection. It becomes private to `transform.rs` if nothing else then reads it; otherwise a source guard refuses `OrientationClass` outside `transform.rs`.
- **Tests:**
  - for a plane, a cylinder and a NURBS face, `reverted()`'s face sense equals `reverted_sense` of the source;
  - a ∖ coplanar-glue row on a curved glued pair, if none exercises the non-plane arm today;
  - the existing ∖ rows stay green.

**Residue, not folded in:** "∖ complements B" stays spelled `op == Subtract` at `finish.rs:289`, `holdings.rs:311` and `join.rs:1095`. That is the Boolean's own fact, kind-independent, and filed separately. Also found: `of_result`'s doc says a pair whose record no result face wears licenses nothing, but the code checks only `result.get_surface(..).is_some()` (`ops.rs:5601-5604`). The lane fixes whichever side is wrong and says which.

Letter M, dual tier (sequential): it moves revert's convention, which the reflection, the Boolean and every revert caller read.
