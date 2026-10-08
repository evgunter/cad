---
id: sentence-labels-is-a-second-meaning-of-label
kind: issue
title: sentence::Labels and Labelled, the McReport's advisory label and a check's honesty label are other meanings of 'label'
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [node-labels-are-document-data]
---

`editor_core::sentence::Labels` and `Labelled` (`crates/editor-core/src/sentence.rs`) name whether a refusal is rendered with its STAGE words (`product:`, `persist:`) — "labels" in the sense of a sentence's prefixes. Since PR 3713, `Label` is a node's (and a face's) human text, and the node-label row asked that "label" mean one thing (it renamed Python's `Doc(label=...)` identity seed to `seed` for that reason). These two types are one other meaning; the Python surface carries two more in prose, both in `crates/pncad-py/pncad.pyi`: the McReport's "advisory label" (`McReport`'s `__str__` docstring, ~line 3056, and the ADVISORY paragraph after it) and a check's "honesty label" (`Certainty`, ~line 6288, and `Check.certainty`'s docstring). A rename of the types (`Prefixes`/`Prefixed`, or `Staging`) is mechanical across the 13 call sites that name them; the two prose uses want another word ("advisory tag", "honesty grade").
