---
id: vseam
kind: program
title: VSEAM — the viewer's seams and session vocabulary
status: ready
opened: 2026-09-17
area: gui
prefix: vseam/
tag: (VSEAM orchestrator)
ab_band: 5400-5499
paths: [crates/viewer/src/app.rs, crates/viewer/src/session.rs, crates/viewer/src/session/author.rs, crates/viewer/src/session/delete.rs, crates/viewer/src/session/op.rs, crates/viewer/src/session/probe.rs, crates/viewer/src/session/refuse.rs, crates/viewer/src/session/select.rs, crates/viewer/src/evalseam.rs, crates/viewer/src/pickcache.rs, crates/viewer/src/pickindex.rs, crates/viewer/src/generation.rs, crates/viewer/src/history.rs, crates/viewer/src/g1.rs, crates/viewer/src/docio.rs, crates/viewer/src/forms.rs, crates/viewer/src/vocab.rs, crates/viewer/src/combine.rs, crates/viewer/src/tools.rs, crates/viewer/src/pane/create.rs, crates/viewer/src/pane/viewport.rs]
keep_out: [OPENED 2026-09-17 in VIEW's re-scope as one of four successors on VIEW's ground - VIEW stays open until its exit walk and its paths still cover crates/viewer/src/* so every path above is VIEW's too by declaration and VIEW's keep_out names this program in return, the three sibling successors are vnews and vgeom and vdoc and the shared files are written on both sides rather than fenced - pane/viewport.rs is claimed here and by vnews and by vgeom because a viewport row can be a staleness row or a news row or a projection row and which it is reads off the row not the file, pane/create.rs and tools.rs and session/refuse.rs are claimed here and by vnews - which op exists and what it may do is this program's and the sentence a refusal shows a reader is that one's, pickindex.rs is claimed here and by vgeom - the seam the index lives behind is this program's and the index's own numeric doors are that one's, frame.rs and display.rs and widgets.rs and scene.rs are vnews's or vgeom's and are worked here only where a row's held state crosses them which it announces, crates/viewer/README.md and crates/viewer/tests/* and crates/viewer/src/lib.rs are vdoc's - a prose or census or citation defect found here is filed on vdoc and never fixed across that fence except where a unit's own diff moves the lines a row cites which it repoints in its own PR, crates/viewer/src/* is also CHROME's by declaration and CHROME's carve-out of 2026-09-15 cedes app.rs and session.rs and session/* and pane/* and frame.rs and pickindex.rs and display.rs and props.rs and forms.rs and sketch.rs to VIEW so this program inherits that cession - CHROME's keep_out names VIEW and not yet this program and the row asking it to write its side is on CHROME's slate, crates/viewer/tests/* is S-TCOST's and S-TINT's and Track W's by declaration so a test-mechanism change is announced to both, crates/editor-core is EDIT's and MSOLVE's since DOCM exited and crates/pncad is LIB's - an op this program wants that needs a kernel or document door is a hand-off filed on that owner and never a diff from here except for the EditError Display wording Ev authorised in-chat 2026-09-04 which this program inherits unchanged, GQ7 and the GUI-3 section 5 seam are ratified design in crates/viewer/GUI-DESIGN.md and a revision is an ev PR - the deferred pick-priority row that arrived here rests on GQ7 and stays deferred until that ruling moves]
priority: P1
---

One of four successors opened by VIEW's re-scope of 2026-09-17, on the
half of VIEW's slate whose subject is **what the viewer holds on behalf
of the document**: an op it can author, a spec it lowers, a field
derived from the document, an answer cached across a generation, a
gesture held across frames. Fourteen rows arrived from `work/view/`,
each by `git mv` with its body and its history unchanged.

Charter and order: `work/vseam/plan.md`; narrative in
`work/vseam/log.md`. The lane register that binds every lane dispatched
here is `work/view/plan.md`'s, inherited by reference and not copied —
`work/vseam/plan.md` §The register says why and what happens to it when
VIEW's directory goes.
