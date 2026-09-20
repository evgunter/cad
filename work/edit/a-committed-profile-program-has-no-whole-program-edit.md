---
id: a-committed-profile-program-has-no-whole-program-edit
kind: issue
title: A committed profile's program can be edited only one argument at a time; no edit reshapes it or writes it whole
status: spec
branch: edit/program-edit
opened: 2026-09-18
---


## What is missing

`DocEdit` (`crates/editor-core/src/edit.rs`) writes a profile node's
program through its SLOTS: `SetParam` / `SetStructuralParam` /
`SetExpression`, one argument each, and every one of them re-runs the
whole-program check afterwards (`check_profile_after_slot_edit`). There
is no edit that replaces a live profile's program — its verbs, their
order and count, arc modes, sides, windings, target forms, a split
circle's `n`, or its loop count. `SetMembers` is the only edit that
rewrites a live node's content wholesale, and it covers list inputs
only.

## What that costs the viewer, today

The viewer's profile editor is one editor with two doors (VSEAM,
`editing-a-profile-does-not-share-the-create-forms-interface`): the
add-profile form's step list, opened either on nothing (commits
`InsertNode`) or on a committed profile (commits
`SessionOp::EditProfile`, `crates/viewer/src/session.rs`,
`DocSession::edit_profile`). Because of the gap above, the edit door:

1. **Locks the shape.** Every control that would change the program's
   structure is drawn disabled on a committed profile
   (`forms::ShapeEdits::Locked`, `pane::create::path_steps_ui`), and a
   reshaped program sent anyway refuses
   `Refusal::ProfileRestructure` (`sketch::program_edits`). Reshaping a
   committed profile — inserting a leg, turning a line into an arc —
   has no route at all short of deleting the profile, which cascades
   through everything built on it.
2. **Writes numbers one slot at a time.** A set of numbers that is a
   valid profile together can pass through an intermediate program the
   door refuses. `session::accepted_order` holds a refused write back
   until the others have made it valid (a square moved bodily lands —
   `tests/profile_edit.rs`,
   `a_move_whose_first_write_alone_crosses_still_lands`), but where no
   order works the edit refuses `Refusal::ProfileEditOrder` about a
   state nobody wrote.

## What would close it

An edit that replaces a live profile node's whole program (plane
excepted — that is an input, DM6), validated once, as ONE `DocEdit`.
That is a vocabulary change at the edit layer and a persistence one
(every `DocEdit` is persisted, GQ3), so it is a design decision for
this program, not something the viewer can add. With it, the viewer
drops the lock and `accepted_order`, and the edit door commits the
editor's program as that one edit.

## Put to Ev (2026-09-20, EDIT orchestrator) — the seventh `[ev]` PR

**The question.** Should the document gain an edit that replaces a
live profile's whole program (plane excepted) as ONE validated
`DocEdit` — and if so, what happens to the names that hold that
program's steps?

**Why it is Ev's.** V2 (`crates/profile/README.md`, ratified) says
"structural data … stays literal; changing it is re-authoring … step
indices are stable because structure changes only by re-authoring",
and `SlotId::Profile`'s doc rests on that sentence (`node.rs` ~406:
"the frozen-selection argument, V2"). Re-authoring today means delete
the profile and insert a new one, which cascades through everything
built on it (DM7 reports every stranded name at that delete). An edit
that reshapes a LIVE profile changes what V2 decides, and it touches
DM7's scope — today "a stranded name is reported at THE DELETE" — so
it is a design change on two ratified pages, not a description the
code moved. The viewer's cost is measured on this row: a locked shape
(`forms::ShapeEdits::Locked`), a per-slot write order search
(`session::accepted_order`, capped) and two refusals about states
nobody wrote.

