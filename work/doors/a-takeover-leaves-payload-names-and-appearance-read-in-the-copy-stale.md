---
id: a-takeover-leaves-payload-names-and-appearance-read-in-the-copy-stale
kind: issue
title: A takeover rebinds selections read in the copy, but a payload name or an appearance key read in it stays stale: Rebind has no body-scoped payload site
status: open
opened: 2026-11-01
priority: P3
cost: M
design: true
---

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). `Recording::take_over` (`crates/editor-core/src/refactor/take_over.rs`) writes `DocEdit::Rebind { body: Some(read), .. }` for each SELECTION that reads a taken-over copy, at any depth. Two other kinds of reference read a copy's names and stay stale:

- a payload name a node holds against the copy, e.g. a fillet whose `selection` names edges of the copy's read (`Node::payload_names`, `crates/editor-core/src/node.rs`);
- an appearance key on the copy (`AppearanceKey { at, name }`, `crates/editor-core/src/appearance.rs`).

`Rebind { body: None }` reaches both, but unsited: it rewrites every payload site and appearance key naming `from` in the document (`DocEdit::Rebind` arm of `apply`, `crates/editor-core/src/edit.rs`). The taken-over copy's old names are its source's names, which the result itself (a fillet of the source, say) also holds as payload, so an unsited rebind would rewrite the result's own payload too. The takeover therefore writes none.

**Decide when dispatched:** whether `Rebind` gains a sited payload form (the read the payload is read in, as `SitedRef` is), or the takeover leaves these stale with their diagnosis. The first changes a ratified edit's shape.
