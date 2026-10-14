---
id: a-flush-pair-with-no-readable-extent-has-no-typed-finding
kind: issue
title: A flush pair with no readable extent refuses through a labelled Indeterminate, not a typed finding
status: closed
closed: 2026-10-11
opened: 2026-10-02
priority: P2
cost: E
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

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: flush::pair_finding feeds the flush seat, which offers declarations; the detector retires or becomes the unproven-coincidence lint at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Evidence (ENCL, PR 4629, 2026-10-11)

`FlushRefusal::PairInBand` ends this arm through `Indeterminate::ending`, so since PR 4629 an `EXTENT_UNREAD` pair reads `Recourse: check the operation's inputs upstream, then separate the geometry; an unreadable margin may indicate a kernel bug worth reporting`: an input lever on a kernel limit, where its sibling `PairUnreadable` ends in `KERNEL_OR_FILE_DEFECT_ENDING`. The typed outcome this row asks for retires that ending too.

## Closed (2026-10-11, stage 4 F)

`flush::pair_finding` and the editor's flush seat are deleted with the detector; nothing reports a finding any more.
