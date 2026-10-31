---
id: the-viewer-cannot-author-a-mirror
kind: issue
title: The viewer offers no gesture that writes Node::Mirror, so a symmetric part cannot be authored through the UI
status: closed
opened: 2026-10-22
priority: P0
cost: M
refs: [5607]
closed: 2026-10-25
pr: 5683
branch: authtail/viewer-mirror
---


Found by `a-mirror-is-a-construction` (2026-10-22). `Node::Mirror { body, plane }` evaluates, saves and is bound in Python (`Node.mirror`), but no viewer form writes one: the viewer's sites match it only to classify it (`crates/viewer/src/session/refuse.rs` `seat_kind`, `crates/viewer/src/session.rs` `puts_an_instance`, `crates/viewer/src/tree.rs` `node_note`). The gesture is the split's: pick a body and a plane read off its own geometry (`crates/viewer/src/pane/create.rs`'s split form, `Node::Split`). A symmetric part is then `Union` over the body and its mirror.

## Closed

2026-10-25, PR 5683. The mirror tool (`ToolKind::Mirror`) holds a body seat and a planar face held in the tool's own state. It writes `Mirror { body, plane: Plane { face } }` with the plane read off the face; the typed-plane path is not used. On a body the world draws, the gesture is a takeover: it writes `Union[B, m]` in B's space and re-points B's drawn copies onto it (`repoint`), so the symmetric part stands where each copy stands and moves with it.
- **Decided by** the mirror arrival fork (designer pair, converged; `log.md` 2026-10-25), after two review rounds rejected an identity placement and a copied bundle.
- **Carried:** INTENT's `a-mirrors-body-reads-as-in-no-space` (the `SpaceWalk` arm), and the split-half port fix in the mirror door.
- **Residue, each its own row:**
  - `the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3`;
  - `a-face-on-a-splits-second-half-refuses-sketch-and-mate`;
  - `a-duplicate-of-a-moved-copy-lands-at-the-identity`;
  - `a-mate-on-a-taken-over-copys-face-goes-stale`;
  - `repoint-matches-a-shape-by-its-operation-not-its-read`.
- **Review tier:** full single review, then two re-verifies of the fix passes.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26). Kept although
closed: `the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3`
is the residue of this row's mirror arrival fork, and its body points
at the `## Closed` section above for the decision it builds on.
