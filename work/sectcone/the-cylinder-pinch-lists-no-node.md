---
id: the-cylinder-pinch-lists-no-node
kind: issue
title: The cylinder pair's inner pinch lists two lobes and no node, where C5 lists a Zero tangency with carriers as a node
status: open
opened: 2026-11-22
---


Found in PR 7357's review, while making a cone pair's crossing tangency a node.

`pair_section/cylinders.rs`, `cylinder_cylinder`, the `CcNode::Pinch` arm (~985): the thinner wall touching the fatter from inside lists two `Open` components `Unminted(NoCarrier::Pinch)`, lists no `SectionNode`, and its comment says "the pinch's node is not listed while its lobes mint no carrier". C5's node clause (`crates/geom-brep/README.md`, "A node is decided on the pose (a Zero tangency whose section still has carriers …)", and "Arcs that share nodes are one node-joined set") would list the pinch point as a node, and the two lobes as arcs ending at it in one `NodeSet`.

So there are three spellings of one shape:
- the cylinder pinch: two lobes, no node;
- the cylinder × sphere figure-eight (`pair_section.rs` `cylinder_sphere`, the `(Positive, Some(Zero))` arm): one `Loop`, `Unminted(Tangent)`, no node;
- the cone pair's crossing tangency (PR 7357): a node, with its arcs in one set.

Whether the cylinder pinch should follow C5 is the owner's call. The lobes have no carrier, and the march to a node is not built. The value's two other readers of this shape now differ from it.
