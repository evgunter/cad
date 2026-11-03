# FORMS — the plan

one editor per node kind: creating a thing commits it and hands it to
the edit surface

Opened 2026-09-22 by CHROME's priority-seam cut. Re-chartered
2026-11-02, when Ev's create/edit reports (2026-10-31, 2026-11-01)
took the track over budget. The vocabulary and seat-gate rows went to
FORMWORDS (`work/formwords/`).

## The rule (Ev, chat, 2026-11-01)

Every node kind that has both a create form and an edit surface has
two implementations, and they become one. The choice is made once for
the whole viewer, and Ev prefers (a): **a create gesture commits the
node, and the node's edit surface takes it from there.** Every change
made while the thing is being created is then an ordinary edit in the
history, and undo reaches it. The full statement is in
`creating-a-thing-and-editing-it-are-two-implementations`.

## The slate

| pri | item | cost | role |
|---|---|---|---|
| P1 | `creating-a-thing-and-editing-it-are-two-implementations` | H | the rule, and the census of pairs it is cut into |
| P1 | `the-profile-create-form-is-a-second-profile-editor` | H | the instance Ev hit first: a new-profile button opens the one editor |
| P1 | `the-add-profile-form-takes-numbers-and-no-formula` | E | rides with the profile unit, which subsumes it |
| P1 | `the-extrude-and-revolve-forms-take-no-variables` | M | the extrude and revolve pair |

## Order

1. **Designers first, on the rule's open points**: what a create
   gesture must collect before a node can exist (picks), and what is
   left to the edit surface (values); how a node committed with
   defaults reads when it cannot evaluate yet; whether an empty
   profile is a document state; where the circle and rectangle
   templates go; and the census of pairs. Ev has chosen (a), so this
   elaborates a ruling rather than reopening it. Only a sub-question
   with several viable answers that Ev's words do not settle goes back
   to Ev.
2. **The profile unit**: the new-profile button, deleting the add-profile
   form, with `the-add-profile-form-takes-numbers-and-no-formula`
   riding along.
3. **The extrude and revolve unit.**
4. **The seated tools**, cut per tool from the census once the profile
   unit has set the pattern.

**Not this track's to build.** The placement editor's Create door
(PLACING, `crates/viewer/src/matetool.rs`) and the transform and
duplicate tools, which PLACING is retiring into it. The add-variable
widget is CHROME's
(`a-variable-is-edited-in-a-separate-widget-not-on-its-own-row`).
The rule binds them, and the census names them, but they are built on
their owners' slates.

## Review posture

No duals and no `docs/MODEL-AB-LOG.md` row (Ev, in chat, 2026-09-04,
reaffirmed 2026-09-22: *"no AB protocol"*). Each unit gets a full
review: a style review against `docs/prompts/reviewer-style-lane.md`,
plus claims to falsify. Deleting a form while undo has to reach the
edits made during creation carries a real chance of correctness bugs.
