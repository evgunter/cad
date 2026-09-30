---
id: a-refusal-offers-no-action-in-the-viewer
kind: issue
title: viewer: a kernel refusal is shown as prose with no action the viewer can take on it
status: open
opened: 2026-09-17
priority: P1
cost: D
---

**Ev reported this** (in chat, 2026-09-17), from the same failed union
as `a-derived-pick-index-failure-outshouts-its-cause.md`.

The Boolean refusal ends with "Recourse: move the two apart, or
express the cut with cylindrical or spherical tooling, or wait on the
join lane". None of the three is something the viewer offers as an
action, and the refusal is shown as a long paragraph with the recourse
buried at the end. The kernel gap itself (torus×plane) is CURVED's and
already scheduled (`c5-plane-torus-cone-cylinder-arms`,
`torus-operand-gate-admission`). This row is the viewer half: a
refusal's recourse should be shown as its own thing, separate from the
explanation. Where the recourse is something the viewer can do (the
common Boolean case the message itself names, declaring a
coincidence), it should be a control, not a sentence. Where the only
recourse is a kernel capability that does not exist yet, the viewer
should say that plainly.

Whether the kernel's typed refusals carry a machine-readable recourse
to build this on, or only prose, has not been checked. If they carry
only prose, the typed half is a kernel-side row for whoever takes this
one.

## Evidence: the Boolean-declare case now has a control (AUTH-9, 2026-09-30)

The case this row names, "declaring a coincidence", is a control since
AUTH-9 (`author/declared-union`, PR #3543). It is this row's second
instance of refuse-then-offer in the viewer, after the parse door's
unknown-parameter offer. What an OFFER unit inherits from it:

- **Two instances, and each has its offer split across the same
  homes.** The wording is `Refusal::offer_wording` for the parameter
  and `Refusal::declare_question` / `Refusal::declare_pair_wording` for
  the declaration, all in `crates/viewer/src/session/refuse.rs`. The
  reader the frame loop calls is `frame::creation_offer` and
  `frame::declare_offer`, side by side in `crates/viewer/src/frame.rs`.
  The held value is `drafts.new_param_offer` and `drafts.declare_offer`.
  There is no general framework yet, so neither instance goes through
  one. They share homes instead.
- **Staleness is split, and deliberately.** The parameter offer stands
  while the add-parameter form's name field still says the offered name
  (`pane/properties.rs`, `add_param_ui`). That is honest because a name
  to create does not depend on the document state. A declaration is
  sited in one document, so `DeclareOffer::is_for` holds it to the
  session generation it was refused at, plus the tool's op and picks.
  One rule could not serve both: the parameter offer has no generation
  to be stale against, and a declare offer would survive an undo under
  the name-field rule.
- **The recourse is doubled.** When a boolean is refused, the status
  line prints the kernel's sentence whole (`RefusedBoolean`'s
  `Display`). That sentence ends on the kernel's prose recourse,
  "declare the candidate pair this refusal carries and wire it into the
  Boolean's declare input, or move the geometry"
  (`UndeclaredContactFinding::recourse`, `crates/editor-core/src/eval/mod.rs`).
  Directly below it, the boolean tool shows the Declare button that
  *is* that recourse. This row's own ask, the recourse shown as its own
  thing and as a control where one exists, is the fix. AUTH-9 left it
  for this row rather than trimming the kernel's sentence in the
  viewer.

## Evidence: a pin mismatch now has a control (AUTH-15, 2026-09-30)

An instance whose part's pin no longer holds now offers **Accept
updated version** under its tree row (`author/accept-part-version`).
It is this row's third refuse-then-offer instance, and the first whose
refusal is a landed node failure rather than a refused batch. It uses
the same homes: `Refusal::version_question` for the wording,
`frame::version_offer` beside `creation_offer` and `declare_offer` for
the reader, and `session::VersionOffer` beside `DeclareOffer` for the
value. Two things differ, and an OFFER unit inherits both:

- **Nothing holds it.** The offer is read off the landed run when the
  tree's rows are built (`TreeRow::version_offer`), so there is no
  `drafts` field. It still needs one staleness rule: the landed run can
  be older than the committed document. That is true right after an
  accept, before its run lands, when the old row would offer the same
  accept again. So `DocSession::tree_rows` withholds every offer while
  `busy()`. The store can also move between the landing and the click.
  The op therefore mints the pin at the commit, and a store that has
  nothing newer refuses in its own words.
- **The recourse is doubled again**, as in the Boolean case. The badge
  prints `PIN_MISMATCH_RECOURSE`, and directly under it is the
  button that records the edit that sentence quotes by name.

**Sweep residue: the tolerance lever has no viewer door.**
`geom_core::predicate::COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE`,
`SPLIT_PLANE_RECOURSE` and `KERNEL_LIMIT_RECOURSE` all end on "lower"
or "loosen the tolerance". So do recourses in fourteen kernel files
across `editor-core`, `geom-brep`, `profile`, `sweep` and `topo`
(`grep -rliE "(lower|loosen)(ing)? the tolerance" crates/*/src`). The
ε-seam part refusal
(`PartFault::Unresolved { fault: EpsilonSeam }`,
`crates/editor-core/src/eval/parts.rs`) says to "record the edit that
sets this process's tolerance". No `SessionOp` emits
`DocEdit::SetTolerance`, and a session's ε is fixed when it is built.
So a GUI author reads a recourse the viewer cannot take and is not told
so. The sweep counted every `DocEdit` variant no viewer door emits (the
constructions in `crates/viewer/src`), then grepped the kernel's
sentences for each one's name or act. The tolerance edit is the only one
a refusal reaching the viewer names. `Rebind` and `ReWitness` are named
only by refusals of those same edits, and of the split refactor, and no
viewer door raises any of those. What the grep cannot see is a recourse
naming the act in other words. The 40 recourse constants and `Recourse`
literals were read by eye for that, and none names a door beyond these.

**The general offer type this row is missing** (named by AUTH-15's
review, as evidence rather than a fix). There are now three offers in
three shapes:

- a bare `ParamName` (`frame::creation_offer`, held in
  `drafts.new_param_offer`);
- `DeclareOffer { accept(), is_for(..) }`;
- `VersionOffer { accept() }`.

They have three question composers (`Refusal::offer_wording`,
`::declare_question`, `::version_question`) and two drawers
(`pane::create::declare_offer_rows`, `pane::features`'
`version_offer_lines`); the parameter offer draws only its sentence.
Their button labels are three associated consts (`VersionOffer::LABEL`,
`DeclareOffer::ACCEPT_LABEL` and `DECLINE_LABEL`). The type they share
is an `Offer { question, label, accept() -> SessionOp, stands(now) }`
with one drawer. Each of the three answers `stands` in its own way: the
name field, the generation plus the tool's picks, and the session not
being `busy()`.
