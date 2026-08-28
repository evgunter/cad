# GUI v1 exit walk (2026-08-28, proposed — awaiting Evan's sign-off)

The plan is `docs/GUI-PLAN.md` (RATIFIED 2026-08-27); this walk
checks the program against it, item by item, with the evidence
named. Units GUI-0…GUI-4 merged as PRs #1094, #1093, #1101,
#1106, #1113 (A/B samples #29–#33, ordinals 400–404), all through
protocol-v6 cross-model duals with implementer-inherited fix
passes. The whole program ran 2026-08-27 16:00Z → 2026-08-28
01:55Z.

## The G3 four, plus the ruled addition

- **Click-to-select** — GUI-2: single-select `Selection::Face`
  (stable refs only), ray path through `NodePick`, GPU id pass
  comparative, one selection value shared with the tree. Evidence:
  186→227 headless rows; the id pass executed on lavapipe.
- **Pan / rotate / zoom** — GUI-0: typed renderer-free
  `Camera`/`CameraOp`/fold; fail-loud framing after review.
- **Free-move of completely-unconstrained parts** — GUI-4:
  document-derived eligibility, typed refusals (incl. the
  fused-geometry arm), the G3 visual-distinctness treatment as an
  asserted value (violet probe), superseded-and-discarded on mate
  commit. Works through pattern ancestry on the real flat-pack
  (hosted probe).
- **Hiding parts** — GUI-4: scene and pick drop it, tree and
  document keep it, never persisted; hide-everything draws an
  honest empty scene.
- **The mate tool (ruled in)** — GUI-4: two sequential picks in
  tool state, kernel-read admissions (`ContactClass::ALL`),
  exactly one committed `DocEdit`, typed pick-vanish degradation;
  the placement pullback pinned by non-circular oracle rows (the
  identity mutant dies at four rows). Tangent commits carry their
  verdict (tree note + the once-per-landing A5 at-rest badge) —
  adjudicated with both reviewers: a viewer refusal would be
  stricter than the API.

## The implied substrate

Viewport over evaluated documents at display-δ (GUI-0); feature
tree over the GQ2 DAG with typed badges (GUI-3); property panel
through pure `apply` with busy + `CancelToken` + `Reevaluate` and
the expression-refusal affordance (GUI-3; canceled runs never
land); linear undo chrome over tree-shaped state, sibling-minting,
save writes the current path (GUI-3); open/save + dialog + the
demo-document gallery via the tour exporter (GUI-3/4; scene list
corrected to {assembly, checks, ring, diefillet, heatsink}).

## Acceptance (the plan's ruled shape)

**Hosted, on the real gallery**: the render lane runs
`demo-tour gallery` and an exit-nonzero probe — 10/10 documents
open through the typed doors and resolve; hide/probe work where
geometry allows and refuse typed where it does not; no
accepted-but-inert operation (the class GUI-4's R1 found is gated
against). **The full interactive sequence**: the ten-stage
headless exit walk (`assembly_walk.rs`), on a gallery-shaped
fixture whose in-file argument states why no single real document
can carry the whole sequence (the stand is fully constrained; the
flat-pack has nothing to mate). **Human-side**: screenshots
2026-08-27/28 (Xvfb + lavapipe — chrome, panels, selection,
free-move probe, hide, mate tool with admission verdict); the
real-hardware first light remains #1097's checklist.

## The plan's named risks, closed

Immediate-mode seam: measured GO at GUI-0, re-taken
authoritatively at GUI-3 — no §5 fallback condition met; the iced
fallback question is closed for v1. wgpu plumbing: running (id
pass included) on a software adapter. egui churn: 0.36 pinned,
MSRV under the toolchain, watch item stands.

## Residue (owned, none blocking)

#1097 hardware first light (checklist extended by GUI-2/4);
#1111 Display-gap class; #1117 save-a-copy identity; #1120
SetPlacement vocabulary gap (banked from GUI-4's R1); the
viewer-CI seed-gate's SKIP direction first exercised hosted on the
next kernel-only PR; GUI-6 (history graph + sidecar) banked
post-v1 per the plan.

## A/B accounting (details in MODEL-AB-LOG; numbers live there)

Five v6 duals, every finding adjudicated, both suites promoted at
every fix pass. Tally: 0 confirmed + 1 candidate (GUI-4 R1's M1,
unilateral-by-execution) pending the blinded coding, which also
weighs the disclosed blinding contamination on the GUI-2 and
GUI-4 pairs (the orchestrator's log leaks — rule + template fix
recorded — and the block-record format defect resolved by Evan's
no-precommitment-required ruling; block draws now live branch-side
until block conclusion).

## The two decisions this walk asks of Evan

1. **Ratify this walk** (the program's required scope is
   delivered), or name what is missing.
2. **GUI-5 (the threaded web lane)**: dispatch, defer, or drop —
   the plan makes it separable stretch ("skipping it costs v1
   nothing"); block GUI-B2 slot 2 stands ready if wanted.
