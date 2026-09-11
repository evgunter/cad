---
id: viewer-cannot-author-a-part-node
kind: issue
title: The viewer has no AddPart op, so a Part { Instance(i) } node — the road to a nested copy the mate tool now admits — is reachable only from a file or the Python API
status: open
opened: 2026-09-06
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
