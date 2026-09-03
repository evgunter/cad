---
id: pick-target-mesh-pairing-unverifiable
kind: issue
title: Pick provenance - raw PickTarget (node, body) to mesh pairing is unverifiable, GUI-2's cache design must confront it
status: open
opened: 2026-08-27
github: 1098
refs: [1093]
---

## From GitHub issue 1098

opened 2026-08-27, 0 comments.

Class found by both PR #1093 reviews (R2 MINOR-3; R1 style Q7/Q4), same family as the `MeshPatchKey` convention.

**The lane.** Arena keys collide numerically across sibling nodes, so a `PickTarget` whose `(node, body)` is not the pair its `MeshPick`'s mesh was tessellated from makes `pick_face` invert the hit face's key against the WRONG node's table — a plausible, confidently wrong `StableName`, not an error. Verified by the standing (ignored) witness row `editor-core/tests/gui1_pick_r2.rs::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`.

**What the fix pass shipped.** `editor_core::NodePick` — the provenance-atomic door: fetches the body from the evaluation payload itself via `product::sources_of` (the one enumeration the name tables key by), tessellates and indexes in one call, hands back the mesh for display; private fields, no other constructor, so the pairing cannot be mis-asserted. `PickTarget` carries a loud contract naming the failure mode.

**The residual class this issue tracks.** Raw `PickTarget` assembly remains verification-free by construction — the keys carry no node identity to check against. In particular:
- GUI-2's pick cache keyed by `(Evaluation::epoch, node, body)` will make exactly this mistake if the key drifts by one field while holding a raw `MeshPick`. The cache should hold `NodePick` (or an equivalent pairing-carrying value), not bare `MeshPick`s — make that a design decision in GUI-2's spec, not a discovery.
- If a verification story is ever wanted for raw targets (content hash of the mesh against the payload body, or arena keys gaining node identity), sweep BOTH convention sites: `PickTarget` and `resolve::MeshPatchKey` (the tombstone/ghost-render payload has the same unchecked pairing).

The witness row stays `#[ignore]`d as documentation of the class; it should be revisited (gate, or delete) when GUI-2's cache lands.

## Home

Viewer/GUI ground straddling `editor-core/src/resolve` — the GUI v1 program is closed and may hold only closed items, so it lands under `work/issues/`.
