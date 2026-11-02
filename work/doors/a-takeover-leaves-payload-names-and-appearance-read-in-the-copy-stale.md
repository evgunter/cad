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

Found by the takeover unit (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`). `Recording::take_over` (`crates/editor-core/src/take_over.rs`) writes `DocEdit::Rebind { body: Some(read), .. }` for each SELECTION that reads a taken-over copy, at any depth; a fillet's or a mate's selection is one of those (a `Select` variable), and is rebound. Two other kinds of reference read a copy's names and stay stale:

- a payload name read against the copy: today the only payload names are an instance's interface record's crossing `outer`s (`Node::payload_names`, `crates/editor-core/src/node.rs`), so a record whose `outer` names a face of a taken-over copy;
- an appearance key on the copy (`AppearanceKey { at, name }`, `crates/editor-core/src/appearance.rs`).

`Rebind { body: None }` reaches both, but unsited: it rewrites every payload site and every appearance key naming `from` in the document, whatever read it is keyed at (`DocEdit::Rebind` arm of `apply`, `crates/editor-core/src/edit.rs`). The copy's old names are its source's names, so an unsited rebind would also move a key the source body's own read holds under that name (a colour on the source's face) onto the result's name, where the source does not hold it. The takeover therefore writes none.

**Decide when dispatched:** whether `Rebind` gains a sited form for payload and store sites (the read the name is read in, as `SitedRef` is), or the takeover leaves these stale with their diagnosis. The first changes a ratified edit's shape.
