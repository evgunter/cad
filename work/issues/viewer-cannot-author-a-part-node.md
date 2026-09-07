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
