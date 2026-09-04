# DOCM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/docm/plan.md`. A/B band 1800–1899
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose DOCM section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `capend-top-bottom-contradicted-by-negative-extrude` from `work/issues/`
- `fused-step-slot-aliases-arrival-spec` from `work/issues/`
- `sketch-frame-from-face` from `work/issues/`
- `add-profile-mints-no-frame` from `work/issues/`
- `add-profile-placement-on-picked-face-frame` from `work/issues/`
- `split-side-and-pattern-instance-as-operand` from `work/issues/`
- `no-docedit-splices-a-deleted-node` from `work/issues/`
- `document-seam-no-in-session-change-detection` from `work/issues/`
- `layer3-recipenodeid-aliases-across-rewinds` from `work/issues/`
- `no-persistent-setplacement-session-op` from `work/issues/`
- `revolve-pole-export-interior-on-axis-vertex` from `work/issues/`
- `check-registry-gathers-product-twice` from `work/issues/`
- `save-a-copy-duplicate-id-bricks-store` from `work/lib/`
- `memo-admission-and-resolver-state` from `work/lib/`
- `instantiation-seam-drops-mate-identity` from `work/mate/`
- `no-door-mints-mate-frame-from-face` from `work/mate/`
- `certify-locally-valid-range-instead-of-sampling` from `work/m10/`
- `C6` from `work/code-quality/`
- `D365` from `work/code-quality/`
- `D366` from `work/code-quality/`
- `debug-in-prose-residue-after-finding-sink` from `work/code-quality/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestrator opened; scoping before the first `[ev]` PR (2026-09-04)

Orchestrator session opened on Ev's direction ("get ready to be the
orchestrator for docm ... scope out the design work and start a
discussion"). No unit cut, no ordinal claimed. Four read-only code
surveys were run against the eleven questions; what they changed:

- **Question 1 is already ruled.** The Band-4 roadmap line in
  `docs/DESIGN.md` (Ev, 2026-09-01) and BOOL-13 (#1553) removed the
  schema version: an additive vocabulary change invalidates nothing and
  a breaking one is a corpus regeneration. `capend-top-bottom-…`
  (rename `CapEnd` to the sweep vector's own ends) and
  `fused-step-slot-…` (add `SweepVal2`/`ArcLenVal2`/`Bulge2`) are E
  with no ruling owed; so is `C6`'s `WireStep` member. Proposed to Ev
  in chat: collapse the question out of the plan.
- **Questions 2, 3 and 4 share one axis** — which reference shapes the
  recipe admits (DAG edge by `RecipeNodeId`; frozen `StableName`
  resolved live under N5; `Expr` literal). A derived frame is a datum
  carrying a name (the `MeasureRef { at, name }` shape); a part
  operand is a node or an operand carrying `SplitHalf`/`Instance(i)`;
  splice is an edit rewriting a DAG edge whose consumers hold frozen
  names. Proposed as one conversation, three rulings.
- **Questions 5 and 6 share the other axis** — a held value with no
  witness of the world it came from: `Evaluation` carries no document
  or resolver identity; `next_id` is part of the `Doc` value, so undo
  restores it and a re-insert re-mints an id (`history.rs` retains
  values, never replays).
- **Premise corrections for the items** (recorded here, item files
  untouched until the ruling lands): `memo-admission-…` says the
  session hands the previous evaluation as `prior`; today the memo
  lives in `viewer::evalseam::PriorRun`, gated by resolver identity
  (`same_resolver`), and `request_eval` carries no prior — only
  `probe_bounds` hands one. `check-registry-…`'s three gather sites are
  now `session.rs` `land` at ~2020/2024/2032. `certify-…`'s first
  missing door partly exists: `EvalOptions::param_box` widens document
  parameters; what is missing is the slot-widening override.
- **Fences that gate the order.** `MATE-EXIT` is still `needs_ev`
  (PR 1528 merged the walk as PROPOSED; no ratification comment), so
  `mate.rs`/`assembly.rs` stay closed to this program: questions 9 and
  10 wait. Question 11 waits on M10-7 (PR 1725).

Next: Ev's pushback on the recommendations in chat, then the first
`[ev]` PR (the reference-vocabulary conversation) and the E-class
dispatches that need no ruling.
