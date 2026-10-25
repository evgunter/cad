---
id: a-mirrors-body-reads-as-in-no-space
kind: issue
title: Doc::space_of answers no space for a mirror's body, so a union whose first member is a mirror's image is in no space
status: open
opened: 2026-10-23
priority: P2
cost: E
---

Found by the `_ =>` sweep of `a-symmetric-parts-seam-is-not-walked-as-one-construction` (2026-10-23). `Node::Mirror` defines "a new `Body` in its source's space" (its doc in `crates/editor-core/src/node.rs`). But `SpaceWalk::node` (`crates/editor-core/src/doc.rs`, the walk behind `Doc::space_of` and `Doc::operand_space`) names no `Mirror` arm. A mirror falls to `_ => self.own.get(&id).copied()`, which serves a construction that defines a `space` port. A mirror defines only `body` (`Node::outputs`), so the answer is `None`.

A probe on the L plate of `crates/editor-core/tests/a_mirror_is_a_construction.rs` measured it. The plate is mirrored across its own `−x` face:

- `doc.space_of(plate's body)` is `Some(the plate's space)`;
- `doc.space_of(mirror's body)` is `None`;
- `Union { members: [image, plate] }` (the image first) has `space_of` `None`, since a union's space is its first member's.

The likely repair is one arm, `Node::Mirror { body, .. } => self.var(body.read)`, beside `Split`'s, with a test on the probe above. What a `None` space costs downstream has not been measured: the readers of `space_of` and `operand_space` (the insert door's space check, the placement and mate doors) each decide what an absent space means. Add that measurement to the repair.

## Measured through the viewer's mirror tool (2026-10-25, AUTHTAIL)

The viewer's mirror tool (`crates/viewer/src/combine.rs` `MirrorTool`, the session door `add_mirror` in `crates/viewer/src/session.rs`) authors `Mirror { body, plane: Plane { face } }` and places the image in the world. On a 40 × 20 × 10 mm block mirrored across its own `−x` wall, a probe measured what the absent space costs downstream:

- `doc.operand_space(image)` is `None`; the block's is its own space. The image's world placement (`Place` into the world at the stand bundle) is admitted, and the mirror's row is `Ok`.
- The symmetric part through the boolean tool, the block picked first: the geometry is right (volume `2 × 8e-6 m³`), but `in_one_space` (`crates/viewer/src/session.rs`) reads the image's `None` as "a body never placed" and inserts an extra `Place` of the image into the block's space at the identity. So the union's second member is a copy of the image where the image itself already reads the block's space. That is a superfluous node in every symmetric part authored this way, and it would place the image wrongly if the stand bundle and the space's axes ever disagreed.
- The image picked first: no extra placement, the right volume, and the union's `operand_space` is `None` (the union's space is its first member's).

`crates/viewer/tests/mirror_authoring.rs` `a_body_and_its_image_union_to_the_symmetric_part_in_either_order` asserts the volume and the world in both orders. It does not pin the extra placement. Once the `Mirror` arm is in the walk, the block-first case should insert no `Place`, and that row can assert it.
