---
id: viewer-panels-disagree-on-a-poisoned-node
kind: issue
title: The tree re-attributes a cluster-refused node; the properties panel and appearance still do not
status: open
opened: 2026-09-04
refs: [1769, 1463]
---

Found by CHROME's style lane on PR 1769, as a class rather than an
instance.

PR 1769 makes the feature tree draw a cluster-refused instance as
downstream of the mate that caused it. **The viewer's other renderings
of node status were not changed**, so two panels of one window now
disagree about the same node:

- the tree draws it `Poisoned`, weak, pointing at the mate — and
  since the same PR's fix pass, its line under the row is that
  POINTER, so the two panels now disagree in words as well as in
  status: the tree says "upstream failure at node N", the properties
  panel recites the refusal;
- the properties panel reads the kernel arm verbatim through
  `Resolution` and renders `TargetFailed` in `theme.unresolved`
  (`crates/viewer/src/app.rs:3086-3092`,
  `crates/editor-core/src/resolve/mod.rs:815-825`);
- `appearance::AppearanceLossCause` carries the same
  `TargetFailed`/`TargetPoisoned` split
  (`crates/editor-core/src/appearance.rs:396-400`).

The re-attribution was applied at exactly one of several sites that
answer "what is wrong with this node", which is the shape that produces
a user reading two panels and believing the tool contradicts itself.

**Not a defect in 1769's own change** — its tree rendering is right,
and widening to the other surfaces is a behaviour change in code that
unit did not set out to touch, some of it in `editor-core`. Filed so
the class has a home rather than being discovered by the third panel.

Where to look when taking it: the properties panel's resolution
rendering, `appearance`'s loss causes, and `display::DisplayFault`.

Signed: (CHROME orchestrator)
