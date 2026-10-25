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
