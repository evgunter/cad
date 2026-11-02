---
id: a-subtraction-of-two-picked-parts-refuses-its-face-as-not-in-the-document
kind: issue
title: A boolean that fails shows a downstream face-resolution sentence in red at the tool, while its own refusal shows grey elsewhere
status: open
opened: 2026-11-02
priority: P1
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

## 2026-11-02 — what actually happened (Ev, in chat): re-scoped and moved to VNEWS

Ev: "the actual issue was 'Intersect "Intersect 1" (52727543510b)
failed: the Boolean op refused: an edge of the first operand touches or
crosses a curved face of the other operand away from that face's
edges, and the Boolean cannot yet settle where or whether it passes
through. Recourse: move the geometry' but that was shown in grey in a
different place whereas the error i pasted was shown in red right next
to the boolean tool".

So the cause was the kernel's curved-pierce frontier
(`BooleanError::CurvedPierceUnsupported`, `crates/topo/src/boolean/mod.rs`),
on the node the tool had committed. The face-resolution sentence above
is a CONSEQUENCE: something read a face through a node whose
evaluation did not produce it. Two defects follow, and both are about
what a reader is shown:

1. **The consequence outranks its cause.** The tool's red line names a
   face reference and a `Place` id. The failure that explains it, the
   boolean's refusal, sits in grey elsewhere. The frame's verdict policy
   (`frame::frame_status`, the `Refusal::preferred` ranking, and the
   precedent `a_refusal_that_follows_from_a_failed_node_is_quieter_than_it_and_names_it`
   in `frame_policy`) already says a refusal that follows from a failed
   node is quieter than that node and names it. This path escapes that
   rule. Find why: which refusal arm carries `NodeStanding::NotInDocument`
   here, and why it is not tied to the failed node.
2. **The consequence's own words are wrong for a reader.** "Place
   3c3ee5a6951d is not a node of the document this evaluation ran over"
   and "evaluate the document the node is in" describe an internal
   pairing, not anything the person did or can do. Investigate why the
   failed boolean makes a face read land on a document without that
   `Place`, as the original "Measure first" section says.

Moved from VSEAM to VNEWS: which sentence a frame shows, and how loudly,
is VNEWS's charter. Re-banded P0 → P1: the verb's failure is the
kernel's frontier, and this row is the GUI defect Ev reported as
misleading. The kernel side has no row of its own. Many
curved-pierce rows exist, and Ev's geometry (an Intersect of two parts)
is not in hand to file one.

## 2026-11-02 — a third defect, the class (Ev, in chat)

Ev: "it's also baffling to have an inactionable red error for 'the
current selection is invalid'".

3. **An unresolvable selection is a STATE, not an error.** A held pick
   the current evaluation cannot place (its minting node failed, was
   undone, or has not evaluated yet) is not something the person did
   wrong and not something they can repair at that line. Showing it
   red, at the tool, as the tool's own failure is the wrong tone.

   - The sweep is wider than the boolean tool: every place `app.rs`'s
     `indeterminate_wording` ("this {noun} cannot be resolved right
     now") is drawn. That includes the properties pane's face and edge
     rows (`pane/properties.rs`) and each modal tool's held picks.
   - What it should be instead: quiet or advisory, pointing at the
     cause where there is one (the failed node, per defect 1), and
     otherwise saying the selection will resolve once the node
     evaluates. Alternatively, drop the pick if it can never resolve.
   - Check it against the tone rules the frame already has
     (`frame::frame_status`, the Loud/Advisory/Weak split) rather than
     inventing a new one.
