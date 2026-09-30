---
id: viewer-cannot-author-a-part-node
kind: issue
title: The viewer has no AddPart op, so a Part { Instance(i) } node — the road to a nested copy the mate tool now admits — is reachable only from a file or the Python API
status: closed
opened: 2026-09-06
priority: P0
cost: D
branch: author/part-and-duplicate
pr: 3052
closed: 2026-09-24
---


Reported by MSOLVE-2's implementer lane (PR 2039), outside its fence;
filed by the MSOLVE orchestrator. CHROME's by shape (the viewer's
authoring ops).

`crates/viewer/src/session/op.rs` has `AddSplit`, `AddPattern` and
`AddTransform` but no `AddPart`, while `crates/viewer/src/combine.rs`
admits `Node::Part` at every body seat and names CHROME as the door
that authors one. Since MSOLVE-2 the mate tool admits a pick on a
nested copy (`Pattern` over `Part { Instance(i) }` over `Pattern`) and
on a transform over one copy, so the GUI now accepts picks on a shape
it cannot create. MSOLVE-2's viewer rows author the document through
`DocEdit::InsertNode` and open it through the ordinary door; the picks
themselves are real rays onto drawn bodies. An `AddPart` op (which
body of a split or which instance of a pattern) closes the gap.

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **M** — new op plumbed through four files; must
decide seat and instance args. The class is a dispatch estimate made by
reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Re-homed to CHROME (2026-09-11)

Moved out of `work/door/` by the DOOR orchestrator, with Ev's direction
in-chat. It was gathered into DOOR's opening slate at class **M**; that
estimate was wrong, and the row fails DOOR's one charter test.

**Reading it does not tell you the diff.** It names four files and says
an `AddPart` op closes the gap, but not what the op takes — and which
seat and which instance argument it carries is the whole of the work,
not a detail below it. DOOR's charter is explicit that the fix must be
written in the row, and that a row which grows a design question is
re-homed rather than carried at the wrong class.

**CHROME is the owner by shape and by precedent.** `crates/viewer/src/*`
is CHROME's territory, the row's own filing says "CHROME's by shape (the
viewer's authoring ops)", and the sibling gap —
`placed-union-has-no-session-op`, an insert op the session vocabulary
lacked for a kernel node the combine layer already admitted — was
CHROME's and closed by CHROME in PR 1762. That is the shape to copy and
the argument for where the row belongs.

## A second consumer refuses on it (2026-09-21, AUTH-1)

Found by AUTH-1's implementer lane, inside AUTHOR's own fence, and
recorded here rather than as a second row.

The add-datum form's new `frame on face` seat declines a face picked on
a node whose value is several bodies, because `Datum::FaceFrame` reads
its face through the evaluator's single-body operand door
(`eval::wire::body_operand`, reached from the `Datum::FaceFrame` arm of
`wire_datum`) and a split's sides and a pattern's instances are not one
body. The refusal is a value — `session::refuse::FaceFrameFault::
NotOneBody`, with `session::add_datum`'s `WrongNodeKind { wanted: Body }`
behind it — and
`docm1_face_frame::several_bodies_is_no_seat_for_a_face_frame` drives
both.

So the gap this row names now costs a SECOND ordinary gesture, not just
the nested-copy one: click the top of one half of a split, and the GUI
cannot put a sketch frame on it. `combine::denotes_body`'s own doc says
the recipe's way of naming one of several is `Node::Part` and that "the
door that authors one is CHROME's" — the door this row asks for. An
`AddPart` op would make the face-frame seat reachable there with no
further change to the datum path: the author projects the half, picks
the face on the projection, and the same gate admits it.

## Closed 2026-09-24 — PR 3052 merged (`2273a3a1`)

`SessionOp::AddPart { of, select: PartSelectSpec }`, with the instance
index an `i64` at the op door on `AddPattern`'s structural-slot
precedent. The panel calls it the **projection tool**, since "part"
already names another document in the `Add part…` chooser beside it.
It seats a split or a pattern picked in the viewport or the tree, and
says that projecting one body stops drawing the rest.

Residue: `work/forms/a-projected-split-is-unreachable-from-the-viewport`
(after one projection the other half is reachable only from the tree),
the body-seat kind-vs-value disagreement now pinned as a named
exception on `work/forms/body-seat-reads-through-the-placer-chain`, and
`no-row-holds-that-the-create-pane-offers-the-tools-it-has`.
