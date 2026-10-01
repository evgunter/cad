---
id: committed-nodes-do-not-light-the-faces-they-reference
kind: issue
title: Selecting a committed Declare, Mate, face-frame datum or Measure node lights nothing of the faces it references
status: open
opened: 2026-09-30
priority: P3
cost: M
---


Found by a designer on the face-naming fork (#3571).

A `Declare`, a `Mate`, a `Datum::FaceFrame` and a `Measure` each reference faces by `StableName` (`Node::payload_names`). Once committed, no surface lights those faces.
- `marks::focus` lights what a selected node BUILDS, not what it references. For a face-frame datum that is the bodies built on the datum, not the face it sits on.
- Their tree rows name no face either (`face-pick-cannot-name-which-face`).

So an author cannot see which two faces a committed declaration or mate joins.

This is the picture half of the face-naming answer. It needs no ruling: whatever #3571 decides about words, selecting such a node should light its referenced faces. The designer proposed reading them from `Node::payload_names`. The open part is the mark: a referenced face is neither selected nor held, and `marks::Held` has fixed slots.
