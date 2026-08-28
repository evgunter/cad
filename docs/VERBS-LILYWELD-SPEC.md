# VERBS-LILYWELD — the lily's flower weld, circle-coincident (two PRs)

Evan's content call on #1059 item 2 (ratified in chat 2026-08-27):
re-author the flower/arch junction circle-coincident, then flip
wall 2 through the #968-shaped kernel half. Branches
`verbs/lilyweld-1`, `verbs/lilyweld-2`. Difficulty pre-logged:
PR-1 **S** (content + pins), PR-2 **M** (gate + rung). #1059 is
the derivation record and binds; #968 is the shape template for
PR-2 (its checklist items 1–2 generalized; its item 3 — the
torus×torus tangency disposition — is NOT needed for this pair
and stays banked with wall 1).

## PR-1 — the re-authoring (content, demo layer)

1. Re-author the lantern/arch junction per #1059's geometry: the
   lantern axis is the stem's own tangent (already pinned by
   `lantern_axes_are_the_stored_stem_tangents`); the cone is cut
   at the tube's minor radius so its rim IS the torus meridian
   circle at that station — analytically identical carriers, a
   shared circle, not a transverse SSI curve. The current 0.08
   setback (pedicel tip inside) retires; whatever visual intent
   the setback served is re-achieved inside the coincident
   authoring or its loss is recorded as a content note.
2. Wall 2's text and pin update to the NEW honest refusal: with
   coincidence authored, the union still refuses at the operand
   gate (declared cone×torus has no admission until PR-2) — the
   wall pins THAT door/payload, cites this spec and #1059, and
   its retire note names PR-2. The old "germ-chord lane" schedule
   sentence retires (the third-instance lesson: the payload and
   raising site are the evidence).
3. Pins: the shared-circle coincidence asserted analytically in
   the scene (both carriers' station circles equal to closed
   form — the coincidence is the CONTENT, so it gets its own
   assert); census/mass-properties re-pins for the re-authored
   bodies; renders re-cut; baseline rows moved per the runbook
   with the moves argued (this is a re-authoring — rows may
   legitimately CHANGE, each stated).
4. Fences: no kernel changes; no other scene moves; wall 1
   (#968, torus×torus) untouched.

## PR-2 — MEASUREMENT, and the unit's close

**This section is CORRECTED (2026-08-27). What it originally
specified — operand-gate admission for declared cone×torus plus a
cone/torus rung in `carrier_eq` — was refuted by its own opening
measurement before any of it was built.** The correction is recorded
here rather than quietly rewritten, because this is the FOURTH
instance of the *stated blocker is not the binding one* class
(#1031's wall 7, #1059's wall 2, the M9 exit walk's wall 8) and the
first caught in a SPEC's own text rather than in a probe comment.

### The measured door sequence for the declared lily weld

Run at head, then scratch-widening one door at a time (every scratch
patch reverted; no kernel change shipped):

| # | door | payload | disposition |
|---|---|---|---|
| 1 | `gate_operand_pairs` (`boolean/reduce.rs:341`) | `CurvedPairUnsupported { op: None, operand: A, face: 3v1, kind: Cone, other_face: 3v1, other_kind: Torus }` | today's refusal; the original item 1's target |
| 2 | `gate_maximal_faces` | `NonMaximalFaces { operand: A, edge: 1v1 }` | **#1031** — the binding blocker |
| 3 | reduction's curved-face arm | `CurvedPierceUnsupported { A, face 3v1 (neck cone), edge 2v1 (its seam strut) }` | wall 12's door |
| 4 | `carrier_eq` cone/torus rung | **never reached** | the original item 2 |

The DECLARED and UNDECLARED unions return the byte-identical payload
at door 1: the gate reads kinds before any declaration is consulted.
`flush_declarations` does find the contact — two pairs, the lantern's
two throat-disk half-faces against the arch's single end cap, an exact
coincident planar Rest — and it is never looked at.

Door 2, named face by face: the lantern has TWO planar same-key
adjacencies, the LIP disk (faces 1v1/2v1) and the THROAT disk
(6v1/10v1), each a full revolve's axis-touching cap arriving as two
half-faces on one plane key. Its three CURVED same-key adjacencies
(two cones, one sphere zone) are the canonical maximal form, not
defects. `merge_coplanar_faces` still refuses
`MergedFaceRoleAmbiguous { face: 1v1 }` on the re-authored lantern.

### The three rulings (VERBS orchestrator, 2026-08-27)

1. **The `carrier_eq` rung is KILLED.** It has no consumer. PR-1's own
   achievement is what dissolves it: authoring the junction so the two
   solids ABUT on a full disk made the weld's declared contact
   plane×plane, which the existing plane arm already handles. The
   shared circle is not a Rest contact — it is the RIM of one. A cone
   and a torus are never the same carrier, so a Rest declaration on
   that pair would be `Contradicted`, correctly.
2. **The operand-gate admission is DEFERRED, not built.** It lacks an
   honest trigger: no declaration covers the cone×torus pair, so an
   admission rule would have no covering declaration to consult —
   either dead code, or a new design question (neighborhood-of-a-
   covered-contact admission) that does not belong in this unit. The
   question reopens with data once #1031 lands and the sequence is
   re-measured.
3. **Wall 2's blocker moves to #1031**, which is now triple-demanded
   (wall 2, lily wall 7, the teapot's F7 caps) and is this lane's next
   unit.

### What PR-2 actually ships

Two pins in `demos/tour/src/lily.rs`, no kernel change:

- `the_declared_weld_refuses_exactly_as_the_undeclared_one_does` —
  declaring the weld changes nothing today, and the two payloads are
  byte-identical. This is the row that will show the two calls
  SEPARATING the day the gate learns the declared pair.
- `the_lanterns_two_axis_touching_caps_are_the_f7_defect` — the
  (4 planar, 6 curved) same-key split and the still-shut merge door,
  so a change that started treating curved same-key pairs as defects
  fails here rather than silently.

Wall 2's own comment and retire note carry the corrected blocker.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR drafts. Targeted local runs;
verify hosted coverage at the STEP level (klint_row). Merge
origin/main before opening; confirm CI jobs actually RUNNING;
note the drawn point; watch to completion; cancel detached timers
before the final report; kill superseded detached jobs. Do not
merge.
