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

**Where it stands (2026-11-03, hand-back).** Steps 1-3 are done (#6388,
#6393, #6401, #6403). Step 4's agreed half landed as #6414: blend size,
mirror label, clearance bounds as slots, and the count row. The rest of
step 4 is parked on `[ev]` PR 6406 (fork row 247):
- When Ev answers, the next orchestrator does three things:
  - fills row 247's Ev, match, token and mapping columns (bytes 24 and
    227: C = Fable, A and B = Opus; tokens A 154081/177453,
    B 126435/147961, C 205783/230679);
  - makes 6406's README text match the answers;
  - takes `[ev]` off the title and merges it.
- The next units are then:
  - the split unit: face pick, offset row, `SlotId::PoseScalar`,
    discovery;
  - the pattern-fields unit, after PLACING's build of #6369, with
    `the-pattern-gesture-names-its-index` riding along.
- **Never merge an `[ev]` branch into forms/orchestrator.** Units are
  cut from that branch, so they would carry the unratified text (#6414
  did, and its review caught it).

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
