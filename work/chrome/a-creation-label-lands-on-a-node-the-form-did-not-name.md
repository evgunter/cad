---
id: a-creation-label-lands-on-a-node-the-form-did-not-name
kind: issue
title: a creation form's label lands on a node the form did not name
status: open
opened: 2026-10-31
priority: P2
---

## Finding

Found driving the viewer (2026-10-31; Ev: file it). A creation form's label field names one node and the label lands on another:

- **Profile on the picked face.** With **on the picked face** ticked, the label typed in the profile form (`Profile 2`) becomes the Union's label, and the new profile has none.
- **Mirror.** The mirror form's label reads `Mirror 1` until a face is picked, then `Union 1`. The committed Union gets it and the Mirror node is unlabelled (`Mirror aa8bedaa77d7` in the tree).
- **Pattern.** The pattern form's label defaults to `Place 2`: it counts the Place node the op mints, not the pattern a person asked for.

In each, a form that mints more than one node labels the last, and the default counts by that node's kind (`ViewerBehavior::tool_noun`, `push_labelled`, `crates/viewer/src/pane/create.rs`). Related: `work/wiretail/the-seated-tools-label-nouns-are-not-pinned-to-the-nodes-they-mint.md` (P4, pins the nouns to the minted nodes; this row is about which node gets the label).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
# the mirror
click "Combine bodies"
click "Mirror tool…"
vclick 0.3 0.62
settle
dump
click "Commit mirror"
settle
tree
```
