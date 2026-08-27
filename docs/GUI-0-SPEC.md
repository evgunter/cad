# GUI-0 spec — the scaffold spike (`viewer` crate)

Unit 0 of `docs/GUI-PLAN.md` (RATIFIED 2026-08-27); the increment
`docs/GQ6-RESURVEY.md` §5 names. Read the plan's Rulings and
Platform sections and `docs/GUI-DESIGN.md` G1/G3 before starting —
this spec cites, it does not restate. Standing lane obligations:
`docs/prompts/implementer-discipline.md`.

## Deliverable

The **`viewer` crate** — the name DESIGN.md's crate table reserves
(the "Not yet a crate" row) — as a workspace member:

1. **eframe app** (egui, ratified GQ6 toolkit; toolchain 1.97.0
   covers egui 0.36's MSRV 1.95) with docked chrome: at minimum a
   viewport pane plus one side panel (a placeholder for GUI-3's
   feature tree is fine — this unit ships chrome, not content).
   **OQ-b is decided here**: pick `egui_tiles` or `egui_dock` and
   record the rationale in the PR body (both were live and
   MIT-compatible as of the re-survey; verify the ≥2-week
   dependency-age rule, `memories/review-and-dependency-policy.md`,
   against whatever versions you pin).
2. **Thin custom wgpu viewport** (egui-wgpu paint callbacks — the
   rerun shape) drawing **one tessellated body at display-δ**: a
   real body obtained through the public API (kernel `build` or an
   evaluated `editor-core` document — never a hand-built mesh;
   `memories/demo-purpose.md` applies to the spike's scene too).
   Chordal display tolerance is the fidelity lever; ε is never
   touched (GUI-DESIGN ratified micro-decision). Depth buffer,
   sane shading (flat or single-light Lambert is plenty), outward
   winding per `mesh::FacePatch`'s documented contract.
3. **Orbit / pan / zoom as typed layer-3 operations** (the G1
   operations-are-API rule, recorded 2026-08-27): a camera state
   value plus typed operations on it — callable with no renderer
   present. The egui input mapping folds pointer/scroll events into
   those operations; rendering is a pure view of the state they
   produce. Nothing is expressible only as a widget interaction.

## The measurement (why this unit is a spike)

This unit takes the one reading no survey could: **the friction of
holding an authoritative document/state value under an
immediate-mode loop**. GQ6-RESURVEY §5's egui→iced fallback
conditions get their first data here. The PR body must report,
explicitly against §5's three conditions: did keeping the
authoritative value coherent under egui require ad-hoc
frame-to-frame widget state, and where? A "no friction" reading is
a finding too — state it rather than leaving it implied.

## Constraints (carried by every GUI unit, from the plan)

- **The interaction layer never assumes threads.** The spike has no
  evaluation service yet; do not introduce one. Anything long-running
  it does grow must sit behind a seam per the plan's platform
  section.
- **No dependency that breaks the existing wasm guard** (the
  `--features interval` wasm32 check step, GQ6-RESURVEY §4). If
  including `viewer` in that `--workspace` check would break or
  meaningfully slow it, exclude `viewer` from the guard step with
  the reason stated in the PR (GUI-5 owns the real wasm lane);
  either way the guard must be green on your PR, and say which way
  you went.
- **Licensing**: all new dependencies must be MIT/Apache-2.0
  compatible (egui's ecosystem is; verify what you actually pin).
- **CI cost is a reported number, not a surprise**: eframe/wgpu are
  the heaviest dependency subtree this workspace has taken. Report
  the code-tier CI wall-clock delta in the PR body
  (`docs/CI-MINUTES-2026-08.md` is the context).

## Testing (G1's testability rule)

Layer 3 is headless-testable; only pixel-painting escapes. CI-side
this unit ships at minimum:

- camera-operation tests: replay synthetic operation sequences,
  assert on resulting camera state (orbit composition, zoom bounds,
  pan in view plane — whatever invariants your camera claims);
- an input-mapping test: a synthetic event stream folds to the
  expected operation sequence;
- the crate builds and its tests run in ordinary hosted CI with no
  display (no windowing in unit tests; eframe app construction may
  stay untested where it genuinely needs a display — say so).

No screenshot/pixel gate in this unit. Do not stand up a software
rasterizer lane for it.

## Out of scope

Selection and picking (GUI-1/GUI-2), document panels and evaluation
service wiring (GUI-3), any assembly interaction (GUI-4), wasm
build lane (GUI-5), any `DocEdit` emission. The spike renders and
navigates; it edits nothing.

## Acceptance

- `viewer` runs natively (you will not have a display in the lane;
  verify construction as far as headless allows and say exactly
  what was and was not exercised — a maintainer-side run is part of
  the review, not this lane's claim),
- the tessellated body renders through the paint-callback pipeline,
- camera operations are typed, renderer-free, and CI-tested,
- the seam-friction reading and OQ-b rationale are in the PR body,
- hosted CI green (the gate; local runs are iteration tools).

Branch `gui/gui-0-scaffold`; merge `origin/main` immediately before
opening the PR and re-merge whenever main moves (CONFLICTING = a
silent CI outage). NO Co-Authored-By trailer in lane commits
(A/B blinding; `memories/model-ab-experiment.md`).
