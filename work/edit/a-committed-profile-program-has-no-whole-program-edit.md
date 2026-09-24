---
id: a-committed-profile-program-has-no-whole-program-edit
kind: issue
title: A committed profile's program can be edited only one argument at a time; no edit reshapes it or writes it whole
status: review
branch: edit/program-edit
pr: 2927
opened: 2026-09-18
priority: P0
cost: D
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

## Built (2026-09-20, lane `program`, branch `edit/program-edit`)

Landed, as the spec's premises with three of them corrected by
measurement (the PR body is the record):

- `DocEdit::SetProgram { node, loops, provenance }` with
  `LoopProvenance { from, steps }`; the shape checked first through
  `ProvenanceFault`'s seven arms carried as ONE `EditError` arm
  (`ProvenanceMalformed { node, fault }`, the `MeasureMalformed`
  precedent) plus `SetProgramOnNonProfile`; then `check_node_slots`
  and `ProfileProgram::check_returning` — the insert door's own
  functions — on the rewritten node.
- `ProfileProgram::check` split into `replay_records` →
  `check_returning` → `check`, one body; the trait gains
  `check_returning`, `replay_records`, `loops`, `with_loops`.
- The segment map (`SegmentMap`) read off both replay records; a
  vertex is carried by the segment ARRIVING at it (premise 3 said
  leaving, which is wrong exactly at an inserted leg's start — measured
  in `a_vertex_is_carried_by_the_segment_arriving_at_it`).
- The walk over `Doc::name_carriers` descends every `NameRef` and asks
  each minting node which profile anchors its locators
  (`Node::anchoring_profile`: extrude, revolve, loft's first section,
  sweep) — premise 4's `name.node == node` filter finds nothing, since
  a profile mints no name. Kept names are rewritten in place through
  `Node::rewrite_payload_names` (one simultaneous pass per node, the
  substrate `rebind_payload_names` now calls) and the store re-keyed
  through `move_appearance_record` (the `Rebind` arm's collision rule,
  one home). `Maintenance::Rebound { from, to }`, one row per name.
- A stranded name is RETIRED to a coordinate past its loop's end and
  reported `Strand`/`StrandedAppearance` with that spelling — left in
  place it would alias the segment the new program draws at its old
  index (the DI1 class the ruling exists to end) and collide with a
  rebound name in the same selection; retired it resolves `Vanished`,
  which is the evaluation refusal the spec's finding row expected
  (`NodeGone` was the wrong rung: the node is live).
- The order contract's rebound clause; persistence (the non-finite
  walk's arm, the wire spelling pinned, the corpus's `reshaped_rod`
  as the first persisted `SetProgram`); the Python door
  `DocEdit.set_program` sharing `Node.profile`'s outline reading, the
  `rebound` maintenance row with `rebound_to`; V2 and DM7 re-worded on
  #2904; `SlotId::Profile`'s doc cites V2.

Filed: `work/author/the-viewer-keeps-its-profile-lock-and-order-search-after-set-program.md`
(VIEW's follow-up, every survey site listed) and
`work/edit/an-unknown-edit-tag-in-a-log-refuses-without-naming-it.md`
(the log wrapper's untagged miss, measured by the persisted-spelling
row).

### Fix pass (2026-09-20, lane `program-fix`, the union of R1's and R2's findings under the orchestrator's rulings)

R1's MAJOR stood: a retired coordinate one past the loop's end went
live under a plain `SetParam` (a corner fillet's runs through a `Zero`
fit change the loop's segment count), so a stranded name is now
retired to `RETIRED_FLOOR + s` (segment) or loop `RETIRED_FLOOR + l`,
`RETIRED_FLOOR = u32::MAX / 2` — a coordinate no program can draw
under any edit — and a name already at or above the floor is left
exactly as it is and reported by no later edit (R2's growth probe now
asserts an empty maintenance). The retirement is Ev's to ratify: DM7
and V2 say only what #2904 ruled, the mechanism is documented on
`DocEdit::SetProgram` and `RETIRED_FLOOR`, and
`stranded-names-are-retired-to-an-undrawable-coordinate` (`needs_ev`)
carries the case, the cost of "left in place", R1's finding and the
alternatives. The segment map is read through DM8's checked door
(`CheckedRecords`, one shape check for the evaluation's doors and the
edit's; a new program whose record the door refuses is
`ProgramRefusal::Record`, an old one strands). One roster drives both
reports: `Doc::rewrite_names` is `name_carriers`' `&mut` twin over
`Carrier::ALL`, and the order row holds that the twin meets exactly
what the read walk yields. One walk over `RoleSeg`'s shape
(`RoleSeg::rewrite` under a `SegRewrite`) serves the anchor rewrite,
the split re-map and the program edit; `name_free_seg!` is the union
of `inert_seg!` and `locator_seg!`, because the compiler refuses the
union after the locator arms. `anchor.rs`'s "a parameter edit CANNOT
renumber" names its exception and
`a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names` is
filed with a live-name row pinned as measured; DI1's record names it.
Both probe suites were merged authorship-preserving, then folded by
name into `edit_set_program.rs` with the duplicates retired (the PR
body says which of each pair survived); the suite's rod helpers are
the corpus module's. New rows: the retired name through the resolver
door and a Python `resolve`, a frame on a dropped wall refusing
`Vanished`, and one row per sweep kind tying `Node::anchoring_profile`
to what the evaluation publishes.
