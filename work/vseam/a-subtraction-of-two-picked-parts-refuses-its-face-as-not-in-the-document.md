---
id: a-subtraction-of-two-picked-parts-refuses-its-face-as-not-in-the-document
kind: issue
title: Subtracting two parts picked in the viewer refuses: a picked face's minting Place "is not a node of the document this evaluation ran over"
status: open
opened: 2026-11-02
priority: P0
cost: M
---

Reported by Ev (in chat, 2026-11-02). Filed by the VSEAM orchestrator.

## What Ev saw

Ev picked two parts by clicking in the viewer and asked for a
subtraction. It refused with:

> this face cannot be resolved right now: the reference is
> indeterminate until its minting node evaluates: Place 3c3ee5a6951d is
> not a node of the document this evaluation ran over. Recourse: ask
> about one of that document's nodes, or evaluate the document the node
> is in

Both parts existed. They were picked off the picture.

## Where the words come from

- `crates/viewer/src/app.rs`, `indeterminate_wording`: "this {noun}
  cannot be resolved right now: …".
- `crates/editor-core/src/resolve/mod.rs`: "the reference is
  indeterminate until its minting node evaluates: …".
- `crates/editor-core/src/eval/mod.rs`, `NodeStanding::NotInDocument`:
  "… is not a node of the document this evaluation ran over".

So a face selection minted by a `Place` (a world placement) was asked
of an evaluation whose document does not hold that `Place`.

## Measure first

`memories/refusal-text-is-not-cause.md` applies: the sentence names
where it was raised, not why. Reproduce it through the session doors
(two parts placed in the world, a click-pick of each, then the
boolean's subtract) and find which evaluation the face was asked of.
Candidates to check, not conclusions:
- the landed evaluation of an earlier document (the pick taken off the
  landed run while the door reads the committed document; neighbour:
  `a-held-pick-is-admitted-off-the-landed-run-while-its-door-reads-the-committed-document`);
- an instance's part document (a face of a part instance resolved
  against the part's own evaluation rather than the host's);
- a pick held across the edit that minted the `Place`.

## Why P0

A normal verb (subtract) broken on normal geometry (two parts picked in
the picture), reported by Ev. The wording also asks the person to do
something they cannot ("evaluate the document the node is in"), which
is a second defect to fix with it.
