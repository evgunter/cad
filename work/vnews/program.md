---
id: vnews
kind: program
title: VNEWS — the viewer's news vocabulary
status: active
opened: 2026-09-17
area: gui
prefix: vnews/
tag: (VNEWS orchestrator)
ab_band: 5200-5299
paths: [crates/viewer/src/frame.rs, crates/viewer/src/display.rs, crates/viewer/src/pane.rs, crates/viewer/src/pane/create.rs, crates/viewer/src/pane/features.rs, crates/viewer/src/pane/properties.rs, crates/viewer/src/pane/viewport.rs, crates/viewer/src/seats.rs, crates/viewer/src/tools.rs, crates/viewer/src/tree.rs, crates/viewer/src/session/refuse.rs]
keep_out: [OPENED 2026-09-17 in VIEW's re-scope as one of four successors on VIEW's ground - VIEW stays open until its exit walk and its paths still cover crates/viewer/src/* so every path above is VIEW's too by declaration and VIEW's keep_out names this program in return, the three sibling successors are vgeom and vseam and vdoc and the shared files are written on both sides rather than fenced - pane/viewport.rs is claimed here and by vgeom and by vseam because a viewport row can be a news row or a projection row or a staleness row and which it is reads off the row not the file, pane/properties.rs and pane/create.rs are claimed here and by vgeom and vseam respectively for the same reason, frame.rs and display.rs and tools.rs and session/refuse.rs are claimed here and by vseam because a session op's refusal is spelled here and decided there, display.rs and sketch.rs word-rendering sites are this program's and their geometry is vgeom's, crates/viewer/README.md and crates/viewer/tests/* and crates/viewer/src/lib.rs are vdoc's - a prose or census or citation defect found here is filed on vdoc and never fixed across that fence, crates/viewer/src/* is also CHROME's by declaration and CHROME's carve-out of 2026-09-15 cedes app.rs session.rs session/* pane/* frame.rs pickindex.rs display.rs props.rs forms.rs sketch.rs to VIEW so this program inherits that cession - CHROME's keep_out names VIEW and not yet this program and the row asking it to write its side is on CHROME's slate, crates/viewer/tests/* is S-TCOST's and S-TINT's and Track W's by declaration so a test-mechanism change is announced to both, crates/editor-core is EDIT's and MSOLVE's since DOCM exited - a news row reaching a refusal raised there is a hand-off and not a diff from here except for the EditError Display wording Ev authorised in-chat 2026-09-04 which this program inherits unchanged, the GUI-3 section 5 seam and GQ7 are ratified design in crates/viewer/GUI-DESIGN.md and a revision is an ev PR]
priority: P3
---

One of four successors opened by VIEW's re-scope of 2026-09-17, on the
half of VIEW's slate whose subject is **what the viewer tells a reader
and the vocabulary that telling travels in**: a frame's news, a
control's refusal, a verdict, a tone, a notice, a recourse. Fourteen
rows arrived from `work/view/`, each by `git mv` with its body and its
history unchanged.

Charter and order: `work/vnews/plan.md`; narrative in
`work/vnews/log.md`. The lane register that binds every lane dispatched
here is `work/view/plan.md`'s, inherited by reference and not copied —
`work/vnews/plan.md` §The register says why and what happens to it when
VIEW's directory goes.
