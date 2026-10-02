---
id: a-flush-pair-with-no-readable-extent-has-no-typed-finding
kind: issue
title: A flush pair with no readable extent refuses through a labelled Indeterminate, not a typed finding
status: open
opened: 2026-10-02
---

## What

`flush::pair_finding` (`crates/topo/src/flush.rs`, the
`PairUnread::Extent` arm) reports a pair one of whose faces has no
readable consumed extent as an `Indeterminate` with an invalid margin
and the label `flush::EXTENT_UNREAD` (`"carrier_pair_extent"`). No
`decide` ran there, so the refusal is not a margin at all: a caller
reading it as an escalation (the editor's flush seat,
`crates/editor-core/src/names/flush.rs`, and the Python tags above it)
sees an undecidable reading where the truth is an input the door could
not take, and which face it was is dropped.

## Shape of a fix

A typed outcome beside the `Indeterminate`, naming the face
(`PairFace`) the extent did not read on, carried through the editor
seat's refusal and the Python tag table. The ripple into the editor
seat and Python is why TANG's lever PR stopped at the label.