**The hazard that decides the shape.** Every holder of a profile
name — fillet/chamfer selections, shell open lists, face frames, mate
heads, measures, declare pairs, instance interfaces, the appearance
store (REFERENCES.md §0's carrier list) — holds a `ProfileEdgeRef {
loop, segment }` by INDEX. A reshaped program (a leg inserted before
segment 2) leaves index 2 valid and denoting a different wall: no
refusal, no report, the DI1 aliasing class. So "state the new program
in full" (the `SetMembers` shape) is not enough on its own.

**Recommendation: (B) the edit carries the new program AND each new
step's provenance; the door reports and rebinds.**
`DocEdit::SetProgram { node, loops, provenance }` where `provenance`
says, per new loop and step, which old `(loop, step)` it continues or
that it is new (the editor knows this — it inserted the leg). The
door replays both programs (it already replays the new one:
`ProfileProgram::check`), maps each old step's segments to the new
step's through DM8's step→segment record, and then: a name on a
segment of a DROPPED step, or of a step whose segment count changed
(a line that became `arc_fillet` is a changed step), is reported
`Maintenance::Strand` — DM7 widened from "the delete" to "the edit
that removes a name's referent", its own "why not as-is" bullet
carrying the argument; a name on a kept step's segment is REWRITTEN
to its new index in place (the split's `rebind_payload_names` door
and the appearance store's twin) and reported as its own
`Maintenance` arm, so a moved name is visible in the accepted edit's
maintenance and never silently re-denotes. Plane excepted (DM6: no
edit rewires a live node's inputs; the plane is the profile's one
input). Persisted like every `DocEdit` (GQ3; serde-derived, no
version, `deny_unknown_fields`; the save-side non-finite walk is
exhaustive on the enum and gains its arm). Python gets a
whole-program door (`slot_from_word` has no `profile` arm by design).
V2's sentence becomes "structure changes only by `SetProgram`, which
reports and rebinds every name its reshaping touches", and
`SlotId::Profile`'s stability claim is re-worded to cite it.

**Alternatives.**
- **(A) Refuse when referenced.** `SetProgram` with the program only;
  it refuses typed when any held name resolves to a segment of that
  profile, so the author deletes the dependents first. Cheap and
  honest, no rebind machinery; but it is a refusal where DM7's spirit
  says report, and it makes the common case (reshape a profile that
  has a fillet on it) a delete-and-rebuild again — most of the cost
  this row measures.
- **(C) Program only, names keep their indices.** Rejected: the
  aliasing hazard above, silent.
- **(D) Leave V2 as it is.** The viewer keeps its lock and order
  search; this row closes as "by design" and the two refusals stay
  documented costs.

**What Ev decides.** (B), (A) or (D) — and, under (B), whether a
moved name is rewritten in place with a maintenance report (the
recommendation) or reported and left for a `Rebind` the author
issues. On a ruling this is spec'd as a kernel unit (v7: a design
decision whose impact is broad), block EDIT-B2 slot 2 (FABLE), with
the viewer's drops announced as VIEW's follow-up.

## RULED (2026-09-20, Ev on `[ev]` #2904) — (B): the edit carries the program and each step's provenance; the door reports and rebinds

Ev: "(B) makes sense!" — the recommendation as put, including its
sub-choice: a name on a kept step is REWRITTEN to its new index in
place by the door and the move reported as its own `Maintenance` arm;
a name on a dropped or changed step is reported `Strand`. So V2's
"structure changes only by re-authoring" becomes "structure changes by
`SetProgram`, which reports and rebinds every name its reshaping
touches", and DM7's subject widens from the delete to the edit that
removes a name's referent — both re-wordings land with the unit that
builds them, this ruling being their ratification. Spec'd next as a
kernel unit (v7: a broad design decision), block EDIT-B2 slot 2
(FABLE), `docs/EDIT-PROGRAM-SPEC.md`; the viewer's drops (the lock,
the order search, the two refusals) are VIEW's follow-up, announced
in the spec.

## Ruled and spec'd (2026-09-20, EDIT orchestrator) — kernel unit, v6 dual, block EDIT-B2 slot 2 (FABLE), branch `edit/program-edit`

Spec: `docs/EDIT-PROGRAM-SPEC.md` (wave 17; deleted at merge and
ledgered). Ev's ruling (B) above is the decision; the spec turns it
into nine premises — the variant and its provenance, the insert door's
own checks, the segment map read from the replay record, the
report-and-rebind walk over both name carriers, the order contract's
new clause, persistence, the Python door, the two ratified sentences
re-worded on the ruling, and the viewer follow-up filed on VIEW's
slate rather than crossed into. Pre-draw fields at the spec (L /
STRUCTURAL, after the block byte — disclosed). The implementer is
dispatched when the claim merges; the dual follows on its frozen head.

