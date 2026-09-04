---
id: viewer-mate-tool-refuses-pattern-picks
kind: issue
title: Viewer mate tool refuses pattern-placed picks — cannot author the mates the A11 member vocabulary admits
status: review
opened: 2026-08-31
github: 1412
refs: [1400]
branch: chrome/viewer-mate-tool-refuses-pattern-picks
---

## From GitHub issue 1412

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, R2 MINOR-2, verified at the site). `crates/viewer/src/matetool.rs:417` gates mate-member picks through `is_instance` (`display.rs:193`), which matches `InstantiatePart` heads only — so the GUI refuses `NotAnInstancePick` for exactly the pattern-placed heads the member-vocabulary rider now admits and the solve now places. A user can author these mates through the recipe/Python doors but not by picking in the viewer.

GAUTH ground (viewer chrome), flagged for the GAUTH orchestrator's queue: the fix is presumably widening the pick gate to the member vocabulary (`Pattern` + `Instance(i)` over a live instance) and emitting the `Instance(i)`-headed reference, with `NotAnInstancePick` retained for everything else.

Signed: (S-MATE orchestrator)

## Home

`work/issues/` — the issue routes itself to GAUTH (viewer chrome), and both GAUTH and GUI are closed programs, so no open program owns `crates/viewer`.

## Fixed (CHROME, 2026-09-04)

The gate is widened, and the widening was not the whole defect.

**The vocabulary has one home.** A11's member rule lived privately
inside `mate::solve::head_of`, and the viewer restated a narrower
version of it as `display::is_instance` — which is how this door came
to refuse heads the solve already places. The rule is lifted to
`pub fn member_of(doc, name) -> Option<Member>`
(`crates/editor-core/src/mate/solve.rs:147`), `head_of` delegates and
attributes the refusal, and the viewer's gate READS it
(`crates/viewer/src/matetool.rs:118`) rather than carrying a copy that
can drift. `display::is_instance` stays narrow for its two remaining
callers: per-instance display state — hide, free-move — is keyed on
`InstantiatePart` nodes and is a different question wearing a similar
name.

**A copy's alignment is authored at its MASTER.** The reference was
free (`pick.name` is already `Instance(i)`-headed), but the frame was
not: `poses.placement` is keyed on instances, so a pattern node reads
the identity, and the pattern-derived offset is `pub(crate)` in
`eval::wire` and unreachable from the viewer. Reading the copy's own
world pose and dividing by the master's placement folds that offset
into the authored numbers, and the solve — whose left factor it is —
then applies it a second time. So the pose is read at the master
entity named inside the head's `Instance(i)` qualifier, on the
pattern's input instance. Verified red: with the naive read, copies 0
and 1 of one pattern author alignments differing by the pattern step.

**`SamePick` compares members, not nodes.** Two copies of one pattern
are two members over one instance, and a mate between them is a loop
the solve records rather than a self-mate. Its payload is `head`
rather than `instance`, a pattern-placed head being no instance node.

**Territory**: this unit edits `crates/editor-core` and
`crates/pncad`, outside CHROME's declared `paths`. Announced rather
than taken silently — the alternative was a second copy of the
admission rule in the viewer, which is the defect being fixed.
