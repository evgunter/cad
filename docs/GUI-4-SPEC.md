# GUI-4 spec — assembly interaction + the mate tool

Unit 4 of `docs/GUI-PLAN.md` (RATIFIED 2026-08-27) — the last
required v1 unit; the plan's exit demo (open the assembly, exercise
every G3 item plus the mate tool) rides on it. Dispatched after
GUI-2 lands (same-crate rule). Read the plan's Rulings (mate
authoring IS v1 scope; two sequential picks held in tool state),
Scope, and Acceptance; `docs/GUI-DESIGN.md` G1 + G3 (free-move and
hide are DISPLAY-layer state, never persisted; fit-probing must be
visually distinct from mated placement — an honesty requirement);
`docs/ASSEMBLY-DESIGN.md` and `docs/ASM-PLAN.md` for the mate
vocabulary; and the GUI-2/GUI-3 unit entries in `docs/GUI-LOG.md`.
Standing lane obligations:
`docs/prompts/implementer-discipline.md`.

## Deliverables

1. **A `PartResolver` for the viewer** (the gap GUI-3 disclosed:
   assembly gallery documents open with typed `InstantiatePart`
   refusals because no resolver is wired). Implement the shipped
   `editor_core::part::PartResolver` trait over the gallery
   directory (the assembly's part documents sit beside it), wired
   through the session's open path. Resolution failures stay
   typed and render as the tree badges GUI-3 built. Design the
   lookup so a document never silently resolves against the wrong
   directory (state the rule: e.g. resolve relative to the opened
   document's directory, refuse otherwise).
2. **Per-instance hide**: a typed layer-3 operation toggling an
   instance's visibility; hidden instances drop out of the drawn
   scene and the pick index but never out of the document or the
   tree. Never persisted (G3).
3. **Free-move for completely-unconstrained instances**: a typed
   layer-3 display transform (`placement::Frame` composes) on
   instances with NO mate participation — the
   completely-unconstrained test is derived from the document
   (which instances appear in no mate node), not guessed from
   solver state. No solver involvement; never persisted; one
   gesture = preview stream + one committed layer-3 transform (the
   G1 preview/commit shape applies to layer-3 state too — one
   undo step in the LAYER-3 sense only, since nothing enters the
   document; say in code which history, if any, holds it — the
   plan's undo note governs document state only).
   **Visually distinct from mated placement** (the G3 honesty
   requirement): pick a clear treatment (e.g. tint/ghosting) and
   assert its presence as a value in the scene-build rows (the
   pure-function-of-state discipline makes that testable
   headlessly).
4. **The mate tool** (ruled into v1): a modal layer-3 tool holding
   **two sequential face picks in tool state** (GUI-2's
   single-select vocabulary consumed twice, per the ruling that
   closed round-2 OQ-a) — `names::interrogate::face_frame`
   derives each pick's frame; then a **class/alignment choice
   from the shipped ASM vocabulary** (`editor_core::mate`:
   `MatePrimitive`/`Alignment`/`ClassAdmission` — expose what the
   two picked frames ADMIT, refuse typed what they do not); then
   **exactly one committed `DocEdit` adding the mate node**. The
   instance's free-move transform is superseded by the solved
   placement when the mate lands (the shipped placement solving;
   the free-move value is discarded or zeroed at that commit —
   state which and test it). Tool state survives a picked ref
   vanishing (GUI-2's survival semantics; the tool degrades to
   its previous step, typed, no crash).
5. **The exit-demo walk, headless**: one CI row (or example)
   driving the acceptance sequence end-to-end through the typed
   vocabulary on the gallery assembly: open → resolve → tree
   shows instances → hide one → free-move an unconstrained one
   (visibly distinct value asserted) → two picks → admitted
   class chosen → one committed mate edit → placement solved →
   free-move superseded → save/reopen round-trips the document
   (layer-3 state gone, per G3). This row is the program's
   acceptance evidence; treat its readability as part of the
   deliverable.

## Constraints

- G1 boundary rules bind throughout: tools are
  `handle(event, ui_state) → (ui_state′, edits, overlay)`; every
  operation typed and renderer-free; no arena key in layer 3
  (GUI-2's selection value and the `pncad::select` door are the
  vocabulary); transient tool state (the held picks, in-flight
  free-move) never enters the document.
- Mate math, placement solving, and admission logic are SHIPPED
  ASM substrate — this unit consumes them. A gap found there is a
  finding to report (and possibly a small argued extension), never
  a re-implementation in layer 3.
- The seam-friction inventory discipline binds new state
  (hide/free-move/tool state each get one home; no per-widget
  shadows).
- Your changes seed `viewer` (+ possibly `pncad`): the every-lane
  toolkit verdict + clippy-app steps must show RUN at step level.
- #1102 (main red at the 1e-12 draw) stands until fixed — cite,
  don't investigate. No new dependencies expected.
- Do not modify `crates/bvh`; `editor-core` extensions follow the
  GUI-2 precedent (small, argued, closing a lane — not features).

## Testing

Headless CI: resolver rows (assembly evaluates; a missing part
refuses typed; the directory rule enforced); hide rows (scene and
pick index drop the instance, tree keeps it, document untouched);
free-move rows (only completely-unconstrained instances accept,
typed refusal otherwise; preview/commit shape; the distinctness
value; supersession on mate commit); mate-tool replay rows (two
picks → admission set → refusals for inadmissible classes → one
committed edit; pick-vanish degradation); the exit-demo walk row.

## Out of scope

Solver work beyond the shipped placement machinery (M10);
drag-to-move mated geometry (G3 excludes live constrained
editing); multi-select; mate editing/deletion UX beyond what the
tree + property panel already give; the history-graph UI (GUI-6).

## Acceptance

The plan's exit demo, executed: the app opens the gallery
assembly through the dialog and every G3 item plus the mate tool
works through the typed vocabulary, with the headless walk row as
CI evidence and the hardware half riding #1097's checklist.
Hosted CI green.

Branch `gui/gui-4-assembly`; merge `origin/main` immediately
before opening the PR and re-merge whenever main moves. NO
Co-Authored-By trailer in lane commits (A/B blinding;
`memories/model-ab-experiment.md`).
