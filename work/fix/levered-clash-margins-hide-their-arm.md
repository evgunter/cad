---
id: levered-clash-margins-hide-their-arm
kind: issue
title: Three levered mate-fold clash margins reach the refusal with their arm invisible
status: open
opened: 2026-09-04
---


Residue disclosed by `mate-contradiction-names-one-mate-twice`, which
gave `MateFault::Contradictory` a `lever: Option<ClashLever>` and filled
it at the one raising site that has the lever in hand
(`crates/editor-core/src/mate/solve.rs:544`, the clocking rider). The
socket is in place; three sibling margins still arrive with `None` and
so still print a levered product as a bare metre figure the reader
cannot re-derive:

- `crates/editor-core/src/mate/coset.rs:582` — `mate_member_axis_fixed`,
  margin `(x.linear * axis - axis).norm() * arm`.
- `crates/editor-core/src/mate/coset.rs:643` — `rotation_residual`,
  feeding `mate_member_rotation_identity`, margin `‖Q − I‖_F · arm`.
- `crates/editor-core/src/mate/coset.rs:743` —
  `mate_rotation_two_axis_reachable`, margin `reach * arm`.

All three reach `MateFault::Contradictory.clash` through
`FoldStop::Clash { predicate, margin }`
(`crates/editor-core/src/mate/solve.rs:690`). Filling the lever there
means widening `member_of`'s `Err((name, margin))` and the `checks`
vec to carry the disagreement and the arm beside the product, plus the
same for `candidate_rotation` — solve-internal plumbing in S-MATE's
live territory (`crates/editor-core/src/mate/*`), not the
refusal-display prose S-MATE's `keep_out` cedes. It was left out of the
display unit on that fence and needs S-MATE's assent or a re-home.

The five remaining `member_of` predicates
(`mate_member_translation_zero` / `_along` / `_in_plane`,
`mate_member_point_on_axis`, `mate_member_point_fixed`) measure lengths
outright and correctly carry no lever; two rows in
`crates/editor-core/tests/asm_r2a_mate_solve.rs` pin that.

The tree's own precedent for the shape is
`crates/profile/src/path.rs:1201`, `PathError::JunctionCusp`, which
renders "turn margin {margin} m on a {arm} m arm".
