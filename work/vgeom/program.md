---
id: vgeom
kind: program
title: VGEOM — the viewer's geometry, camera and numeric renders
status: ready
opened: 2026-09-17
area: gui
prefix: vgeom/
tag: (VGEOM orchestrator)
ab_band: 5300-5399
paths: [crates/viewer/src/camera.rs, crates/viewer/src/datums.rs, crates/viewer/src/marks.rs, crates/viewer/src/scene.rs, crates/viewer/src/bounds.rs, crates/viewer/src/props.rs, crates/viewer/src/widgets.rs, crates/viewer/src/readout.rs, crates/viewer/src/input.rs, crates/viewer/src/sketch.rs, crates/viewer/src/gpu.rs, crates/viewer/src/idpass.rs, crates/viewer/src/pickindex.rs, crates/viewer/src/pane/view.rs, crates/viewer/src/pane/viewport.rs, crates/viewer/src/pane/properties.rs]
keep_out: [OPENED 2026-09-17 in VIEW's re-scope as one of four successors on VIEW's ground - VIEW stays open until its exit walk and its paths still cover crates/viewer/src/* so every path above is VIEW's too by declaration and VIEW's keep_out names this program in return, the three sibling successors are vnews and vseam and vdoc and the shared files are written on both sides rather than fenced - pane/viewport.rs is claimed here and by vnews and by vseam because a viewport row can be a projection row or a news row or a staleness row and which it is reads off the row not the file, pane/properties.rs is claimed here and by vnews because a field's VALUE is this program's and the sentence beside it is that one's, pickindex.rs is claimed here and by vseam - the index's NaN and rounded-t and tie-break doors are this program's and the seam the index lives behind is vseam's, scene.rs and widgets.rs and display.rs and sketch.rs are worked here for their numbers and by vnews or vseam for their words and their held state, crates/viewer/README.md and crates/viewer/tests/* and crates/viewer/src/lib.rs are vdoc's - a stale citation or a census or a missing assertion found here is filed on vdoc and never fixed across that fence except where a unit's own diff moves the lines a row cites which it repoints in its own PR, crates/viewer/src/* is also CHROME's by declaration and CHROME's carve-out of 2026-09-15 keeps datums.rs and bounds.rs for CHROME and cedes scene.rs and gpu.rs and marks.rs and blend.rs and props.rs and sketch.rs to VIEW - this program inherits both halves so a datums.rs or bounds.rs row here is announced to CHROME before it is taken and CHROME's keep_out names VIEW and not yet this program with the row asking it to write its side on CHROME's slate, crates/viewer/tests/* is S-TCOST's and S-TINT's and Track W's by declaration so a test-mechanism change is announced to both, crates/editor-core is EDIT's and MSOLVE's and crates/geom-core and crates/mesh are the kernel programs' - a numeric door the viewer consumes is a hand-off and never a diff from here, PERF cedes per-frame rendering and hover-picking here on the same terms it ceded them to VIEW]
priority: P1
---

One of four successors opened by VIEW's re-scope of 2026-09-17, on the
half of VIEW's slate that sits on the path from a document's geometry
to the picture and to the figures printed beside it: the camera and the
scale it yields, the datum and mark and scene geometry, the display-budget
fit, the GPU id pass and shader, and the numeric fields. Twenty-one rows
arrived from `work/view/`, each by `git mv` with its body and its
history unchanged.

Charter and order: `work/vgeom/plan.md`; narrative in
`work/vgeom/log.md`. The lane register that binds every lane dispatched
here is `work/view/plan.md`'s, inherited by reference and not copied —
`work/vgeom/plan.md` §The register says why and what happens to it when
VIEW's directory goes.
