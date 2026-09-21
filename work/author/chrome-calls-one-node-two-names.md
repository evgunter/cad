---
id: chrome-calls-one-node-two-names
kind: issue
title: The chrome calls a node "feature N" in widgets and "node N" in refusals, including two widgets in one file
status: open
opened: 2026-09-21
priority: P3
cost: D
refs: [add-profile-mints-no-frame]
---

## What

AUTH-3 gave the phrase `feature 3` one home, `tree::node_number`, and
routed every widget that already said it. The sweep that found those
sites was literal (`format!("feature {}", …)`); the second pass, at the
blind spot that pattern names, found the larger class the first one
could not see: **the same thing has two names.**

`feature N` is what a widget says. `node N` is what a refusal says —
and what two widgets say too:

- `crates/viewer/src/pane/create.rs`, the mate tool's held picks:
  `"pick a: face of node {}"` and
  `"pick a: node {}; pick b: node {}"`. Widget text, twenty lines from
  a `frame_picker` that says `feature`.
- `crates/viewer/src/seats.rs`, the seat drop notice: *"the {} pick
  (node {}) is no longer in the document"* — beside `seat_line`, which
  AUTH-3 routed through `node_number` and which says `feature`.

And in refusal and report prose, where nothing is a widget:
`session/refuse.rs` (the `NoSuchSlot`, `WrongNodeKind`,
`ProfileEditStale`, `ProfileEditOrder`, `ProfileEditOrderCapped` and
`FaceFrameFault` arms), `display.rs`, `pickindex.rs`, `props.rs`,
`matetool.rs`, `sketch.rs`'s `NotAProfile`.

## Why it is filed and not fixed

AUTH-3 could route the sites that already said `feature` without
touching what anybody reads: same words, one home. Every site above
would CHANGE the words, and the direction is a decision, not a
cleanup:

- `feature` everywhere reads well in chrome and badly in a kernel-side
  refusal, where the subject really is a recipe node and the reader
  may be looking at a file rather than a panel.
- `node` everywhere loses the word the feature tree is named after.
- Split by surface — `feature` in chrome, `node` in typed refusal
  prose — is defensible and is roughly what the tree does by accident
  today, but then the two mate-tool widget lines and the seat notice
  above are on the wrong side of the split and `node_number` is the
  chrome half's home rather than the only home.

Whichever it is, the refusal prose is `Display` impls with no access
to anything but the id, so a home for them is a second function, not
this one.

## What AUTH-3 did leave true

`tree::node_number` is the one spelling of `feature N`, and the six
sites that say it call it: `tree::node_label`, `Display for
BlendTarget`, `pane/properties.rs`'s entity heading,
`pane/create.rs`'s extrude button, `seats.rs`'s `seat_line` and
`session/delete.rs`'s confirmation label.
