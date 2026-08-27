# GUI-2 spec — selection in the viewport

Unit 2 of `docs/GUI-PLAN.md` (RATIFIED 2026-08-27). Dispatched
after GUI-3 lands (sequencing call L-GUI-1 in `docs/GUI-LOG.md` —
both units edit the same `viewer` files); build on merged GUI-0
(viewport, camera), GUI-1 (`Bvh::ray`, `resolve::pick`, the
`NodePick` door), and GUI-3 (panels, session ops, evaluation
seam). Read the plan's Rulings (single-select), GUI-DESIGN G1 +
GQ7's recorded constraint (tools survive the referenced entity
vanishing), GQ6-RESURVEY §3 (the ID-buffer recommendation), and
issues #1097/#1098 before starting. Standing lane obligations:
`docs/prompts/implementer-discipline.md`.

## Deliverables

1. **The selection value**: typed layer-3 state holding at most
   one stable ref (single-select per the ruling) plus its source
   node — `StableName`-based, never an arena key (G1). Hover is
   transient layer-3 state, never persisted, never in the
   document. Selection ops (select, clear, hover) are typed
   operations callable with no renderer, folded from input events
   like the camera ops; the left mouse button (documented as
   reserved by GUI-0) binds to selection.
2. **The ray path**: click/hover → viewport ray (from the camera;
   the un-projection is a typed camera operation, testable
   headlessly) → GUI-1's service via **`NodePick`** — per #1098's
   directive, the viewer's per-scene cache holds `NodePick`s keyed
   by evaluation generation (a stale generation's picks are
   discarded, never re-paired by hand). Nearest hit's `StableName`
   becomes the selection.
3. **The GPU ID-buffer pass** (RESURVEY §3's hover/click
   exactness lane): an offscreen pass rendering per-patch ids,
   read back at the cursor. The id↔(node, body, face-patch)
   mapping is a pure function pair tested headlessly (round-trip,
   collision-freedom across bodies, stability under re-tessellation
   within a generation); the pass itself is `app`-feature GPU code
   excused from pixel tests — extend issue #1097's first-light
   checklist with the ID-buffer verification (click a known face
   on hardware, compare with the ray path's answer on the same
   cursor). Where both paths answer the same query, they must
   agree; the headless expression of that is a test driving the
   RAY path against the id-mapping's inverse on synthesized
   cursor→ray fixtures.
4. **Selection highlight**: the selected patch rendered visually
   distinct (implementation free — re-tint, second pass);
   hover feedback likewise. A pure function of (scene, selection)
   — no retained per-widget state (the §5 inventory discipline
   GUI-3 established binds here too).
5. **Survival semantics** (the ratified resolution-failure rule):
   after an edit/re-evaluation, the selection value re-resolves
   through the shipped resolution/diagnosis machinery; a vanished
   ref leaves a typed unresolved selection that renders distinctly
   and disables dependent affordances — no crash, no silent
   clear. Test rows: delete the selected feature via the panel;
   edit a parameter so the selected face is consumed; undo across
   the selection's birth.
6. **Panel integration**: viewport selection and the GUI-3 tree
   selection are ONE value — clicking a face selects the owning
   node in the tree (name→node is the shipped inversion); tree
   selection does not require a viewport pick. One home for the
   selection state in the session.

## Constraints

- G1 rules bind (typed ops, headless replay asserting on selection
  state transitions and emitted ops; only pixel-painting escapes).
- Single-select only — no multi-select scaffolding "for later"
  (GQ7 is deferred by design; do not pre-build it).
- Your changes seed `viewer` → the toolkit rows must draw RUN;
  confirm at the filter's verdict step.
- No new dependencies expected; any exception passes the age and
  licence rules with the reason in the PR.
- Do not modify `crates/bvh` or `editor-core/src/resolve/` except
  through need demonstrated in the PR body (they are shipped
  GUI-1 surface; a gap found there is a finding to report, a
  small extension is acceptable with the argument stated).

## Testing

Headless: selection-op fold rows (event stream → selection
transitions, single-select invariant); un-projection rows (cursor
+ camera → ray, checked against projected known points);
id-mapping round-trip/collision rows; ray-path selection e2e on
gallery documents through `NodePick` (click fixtures → expected
`StableName`); survival rows per deliverable 5; tree↔viewport
selection unity rows; generation-invalidation row (stale NodePick
cache discarded on re-evaluation).

## Out of scope

Multi-select and filters (GQ7); box/lasso select; edge/vertex
picking (GUI-1's scoped exclusion stands — face picks only for
v1 selection); in-viewport manipulation of any kind (G3 excludes
it); mate authoring picks (GUI-4 holds those in ITS tool state,
consuming this unit's single-select vocabulary sequentially).

## Acceptance

Open a gallery document; hover highlights under the cursor
(hardware verification via #1097's extended checklist); click
selects — tree row and viewport highlight agree; edit the document
so the selection vanishes and watch the typed unresolved state;
hosted CI green with the toolkit rows drawn RUN.

Branch `gui/gui-2-select`; merge `origin/main` immediately before
opening the PR and re-merge whenever main moves. NO Co-Authored-By
trailer in lane commits (A/B blinding;
`memories/model-ab-experiment.md`).
