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
