# SMELL-SCAN Track T — orchestrator log

**Constituted 2026-08-31, by the S-BLEND orchestrator.** Track T is
`crates/sweep/` (both `src/` and, by exception to W's fence, the
`sweep/tests/` files its own rows name), claimed whole by S-BLEND at
VERBS-SHELLFIX 2b's merge per the ratified partition
(`docs/WORK-STREAMS-2026-08.md`; row schedule in §D of
`docs/SMELL-SCAN-2026-08.md`, Track T table). This file is the
execution record — rulings, lane state, review outcomes, incidents.
Live status is here and in §D, never in `memories/`.

**Branch prefix:** `smellt/` for units; the orchestrator sits on
S-BLEND's own session branches, and cross-references to program
state live in `docs/S-BLEND-LOG.md`.

**This track runs OUTSIDE the model A/B experiment**, following the
F/G/I precedent the S-BLEND plan's "SMELL conventions" phrase names:
no pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md`; nothing on
this track reads or edits that file. Stated honestly: the I-log's
recorded REASON for the exclusion (the experiment pause of
2026-08-21) has lapsed — the experiment is live again — so this is a
precedent-following ruling, not a forced one, and Evan can reverse
it for later lanes if style work should be instrumented.

## Review policy (the F/G/I shape)

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`
  dispatched BY PATH, with the per-lane emphasis a dispatch owes
  (`docs/REVIEW-STYLE-DISPATCH.md`), plus the two standing track
  questions: (1) is the row's original problem COMPLETELY gone — not
  narrowed, not relocated; (2) was it closed the best available way,
  or merely a way that compiles.
- **Adversarial review only where the change carries meaningful
  risk** (Evan's C-R12 criterion: complex enough that there is a
  significant chance of a regression CI will not catch).

## Rulings

| # | Question | Ruling | Who, when |
|---|---|---|---|
| **T-R1** | Serialization vs the S-BLEND implementation slate: BLEND-6 (and later BLEND-3/-4) edit `crates/sweep/src/fillet/`, and BLEND-6's ratified V3 renames the whole module path. | **`fillet/`-touching rows are KEEP-OUT while a BLEND implementation lane is live**: D90 (`fillet/build.rs` + `fillet/surgery.rs`) and D321 (`fillet/admit.rs`) wait, and D321 additionally waits for the V3 rename so its test-utils conversion lands against the final path. Non-fillet rows run in parallel with the BLEND lanes — the partition's own premise (different files). | orchestrator, 2026-08-31 |
| **T-R2** | D91 spans this track and Track W (`LoftError::SeamStructure`'s shape change reaches `editor-core/tests/lib_doors_node_result.rs`). BLEND-6 is simultaneously reworking the kernel-door refusal surface and will plausibly touch the same door-test file. | **D91 DEFERRED until BLEND-6 merges** — the collision risk is in exactly the file the fence exception names. Not staffed into T-a. | orchestrator, 2026-08-31 |
| **T-R3** | C-e/H13 carries §D's own instruction: "Verify against #779 before staffing." | Verification dispatched (read-only) 2026-08-31, before T-a's brief was cut; T-a takes the row ONLY if the verdict is OPEN, and otherwise records the verified-closed evidence here and in §D. | orchestrator, 2026-08-31 |
| **T-R4** | D320 | Filed-not-takeable ahead of D240, per the row itself. Nothing to decide; recorded so the track's ledger is complete. | orchestrator, 2026-08-31 |

## Lane state

| lane | rows | state |
|---|---|---|
| **T-a** | C20 (turning-path orientation pins), D104 (the two hand-run diff artefacts), C-e/H13 (conditional on T-R3's verdict) | dispatching 2026-08-31 |
| (unstaffed) | D124 (re-home the struck-lane findings), C25 (the six-times-built swept body — cross-crate homing, fence note owed at dispatch), D96 (ten `unreachable!` arms — file-set to be enumerated before staffing to check the fillet overlap) | queued |
| (kept out per T-R1/T-R2) | D90, D321, D91 | wait on BLEND-6 (D321 also on V3) |
| (not takeable) | D320 | waits on D240 |
