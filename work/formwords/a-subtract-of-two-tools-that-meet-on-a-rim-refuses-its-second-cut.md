---
id: a-subtract-of-two-tools-that-meet-on-a-rim-refuses-its-second-cut
kind: issue
title: A subtract of two tools that meet on a rim refuses its second cut
status: open
opened: 2026-11-23
priority: P3
cost: M
---

Found merging main's rim junction (`GivenDecision::Junction`, from
HELDRECORDS' elbow lane) into PR 7426 (a subtract's tool is a `Bodies`
seat).

## Read

An n-ary subtract judges its tools pairwise as their union, for their
verdicts alone (`topo::NaryOp::Subtract`). The fold then gives each step
those verdicts, read through the accumulation's faces. A tool's faces
stand turned in the accumulation (`NaryOp::holds_turned`), so a verdict
read through one has its senses reversed (`GivenDecision::reversed`,
`crates/topo/src/boolean/mod.rs:1227`).

A rim junction (`GivenDecision::Junction`, `crates/topo/src/boolean/mod.rs:1215`)
is two faces that end on one circle with their outward normals aligned.
Turning one face breaks that, so a junction has no reversed reading:
`reversed` answers `None`. `given_pairs`
(`crates/topo/src/boolean/nary.rs:1140`) then refuses that step as
`NaryRefusal::Step { error: TangencyUnsupported }`, which editor-core
reports as `NodeErrorKind::FoldStep` naming the member. So a kept body
cut by two tools that meet on a rim junction would refuse its second
cut, where the chain of pair cuts `(a − t₁) − t₂` decides that rim in
the second cut's own door.

Today the tools' judgement stops first, where their union stops too:
past the crossing layer (PR 7463 reads an edge ending on the rim), at
the finish on the seams' stand-in descriptions
(`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`).
`crates/editor-core/tests/a_subtract_of_tools_meeting_on_a_rim.rs`
pins the cut's refusal at either place. The kernel row
`a_subtract_reads_its_tools_turned_and_a_junction_has_no_turned_reading`
in `nary.rs` pins the guard.

## Owed

Some decision a step can be given for a junction met through a turned
face, or a way for the subtract's step to decide that one pair in its
own door. A fold step decides no carrier pair of its own by contract, so
the second way is a change to that contract.

The owner of `GivenDecision::Junction`, HELDRECORDS, would decide what
a reversed junction is. Its neighbour
`work/heldrecords/an-n-ary-fold-carrying-a-rim-junction-refuses-at-its-first-lump.md`
(the fold hands a junction no rim) is the n-ary union's half of the
same seam, and is owed first: a given junction needs its rim before
either reading can place it.

The row that closes this: a block cut by a rod and an elbow off its own
cap, once their union builds. The cut builds a body of the block's
volume less both tools', and does so in either tool order.
